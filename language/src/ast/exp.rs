use std::collections::BTreeSet;

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

  pub fn negated(&self) -> Self {
    Exp::Neg(Box::new(self.clone()))
  }

  pub fn free_variables(&self) -> BTreeSet<String> {
    match self {
      Self::Atom { individuals, .. } => individuals.iter().cloned().collect(),
      Self::Cond {
        antecedent: lhs,
        consequent: rhs,
      }
      | Self::Iff { lhs, rhs }
      | Self::And { lhs, rhs }
      | Self::Or { lhs, rhs } => ((&lhs.free_variables()) | (&rhs.free_variables())),
      Self::Neg(lhs) => lhs.free_variables(),
      Self::UnivGenr { variable, form } | Self::ExistGenr { variable, form } => {
        let mut vars = form.free_variables();
        vars.remove(variable);
        vars
      }
      Self::Falsum => BTreeSet::new(),
    }
  }

  /// Replace free variable and returns new expression
  pub fn var_replaced(&self, alpha: &str, beta: &str) -> Self {
    match self {
      Self::Atom { predicate, individuals } => Self::Atom {
        predicate: predicate.clone(),
        individuals: individuals
          .iter()
          .map(|i| if i == alpha { beta.to_owned() } else { i.clone() })
          .collect(),
      },
      Self::Cond { antecedent, consequent } => Self::Cond {
        antecedent: Box::new(antecedent.var_replaced(alpha, beta)),
        consequent: Box::new(consequent.var_replaced(alpha, beta)),
      },
      Self::Iff { lhs, rhs } => Self::Iff {
        lhs: Box::new(lhs.var_replaced(alpha, beta)),
        rhs: Box::new(rhs.var_replaced(alpha, beta)),
      },
      Self::And { lhs, rhs } => Self::And {
        lhs: Box::new(lhs.var_replaced(alpha, beta)),
        rhs: Box::new(rhs.var_replaced(alpha, beta)),
      },
      Self::Or { lhs, rhs } => Self::Or {
        lhs: Box::new(lhs.var_replaced(alpha, beta)),
        rhs: Box::new(rhs.var_replaced(alpha, beta)),
      },
      Self::Neg(lhs) => Self::Neg(Box::new(lhs.var_replaced(alpha, beta))),
      Self::UnivGenr { variable, .. } if variable == alpha => self.clone(),
      Self::UnivGenr { variable, form } => Self::UnivGenr {
        variable: variable.clone(),
        form: Box::new(form.var_replaced(alpha, beta)),
      },
      Self::ExistGenr { variable, .. } if variable == alpha => self.clone(),
      Self::ExistGenr { variable, form } => Self::ExistGenr {
        variable: variable.clone(),
        form: Box::new(form.var_replaced(alpha, beta)),
      },
      Self::Falsum => Self::Falsum,
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
