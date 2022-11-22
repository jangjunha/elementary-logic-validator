use std::{
  collections::HashSet,
  ops::{BitOr, Sub},
};

use itertools::Itertools;
use language::parser::expression::exp as parse_exp;
use language_derivation_rule::{ast::rule::Rule, parser::rule::rule as parse_rule};
use lazy_static::lazy_static;
use regex::Regex;
use yew::Reducible;

pub struct State {
  pub rows: Vec<Row>,

  // computed properties (memoized)
  pub deps_list: Vec<RowDependency>,
}

#[derive(Clone)]
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
    }
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
        };
        next.deps_list = next.get_deps_for_rows();
        next.into()
      }

      Action::ChangeSentence { num, sentence } => {
        let mut rows = self.rows.clone();
        if let Some(row) = rows.get_mut(num - 1) {
          row.sentence = sentence;
        }

        State {
          rows,
          deps_list: self.deps_list.clone(),
        }
        .into()
      }

      Action::ChangeDerivation { num, derivation } => {
        let mut rows = self.rows.clone();
        if let Some(row) = rows.get_mut(num - 1) {
          row.derivation = derivation;
        }

        let mut next = State {
          rows,
          deps_list: vec![],
        };
        next.deps_list = next.get_deps_for_rows();
        next.into()
      }

      Action::Format => {
        let rows = self
          .rows
          .iter()
          .map(|row| {
            let mut row = row.clone();
            if let Ok(("", exp)) = parse_exp(&row.sentence.trim()) {
              row.sentence = exp.to_string();
            }
            if let Ok(("", rule)) = parse_rule(&row.derivation.trim()) {
              row.derivation = rule.to_string();
            }
            row
          })
          .collect();
        State {
          rows,
          deps_list: self.deps_list.clone(),
        }
        .into()
      }
    }
  }
}

impl State {
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
        Ok(("", rule)) => match &rule {
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
        Ok(_) | Err(_) => RowDependency::new_incomplete(),
      };
      acc.push(dep);
      acc
    })
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
