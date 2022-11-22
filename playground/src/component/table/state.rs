use itertools::Itertools;
use language::parser::expression::exp as parse_exp;
use language_derivation_rule::parser::rule::rule as parse_rule;
use lazy_static::lazy_static;
use regex::Regex;
use yew::Reducible;

pub struct State {
  pub rows: Vec<Row>,
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

impl Reducible for State {
  type Action = Action;

  fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
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
        State { rows }.into()
      }
      Action::ChangeSentence { num, sentence } => {
        let mut rows = self.rows.clone();
        if let Some(row) = rows.get_mut(num - 1) {
          row.sentence = sentence;
        }
        State { rows }.into()
      }
      Action::ChangeDerivation { num, derivation } => {
        let mut rows = self.rows.clone();
        if let Some(row) = rows.get_mut(num - 1) {
          row.derivation = derivation;
        }
        State { rows }.into()
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
        State { rows }.into()
      }
    }
  }
}
