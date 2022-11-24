use std::{
  collections::{BTreeSet, HashSet},
  ops::{BitOr, Sub},
};

use super::parser::{parse_exp, parse_rule};
use itertools::Itertools;
use language::ast::exp::Exp;
use language_derivation_rule::ast::rule::Rule;
use lazy_static::lazy_static;
use regex::Regex;
use yew::Reducible;

pub struct State {
  // source of truth
  pub rows: Vec<Row>,
  pub focused_idx: Option<usize>,

  // computed properties (memoized)
  pub deps_list: Vec<RowDependency>,
  pub rule_vaildity_list: Vec<bool>,
}

#[derive(Clone, PartialEq)]
pub struct Row {
  pub sentence: String,
  pub derivation: String,
}

pub enum Action {
  Add { after_num: usize },
  ChangeSentence { num: usize, sentence: String },
  ChangeDerivation { num: usize, derivation: String },
  Format,
  ChangeFocus { idx: Option<usize> },
}

impl State {
  pub fn init() -> Self {
    State {
      rows: vec![Row {
        sentence: "".to_owned(),
        derivation: "".to_owned(),
      }],
      focused_idx: None,
      deps_list: vec![RowDependency::new_incomplete()],
      rule_vaildity_list: vec![false],
    }
  }

  pub fn init_from(rows: Vec<Row>) -> Self {
    let mut state = State {
      rows,
      focused_idx: None,
      deps_list: vec![],
      rule_vaildity_list: vec![],
    };
    state.reload_computed_properties();
    state
  }
}

impl Reducible for State {
  type Action = Action;

  fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
    // NOTE: state에서 computed properties 갱신이 필요한지 확인해야 함
    match action {
      Action::Add { after_num } => {
        let mut rows = self.rows.clone();

        lazy_static! {
          static ref NUM: Regex = Regex::new(r"\d+").unwrap();
        }
        for row in rows.iter_mut() {
          let mut derivation = row.derivation.clone();

          let targets = NUM
            .find_iter(&derivation)
            .filter_map(|mat| match mat.as_str().parse::<usize>() {
              Ok(num) => Some((mat.range(), num)),
              Err(_) => None,
            })
            .filter(|(_, num)| num > &after_num)
            .collect_vec();
          for (range, num) in targets.into_iter().rev() {
            derivation.replace_range(range, &(num + 1).to_string()[..]);
          }
          row.derivation = derivation;
        }

        rows.insert(
          after_num,
          Row {
            sentence: "".to_owned(),
            derivation: "".to_owned(),
          },
        );

        let mut next = State {
          rows,
          focused_idx: self.focused_idx,
          deps_list: vec![],
          rule_vaildity_list: vec![],
        };
        next.deps_list = next.get_deps_for_rows();
        next.rule_vaildity_list = next.get_rules_validity();
        next.into()
      }

      Action::ChangeSentence { num, sentence } => {
        let mut rows = self.rows.clone();
        if let Some(row) = rows.get_mut(num - 1) {
          row.sentence = sentence;
        }

        let mut next = State {
          rows,
          focused_idx: self.focused_idx,
          deps_list: self.deps_list.clone(),
          rule_vaildity_list: vec![],
        };
        next.rule_vaildity_list = next.get_rules_validity();
        next.into()
      }

      Action::ChangeDerivation { num, derivation } => {
        let mut rows = self.rows.clone();
        if let Some(row) = rows.get_mut(num - 1) {
          row.derivation = derivation;
        }

        let mut next = State {
          rows,
          focused_idx: self.focused_idx,
          deps_list: vec![],
          rule_vaildity_list: vec![],
        };
        next.deps_list = next.get_deps_for_rows();
        next.rule_vaildity_list = next.get_rules_validity();
        next.into()
      }

      Action::Format => {
        let rows = self
          .rows
          .iter()
          .map(|row| {
            let mut row = row.clone();
            if let Ok(exp) = parse_exp(&row.sentence) {
              row.sentence = exp.to_string();
            }
            if let Ok(rule) = parse_rule(&row.derivation) {
              row.derivation = rule.to_string();
            }
            row
          })
          .collect();
        State {
          rows,
          focused_idx: self.focused_idx,
          deps_list: self.deps_list.clone(),
          rule_vaildity_list: self.rule_vaildity_list.clone(),
        }
        .into()
      }

      Action::ChangeFocus { idx } => State {
        rows: self.rows.clone(),
        focused_idx: idx,
        deps_list: self.deps_list.clone(),
        rule_vaildity_list: self.rule_vaildity_list.clone(),
      }
      .into(),
    }
  }
}

