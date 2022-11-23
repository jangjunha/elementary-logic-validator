use std::{
  collections::HashSet,
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
}

impl State {
  pub fn init() -> Self {
    State {
      rows: vec![Row {
        sentence: "".to_owned(),
        derivation: "".to_owned(),
      }],
      deps_list: vec![RowDependency::new_incomplete()],
      rule_vaildity_list: vec![false],
    }
  }

  pub fn init_from(rows: Vec<Row>) -> Self {
    let mut state = State {
      rows,
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
            if let Ok(exp) = parse_exp(&row.sentence.trim()) {
              row.sentence = exp.to_string();
            }
            if let Ok(rule) = parse_rule(&row.derivation.trim()) {
              row.derivation = rule.to_string();
            }
            row
          })
          .collect();
        State {
          rows,
          deps_list: self.deps_list.clone(),
          rule_vaildity_list: self.rule_vaildity_list.clone(),
        }
        .into()
      }
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
      let dep = match parse_rule(&row.derivation.trim()) {
        Ok(rule) => match &rule {
          Rule::Premise => RowDependency::init_from([row_num]),
          Rule::AndIntro(k, l) => RowDependency::new() | acc.get(ntoi(*k)) | acc.get(ntoi(*l)),
          Rule::AndExclude(k) => RowDependency::new() | acc.get(ntoi(*k)),
          Rule::OrIntro(k, l) => RowDependency::new() | acc.get(ntoi(*k)) | l.and_then(|l| acc.get(l + 1)),
          Rule::OrExclude(k, (l0, l1), (m0, m1)) => {
            (RowDependency::new() | acc.get(ntoi(*k)) | acc.get(ntoi(*l1)) | acc.get(ntoi(*m1)))
              - (ntoi(*l0))
              - (ntoi(*m0))
          }
          Rule::IfIntro((k0, k1)) => (RowDependency::new() | acc.get(ntoi(*k1))) - (k0.map(ntoi)),
          Rule::IfExclude(k, l) => RowDependency::new() | acc.get(ntoi(*k)) | acc.get(ntoi(*l)),
          Rule::Falsum(k) => RowDependency::new() | acc.get(ntoi(*k)),
          Rule::NegIntro((k0, k1)) | Rule::NegExclude((k0, k1)) => {
            (RowDependency::new() | acc.get(ntoi(*k1))) - (ntoi(*k0))
          }
          Rule::IffIntro(k, l) => RowDependency::new() | acc.get(ntoi(*k)) | acc.get(ntoi(*l)),
          Rule::IffExclude(k) => RowDependency::new() | acc.get(ntoi(*k)),
          Rule::UnivQuntIntro(k) | Rule::UnivQuntExclude(k) => RowDependency::new() | acc.get(ntoi(*k)),
          Rule::ExisQuntIntro(k) => RowDependency::new() | acc.get(ntoi(*k)),
          Rule::ExisQuntExclude(k, (l0, l1)) => {
            (RowDependency::new() | acc.get(ntoi(*k)) | acc.get(ntoi(*l1))) - (ntoi(*l0))
          }
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
    fn unordered_tuple_eq((a1, a2): (&Exp, &Exp), (b1, b2): (&Exp, &Exp)) -> bool {
      ((a1 == b1) && (a2 == b2)) || ((a1 == b2) && (a2 == b1))
    }

    let items = self
      .rows
      .iter()
      .map(|row| match parse_exp(&row.sentence.trim()) {
        Ok(exp) => (Ok(exp), &row.derivation),
        Err(_) => (Err(()), &row.derivation),
      })
      .map(|(exp, derivation)| {
        (
          exp,
          match parse_rule(derivation.trim()) {
            Ok(rule) => (Ok(rule)),
            Err(_) => (Err(())),
          },
        )
      })
      .collect_vec();

    items
      .iter()
      .map(|(row_exp, row_rule)| match (row_exp, row_rule) {
        (Ok(row_exp), Ok(row_rule)) => match *row_rule {
          Rule::Premise => true,
          Rule::AndIntro(k, l) => match (items.get(ntoi(k)), items.get(ntoi(l))) {
            (Some((Ok(exp_k), _)), Some((Ok(exp_l), _))) => match row_exp {
              Exp::And { lhs, rhs } => (**lhs == *exp_k) && (**rhs == *exp_l),
              _ => false,
            },
            _ => false,
          },
          Rule::AndExclude(k) => match items.get(ntoi(k)) {
            Some((Ok(exp_k), _)) => match exp_k {
              Exp::And {
                lhs: exp_k_lhs,
                rhs: exp_k_rhs,
              } => (*row_exp == **exp_k_lhs) || (*row_exp == **exp_k_rhs),
              _ => false,
            },
            _ => false,
          },
          Rule::OrIntro(k, None) => match items.get(ntoi(k)) {
            Some((Ok(exp_k), _)) => match row_exp {
              Exp::Or { lhs, rhs } => (*exp_k == **lhs) || (*exp_k == **rhs),
              _ => false,
            },
            _ => false,
          },
          Rule::OrIntro(k, Some(l)) => match (items.get(ntoi(k)), items.get(ntoi(l))) {
            (Some((Ok(exp_k), _)), Some((Ok(exp_l), _))) => match row_exp {
              Exp::Or { lhs, rhs } => unordered_tuple_eq((exp_k, exp_l), (lhs, rhs)),
              _ => false,
            },
            _ => false,
          },
          Rule::OrExclude(k, (l0, l1), (m0, m1)) => {
            match (
              items.get(ntoi(k)),
              items.get(ntoi(l0)),
              items.get(ntoi(l1)),
              items.get(ntoi(m0)),
              items.get(ntoi(m1)),
            ) {
              (
                Some((Ok(exp_k), _)),
                Some((Ok(exp_l0), Ok(rule_l0))),
                Some((Ok(exp_l1), Ok(rule_l1))),
                Some((Ok(exp_m0), _)),
                Some((Ok(exp_m1), _)),
              ) => match (exp_k, rule_l0, rule_l1) {
                (
                  Exp::Or {
                    lhs: exp_k_lhs,
                    rhs: exp_k_rhs,
                  },
                  Rule::Premise,
                  Rule::Premise,
                ) => {
                  unordered_tuple_eq((exp_k_lhs, exp_k_rhs), (exp_l0, exp_m0))
                    && (row_exp == exp_l1)
                    && (row_exp == exp_m1)
                }
                _ => false,
              },
              _ => false,
            }
          }
          Rule::IfIntro((Some(k0), k1)) => match (items.get(ntoi(k0)), items.get(ntoi(k1))) {
            (Some((Ok(exp_k0), Ok(Rule::Premise))), Some((Ok(exp_k1), _))) => match row_exp {
              Exp::Cond { antecedent, consequent } => (*exp_k0 == **antecedent) && (*exp_k1 == **consequent),
              _ => false,
            },
            _ => false,
          },
          Rule::IfIntro((None, k)) => match items.get(ntoi(k)) {
            Some((Ok(exp_k), _)) => match row_exp {
              Exp::Cond { consequent, .. } => *exp_k == **consequent,
              _ => false,
            },
            _ => false,
          },
          Rule::IfExclude(k, l) => match (items.get(ntoi(k)), items.get(ntoi(l))) {
            (Some((Ok(exp_k), _)), Some((Ok(exp_l), _))) => match (row_exp, exp_l) {
              (Exp::Falsum, _) => (exp_k.negated() == *exp_l) || (*exp_k == exp_l.negated()),
              (
                _,
                Exp::Cond {
                  antecedent: exp_k_antecedent,
                  consequent: exp_k_consequent,
                },
              ) => (**exp_k_antecedent == *exp_l) && (**exp_k_consequent == *row_exp),
              _ => false,
            },
            _ => false,
          },
          Rule::IffIntro(k, l) => match (items.get(ntoi(k)), items.get(ntoi(l))) {
            (Some((Ok(exp_k), _)), Some((Ok(exp_l), _))) => match (row_exp, exp_k, exp_l) {
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
            },
            _ => false,
          },
          Rule::IffExclude(k) => match items.get(ntoi(k)) {
            Some((Ok(exp_k), _)) => match (row_exp, exp_k) {
              (
                Exp::Cond { antecedent, consequent },
                Exp::Iff {
                  lhs: exp_k_lhs,
                  rhs: exp_k_rhs,
                },
              ) => unordered_tuple_eq((antecedent, consequent), (exp_k_lhs, exp_k_rhs)),
              _ => false,
            },
            _ => false,
          },
          Rule::Falsum(k) => match items.get(ntoi(k)) {
            Some((Ok(exp_k), _)) => *exp_k == Exp::Falsum,
            _ => false,
          },
          Rule::NegIntro((k0, k1)) => match (items.get(ntoi(k0)), items.get(ntoi(k1))) {
            (Some((Ok(exp_k0), _)), Some((Ok(exp_k1), _))) => match (row_exp, exp_k1) {
              (Exp::Neg(negated), Exp::Falsum) => **negated == *exp_k0,
              _ => false,
            },
            _ => false,
          },
          Rule::NegExclude((k0, k1)) => match (items.get(ntoi(k0)), items.get(ntoi(k1))) {
            (Some((Ok(exp_k0), _)), Some((Ok(exp_k1), _))) => match (row_exp, exp_k0, exp_k1) {
              (exp, Exp::Neg(exp_k_negated), Exp::Falsum) => *exp == **exp_k_negated,
              _ => false,
            },
            _ => false,
          },
          Rule::UnivQuntIntro(k) => match items.get(ntoi(k)) {
            Some((Ok(exp_k), _)) => match row_exp {
              Exp::UnivGenr { variable, form: inner } => {
                let src_vars = exp_k.free_variables();
                let inn_vars = inner.free_variables();
                if let Some(beta) = src_vars.difference(&inn_vars).next() {
                  if let Some(deps) = self.deps_list.get(ntoi(k)) {
                    if deps.nums.iter().any(|&dep_num| {
                      if let Some((Ok(dep_exp), _)) = items.get(ntoi(dep_num)) {
                        dep_exp.free_variables().contains(beta)
                      } else {
                        true
                      }
                    }) {
                      return false;
                    }
                  } else {
                    return false;
                  }
                  inner.var_replaced(&variable, beta) == *exp_k
                } else {
                  false
                }
              }
              _ => false,
            },
            _ => false,
          },
          Rule::UnivQuntExclude(k) => match items.get(ntoi(k)) {
            Some((
              Ok(Exp::UnivGenr {
                variable: exp_k_var,
                form: exp_k_inner,
              }),
              _,
            )) => {
              // FIXME: 여기선 k에 등장하는 변수를 써도 됨
              let dst_vars = row_exp.free_variables();
              let inn_vars = exp_k_inner.free_variables();
              if let Some(beta) = dst_vars.difference(&inn_vars).next() {
                exp_k_inner.var_replaced(&exp_k_var, beta) == *row_exp
              } else {
                false
              }
            }
            _ => false,
          },
          Rule::ExisQuntIntro(k) => match items.get(ntoi(k)) {
            Some((Ok(exp_k), _)) => match row_exp {
              Exp::ExistGenr { variable, form: inner } => {
                let src_vars = exp_k.free_variables();
                let inn_vars = inner.free_variables();
                // FIXME: 여기선 k에 등장하는 변수를 써도 됨
                if let Some(beta) = src_vars.difference(&inn_vars).next() {
                  inner.var_replaced(&variable, beta) == *exp_k
                } else {
                  false
                }
              }
              _ => false,
            },
            _ => false,
          },
          Rule::ExisQuntExclude(k, (l, m)) => match (items.get(ntoi(k)), items.get(ntoi(l)), items.get(ntoi(m))) {
            (
              Some((
                Ok(Exp::ExistGenr {
                  variable: exp_k_var,
                  form: exp_k_inner,
                }),
                _,
              )),
              Some((Ok(exp_l), Ok(Rule::Premise))),
              Some((Ok(exp_m), _)),
            ) => {
              let k_inn_vars = exp_k_inner.free_variables();
              let l_vars = exp_l.free_variables();
              let m_vars = exp_m.free_variables();
              if let Some(beta) = (&l_vars - &k_inn_vars).difference(&m_vars).next() {
                if let Some(deps) = self.deps_list.get(ntoi(m)) {
                  if deps.nums.iter().filter(|&n| *n != l).any(|&dep_num| {
                    if let Some((Ok(dep_exp), _)) = items.get(ntoi(dep_num)) {
                      dep_exp.free_variables().contains(beta)
                    } else {
                      true
                    }
                  }) {
                    return false;
                  }
                } else {
                  return false;
                }
                (exp_k_inner.var_replaced(&exp_k_var, beta) == *exp_l) && (exp_m == row_exp)
              } else {
                false
              }
            }
            _ => false,
          },
        },
        _ => false,
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
