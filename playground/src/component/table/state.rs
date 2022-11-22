use language::parser::expression::exp as parse_exp;
use language_derivation_rule::parser::rule::rule as parse_rule;
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
        rows.insert(
          after_num,
          Row {
            sentence: "".to_owned(),
            derivation: "".to_owned(),
          },
        );
        // TODO: derivation refs 변경
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