impl State {
  pub fn reload_computed_properties(&mut self) {
    self.deps_list = self.get_deps_for_rows();
    self.rule_vaildity_list = self.get_rules_validity();
  }

  pub fn get_deps_for_rows(&self) -> Vec<RowDependency> {
    fn iton(idx: usize) -> usize {
      idx + 1
    }
    fn ntoi(num: usize) -> usize {
      num - 1
    }
    self.rows.iter().enumerate().fold(vec![], |mut acc, (row_idx, row)| {
      let row_num = iton(row_idx);
      // 필요하다면 parse_rule에 lru 캐시를 적용할 수 있을 것
      let dep = match parse_rule(&row.derivation) {
        Ok(rule) => match &rule {
          Rule::Premise => RowDependency::init_from([row_num]),
          Rule::AndIntro(k, l) => RowDependency::new() | acc.get(ntoi(*k)) | acc.get(ntoi(*l)),
          Rule::AndExclude(k) => RowDependency::new() | acc.get(ntoi(*k)),
          Rule::OrIntro(k, l) => RowDependency::new() | acc.get(ntoi(*k)) | l.and_then(|l| acc.get(l + 1)),
          Rule::OrExclude(k, (l0, l1), (m0, m1)) => {
            (RowDependency::new() | acc.get(ntoi(*k)) | acc.get(ntoi(*l1)) | acc.get(ntoi(*m1))) - (*l0) - (*m0)
          }
          Rule::IfIntro((k0, k1)) => (RowDependency::new() | acc.get(ntoi(*k1))) - *k0,
          Rule::IfExclude(k, l) => RowDependency::new() | acc.get(ntoi(*k)) | acc.get(ntoi(*l)),
          Rule::Falsum(k) => RowDependency::new() | acc.get(ntoi(*k)),
          Rule::NegIntro((k0, k1)) | Rule::NegExclude((k0, k1)) => (RowDependency::new() | acc.get(ntoi(*k1))) - *k0,
          Rule::IffIntro(k, l) => RowDependency::new() | acc.get(ntoi(*k)) | acc.get(ntoi(*l)),
          Rule::IffExclude(k) => RowDependency::new() | acc.get(ntoi(*k)),
          Rule::UnivQuntIntro(k) | Rule::UnivQuntExclude(k) => RowDependency::new() | acc.get(ntoi(*k)),
          Rule::ExisQuntIntro(k) => RowDependency::new() | acc.get(ntoi(*k)),
          Rule::ExisQuntExclude(k, (l0, l1)) => (RowDependency::new() | acc.get(ntoi(*k)) | acc.get(ntoi(*l1))) - *l0,
        },
        Err(_) => RowDependency::new_incomplete(),
      };
      acc.push(dep);
      acc
    })
  }

