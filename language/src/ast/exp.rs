#[derive(Clone, PartialEq, Debug)]
pub enum Exp {
  Atom {
    predicate: String,
    individuals: Vec<String>, // individual symbols
  }, // atomic formula
  Cond {
    antecedent: Box<Exp>,
    consequent: Box<Exp>,
  }, // conditional
  Iff {
    lhs: Box<Exp>,
    rhs: Box<Exp>,
  }, // biconditional
  And {
    lhs: Box<Exp>,
    rhs: Box<Exp>,
  }, // conjunction
  Or {
    lhs: Box<Exp>,
    rhs: Box<Exp>,
  }, // disjunctions
  Neg(Box<Exp>), // negation
  UnivGenr {
    variable: String,
    form: Box<Exp>,
  }, // universal generalization
  ExistGenr {
    variable: String,
    form: Box<Exp>,
  }, // existential generalization
  Falsum,
}

impl Exp {
  pub fn to_string(&self) -> String {
    match self {
      Exp::Atom { predicate, individuals } => format!("{}{}", predicate, individuals.join("")),
      Exp::Cond { antecedent, consequent } => format!("({} → {})", antecedent.to_string(), consequent.to_string()),
      Exp::Iff { lhs, rhs } => format!("({} ↔ {})", lhs.to_string(), rhs.to_string()),
      Exp::And { lhs, rhs } => format!("({} & {})", lhs.to_string(), rhs.to_string()),
      Exp::Or { lhs, rhs } => format!("({} ∨ {})", lhs.to_string(), rhs.to_string()),
      Exp::Neg(lhs) => format!("¬{}", lhs.to_string()),
      Exp::UnivGenr { variable, form } => format!("({}){}", variable, form.to_string()),
      Exp::ExistGenr { variable, form } => format!("(∃{}){}", variable, form.to_string()),
      Exp::Falsum => "⊥".to_owned(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Exp;
  use rstest::rstest;

  #[rstest]
  #[case(Exp::Atom { predicate: "R".to_owned(), individuals: vec![] }, "R")]
  #[case(
      Exp::Atom {
        predicate: "R".to_owned(),
        individuals: vec![
          "a".to_owned(),
          "b".to_owned(),
        ],
      },
      "Rab",
    )]
  #[case(
      Exp::Cond {
        antecedent: Box::new(Exp::Atom { predicate: "P".to_owned(), individuals: vec![] }),
        consequent: Box::new(Exp::Atom { predicate: "Q".to_owned(), individuals: vec![] }),
      },
      "(P → Q)",
    )]
  #[case(
      Exp::Iff {
        lhs: Box::new(Exp::Atom { predicate: "P".to_owned(), individuals: vec![] }),
        rhs: Box::new(Exp::Atom { predicate: "Q".to_owned(), individuals: vec![] }),
      },
      "(P ↔ Q)",
    )]
  #[case(
      Exp::And {
        lhs: Box::new(Exp::Atom { predicate: "P".to_owned(), individuals: vec![] }),
        rhs: Box::new(Exp::Atom { predicate: "Q".to_owned(), individuals: vec![] }),
      },
      "(P & Q)",
    )]
  #[case(
      Exp::Or {
        lhs: Box::new(Exp::Atom { predicate: "P".to_owned(), individuals: vec![] }),
        rhs: Box::new(Exp::Atom { predicate: "Q".to_owned(), individuals: vec![] }),
      },
      "(P ∨ Q)",
    )]
  #[case(
      Exp::Neg(
        Box::new(Exp::Atom { predicate: "P".to_owned(), individuals: vec![] }),
      ),
      "¬P",
    )]
  #[case(
      Exp::UnivGenr {
        variable: "x".to_owned(),
        form: Box::new(Exp::Atom { predicate: "R".to_owned(), individuals: vec!["x".to_owned()] }),
      },
      "(x)Rx",
    )]
  #[case(
      Exp::ExistGenr {
        variable: "x".to_owned(),
        form: Box::new(Exp::Atom { predicate: "R".to_owned(), individuals: vec!["x".to_owned()] }),
      },
      "(∃x)Rx",
    )]
  #[case(Exp::Falsum, "⊥")]
  #[case(
      Exp::UnivGenr {
        variable: "x".to_owned(), 
        form: Box::new(Exp::Cond {
          antecedent: Box::new(Exp::And {
            lhs: Box::new(Exp::UnivGenr {
              variable: "y".to_owned(),
              form: Box::new(Exp::Cond {
                antecedent: Box::new(Exp::Atom { predicate: "M".to_owned(), individuals: vec!["y".to_owned()] }), 
                consequent: Box::new(Exp::Atom {
                  predicate: "L".to_owned(),
                  individuals: vec!["y".to_owned(), "x".to_owned()],
                }),
              })
            }),
            rhs: Box::new(Exp::Atom { predicate: "W".to_owned(), individuals: vec!["x".to_owned()] }),
          }),
          consequent: Box::new(Exp::Neg(Box::new(Exp::ExistGenr {
            variable: "z".to_owned(),
            form: Box::new(Exp::And {
              lhs: Box::new(Exp::Atom { predicate: "W".to_owned(), individuals: vec!["z".to_owned()] }),
              rhs: Box::new(Exp::Atom {
                predicate: "L".to_owned(),
                individuals: vec!["z".to_owned(), "x".to_owned()],
              }),
            }),
          }))),
        }),
      },
      "(x)(((y)(My → Lyx) & Wx) → ¬(∃z)(Wz & Lzx))",
    )]
  fn exp_to_string(#[case] input: Exp, #[case] expected: &str) {
    assert_eq!(expected, input.to_string())
  }
}
