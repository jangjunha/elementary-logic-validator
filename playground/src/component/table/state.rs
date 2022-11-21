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
        State { rows }.into()
      }
      Action::ChangeSentence { num, sentence } => {
        let mut rows = self.rows.clone();
        if let Some(row) = rows.get_mut(num - 1) {
          row.sentence = sentence;
        }
        State { rows }.into()
      }
    }
  }
}