  /// NOTE: `self.deps_list`에 의존합니다. `get_deps_for_rows`를 부르도록 하는 게 더 나을지도
  /// 모릅니다.
  fn get_rules_validity(&self) -> Vec<bool> {
    fn ntoi(num: usize) -> usize {
      num - 1
    }
    fn one_or_none(set: &BTreeSet<String>) -> Option<String> {
      if set.len() == 1 {
        set.iter().next().map(|e| e.clone())
      } else {
        None
      }
    }
    fn unordered_tuple_eq((a1, a2): (&Exp, &Exp), (b1, b2): (&Exp, &Exp)) -> bool {
      ((a1 == b1) && (a2 == b2)) || ((a1 == b2) && (a2 == b1))
    }

    let items = self
      .rows
      .iter()
      .map(|row| match parse_exp(&row.sentence) {
        Ok(exp) => (Ok(exp), &row.derivation),
        Err(_) => (Err(()), &row.derivation),
      })
      .map(|(exp, derivation)| {
        (
          exp,
          match parse_rule(derivation) {
            Ok(rule) => (Ok(rule)),
            Err(_) => (Err(())),
          },
        )
      })
      .collect_vec();

    items
      .iter()
      .map(|(exp_row, rule_row)| {
        let (exp_row, rule_row) = match (exp_row, rule_row) {
          (Ok(exp_row), Ok(rule_row)) => (exp_row, rule_row),
          _ => return false,
        };

        match *rule_row {
          Rule::Premise => true,

          Rule::AndIntro(k, l) => {
            let (exp_k, exp_l) = match (items.get(ntoi(k)), items.get(ntoi(l))) {
              (Some((Ok(exp_k), _)), Some((Ok(exp_l), _))) => (exp_k, exp_l),
              _ => return false,
            };
            match exp_row {
              Exp::And { lhs, rhs } => (**lhs == *exp_k) && (**rhs == *exp_l),
              _ => false,
            }
          }

          Rule::AndExclude(k) => {
            let exp_k = match items.get(ntoi(k)) {
              Some((Ok(exp_k), _)) => exp_k,
              _ => return false,
            };
            match exp_k {
              Exp::And {
                lhs: exp_k_lhs,
                rhs: exp_k_rhs,
              } => (*exp_row == **exp_k_lhs) || (*exp_row == **exp_k_rhs),
              _ => false,
            }
          }

          Rule::OrIntro(k, None) => {
            let exp_k = match items.get(ntoi(k)) {
              Some((Ok(exp_k), _)) => exp_k,
              _ => return false,
            };
            match exp_row {
              Exp::Or { lhs, rhs } => (*exp_k == **lhs) || (*exp_k == **rhs),
              _ => false,
            }
          }

          Rule::OrIntro(k, Some(l)) => {
            let (exp_k, exp_l) = match (items.get(ntoi(k)), items.get(ntoi(l))) {
              (Some((Ok(exp_k), _)), Some((Ok(exp_l), _))) => (exp_k, exp_l),
              _ => return false,
            };
            match exp_row {
              Exp::Or { lhs, rhs } => unordered_tuple_eq((exp_k, exp_l), (lhs, rhs)),
              _ => false,
            }
          }

          Rule::OrExclude(k, (l0, l1), (m0, m1)) => {
            let (exp_k, exp_l0, rule_l0, exp_l1, exp_m0, rule_m0, exp_m1) = match (
              items.get(ntoi(k)),
              items.get(ntoi(l0)),
              items.get(ntoi(l1)),
              items.get(ntoi(m0)),
              items.get(ntoi(m1)),
            ) {
              (
                Some((Ok(exp_k), _)),
                Some((Ok(exp_l0), Ok(rule_l0))),
                Some((Ok(exp_l1), _)),
                Some((Ok(exp_m0), Ok(rule_m0))),
                Some((Ok(exp_m1), _)),
              ) => (exp_k, exp_l0, rule_l0, exp_l1, exp_m0, rule_m0, exp_m1),
              _ => return false,
            };
            match (exp_k, rule_l0, rule_m0) {
              (
                Exp::Or {
                  lhs: exp_k_lhs,
                  rhs: exp_k_rhs,
                },
                Rule::Premise,
                Rule::Premise,
              ) => {
                unordered_tuple_eq((exp_k_lhs, exp_k_rhs), (exp_l0, exp_m0))
                  && (exp_row == exp_l1)
                  && (exp_row == exp_m1)
              }
              _ => false,
            }
          }

          Rule::IfIntro((Some(k0), k1)) => {
            let (exp_k0, rule_k0, exp_k1) = match (items.get(ntoi(k0)), items.get(ntoi(k1))) {
              (Some((Ok(exp_k0), Ok(rule_k0))), Some((Ok(exp_k1), _))) => (exp_k0, rule_k0, exp_k1),
              _ => return false,
            };
            match (exp_row, rule_k0) {
              (Exp::Cond { antecedent, consequent }, Rule::Premise) => {
                (*exp_k0 == **antecedent) && (*exp_k1 == **consequent)
              }
              _ => false,
            }
          }

          Rule::IfIntro((None, k)) => {
            let exp_k = match items.get(ntoi(k)) {
              Some((Ok(exp_k), _)) => exp_k,
              _ => return false,
            };
            match exp_row {
              Exp::Cond { consequent, .. } => *exp_k == **consequent,
              _ => false,
            }
          }

          Rule::IfExclude(k, l) => {
            let (exp_k, exp_l) = match (items.get(ntoi(k)), items.get(ntoi(l))) {
              (Some((Ok(exp_k), _)), Some((Ok(exp_l), _))) => (exp_k, exp_l),
              _ => return false,
            };
            match (exp_row, exp_k) {
              (Exp::Falsum, _) => (exp_k.negated() == *exp_l) || (*exp_k == exp_l.negated()),
              (
                _,
                Exp::Cond {
                  antecedent: exp_k_antecedent,
                  consequent: exp_k_consequent,
                },
              ) => (**exp_k_antecedent == *exp_l) && (**exp_k_consequent == *exp_row),
              _ => false,
            }
          }

          Rule::IffIntro(k, l) => {
            let (exp_k, exp_l) = match (items.get(ntoi(k)), items.get(ntoi(l))) {
              (Some((Ok(exp_k), _)), Some((Ok(exp_l), _))) => (exp_k, exp_l),
              _ => return false,
            };
            match (exp_row, exp_k, exp_l) {
              (
                Exp::Iff { lhs, rhs },
                Exp::Cond {
                  antecedent: exp_k_antecedent,
                  consequent: exp_k_consequent,
                },
                Exp::Cond {
                  antecedent: exp_l_antecedent,
                  consequent: exp_l_consequent,
                },
              ) => {
                unordered_tuple_eq((lhs, rhs), (exp_k_antecedent, exp_k_consequent))
                  && (*exp_k_antecedent == *exp_l_consequent)
                  && (*exp_k_consequent == *exp_l_antecedent)
              }
              _ => false,
            }
          }

          Rule::IffExclude(k) => {
            let exp_k = match items.get(ntoi(k)) {
              Some((Ok(exp_k), _)) => exp_k,
              _ => return false,
            };
            match (exp_row, exp_k) {
              (
                Exp::Cond { antecedent, consequent },
                Exp::Iff {
                  lhs: exp_k_lhs,
                  rhs: exp_k_rhs,
                },
              ) => unordered_tuple_eq((antecedent, consequent), (exp_k_lhs, exp_k_rhs)),
              _ => false,
            }
          }

          Rule::Falsum(k) => {
            let exp_k = match items.get(ntoi(k)) {
              Some((Ok(exp_k), _)) => exp_k,
              _ => return false,
            };
            *exp_k == Exp::Falsum
          }

          Rule::NegIntro((k0, k1)) => {
            let (exp_k0, exp_k1) = match (items.get(ntoi(k0)), items.get(ntoi(k1))) {
              (Some((Ok(exp_k0), _)), Some((Ok(exp_k1), _))) => (exp_k0, exp_k1),
              _ => return false,
            };
            match (exp_row, exp_k1) {
              (Exp::Neg(negated), Exp::Falsum) => **negated == *exp_k0,
              _ => false,
            }
          }

          Rule::NegExclude((k0, k1)) => {
            let (exp_k0, exp_k1) = match (items.get(ntoi(k0)), items.get(ntoi(k1))) {
              (Some((Ok(exp_k0), _)), Some((Ok(exp_k1), _))) => (exp_k0, exp_k1),
              _ => return false,
            };
            match (exp_row, exp_k0, exp_k1) {
              (exp_row, Exp::Neg(exp_k_negated), Exp::Falsum) => *exp_row == **exp_k_negated,
              _ => false,
            }
          }

          Rule::UnivQuntIntro(k) => {
            let exp_k = match items.get(ntoi(k)) {
              Some((Ok(exp_k), _)) => exp_k,
              _ => return false,
            };
            match exp_row {
              Exp::UnivGenr { variable, form: inner } => {
                let beta = match one_or_none(&(&exp_k.free_variables() - &inner.free_variables())) {
                  Some(beta) => beta,
                  None => return false,
                };
                let deps = match self.deps_list.get(ntoi(k)) {
                  Some(deps) => deps,
                  None => return false,
                };
                if deps.nums.iter().any(|&num_dep| {
                  let exp_dep = match items.get(ntoi(num_dep)) {
                    Some((Ok(exp_dep), _)) => exp_dep,
                    _ => return true,
                  };
                  exp_dep.free_variables().contains(&beta)
                }) {
                  return false;
                }
                inner.var_replaced(&variable, &beta) == *exp_k
              }
              _ => false,
            }
          }

          Rule::UnivQuntExclude(k) => {
            let exp_k = match items.get(ntoi(k)) {
              Some((Ok(exp_k), _)) => exp_k,
              _ => return false,
            };

            match exp_k {
              Exp::UnivGenr { variable: alpha, form } => {
                let beta = match one_or_none(&(&exp_row.free_variables() - &form.free_variables())) {
                  Some(beta) => beta,
                  None => return false,
                };
                form.var_replaced(&alpha, &beta) == *exp_row
              }
              _ => false,
            }
          }

          Rule::ExisQuntIntro(k) => {
            let exp_k = match items.get(ntoi(k)) {
              Some((Ok(exp_k), _)) => exp_k,
              _ => return false,
            };
            match exp_row {
              Exp::ExistGenr { variable: alpha, form } => {
                let beta = match one_or_none(&(&exp_k.free_variables() - &form.free_variables())) {
                  Some(beta) => beta,
                  None => return false,
                };
                form.var_replaced(&alpha, &beta) == *exp_k
              }
              _ => false,
            }
          }

          Rule::ExisQuntExclude(k, (l, m)) => {
            let (exp_k, exp_l, rule_l, exp_m) = match (items.get(ntoi(k)), items.get(ntoi(l)), items.get(ntoi(m))) {
              (Some((Ok(exp_k), _)), Some((Ok(exp_l), Ok(rule_l))), Some((Ok(exp_m), _))) => {
                (exp_k, exp_l, rule_l, exp_m)
              }
              _ => return false,
            };
            match (exp_k, rule_l) {
              (Exp::ExistGenr { variable: alpha, form }, Rule::Premise) => {
                let beta =
                  match one_or_none(&(&(&exp_l.free_variables() - &form.free_variables()) - &exp_m.free_variables())) {
                    Some(beta) => beta,
                    None => return false,
                  };
                let deps = match self.deps_list.get(ntoi(m)) {
                  Some(deps) => deps,
                  _ => return false,
                };
                if deps.nums.iter().filter(|&n| *n != l).any(|&num_dep| {
                  let exp_dep = match items.get(ntoi(num_dep)) {
                    Some((Ok(exp_dep), _)) => exp_dep,
                    _ => return true,
                  };
                  exp_dep.free_variables().contains(&beta)
                }) {
                  return false;
                }
                (form.var_replaced(&alpha, &beta) == *exp_l) && (exp_m == exp_row)
              }
              _ => false,
            }
          }
        }
      })
      .collect_vec()
  }
}

#[derive(Clone)]
pub struct RowDependency {
  pub is_complete: bool,
  pub nums: HashSet<usize>,
}

impl RowDependency {
  fn new() -> Self {
    RowDependency {
      is_complete: true,
      nums: HashSet::new(),
    }
  }

  fn new_incomplete() -> Self {
    RowDependency {
      is_complete: false,
      nums: HashSet::new(),
    }
  }

  fn init_from<const N: usize>(nums: [usize; N]) -> Self {
    RowDependency {
      is_complete: true,
      nums: HashSet::from(nums),
    }
  }
}

impl BitOr<&RowDependency> for RowDependency {
  type Output = RowDependency;

  fn bitor(self, rhs: &RowDependency) -> Self::Output {
    RowDependency {
      is_complete: self.is_complete && rhs.is_complete,
      nums: &self.nums | &rhs.nums,
    }
  }
}

impl BitOr<Option<&RowDependency>> for RowDependency {
  type Output = RowDependency;

  fn bitor(self, rhs: Option<&RowDependency>) -> Self::Output {
    match rhs {
      Some(rhs) => self | rhs,
      None => self,
    }
  }
}

impl Sub<usize> for RowDependency {
  type Output = RowDependency;

  fn sub(self, rhs: usize) -> Self::Output {
    let mut nums = self.nums.clone();
    nums.remove(&rhs);
    RowDependency {
      is_complete: self.is_complete,
      nums,
    }
  }
}

impl Sub<Option<usize>> for RowDependency {
  type Output = RowDependency;

  fn sub(self, rhs: Option<usize>) -> Self::Output {
    match rhs {
      Some(rhs) => self - rhs,
      None => self,
    }
  }
}
