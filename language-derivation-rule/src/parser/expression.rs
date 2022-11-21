use super::{
  individual_constant::{dim, ind_sym, pre, var},
  symbol::{and, existential, falsum, left_right_arrow, negation, or, right_arrow},
  util::ws,
};
use crate::ast::exp::Exp;
use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::multispace0,
  combinator::{map, value},
  multi::{count, many0},
  sequence::{delimited, pair, preceded, tuple},
  IResult,
};

pub fn exp(s: &str) -> IResult<&str, Exp> {
  cond_exp(s)
}

fn _explicit_ind_sym(s: &str) -> IResult<&str, Vec<&str>> {
  let (s1, n) = dim(s)?;
  count(preceded(multispace0, ind_sym), n as usize)(s1)
}

fn _implicit_ind_sym(s: &str) -> IResult<&str, Vec<&str>> {
  many0(preceded(multispace0, ind_sym))(s)
}

fn atom_exp(s: &str) -> IResult<&str, Exp> {
  map(pair(pre, alt((_explicit_ind_sym, _implicit_ind_sym))), |(p, i)| {
    Exp::Atom {
      predicate: p.to_owned(),
      individuals: i.iter().map(|&e| e.to_owned()).collect(),
    }
  })(s)
}

fn if_exp(s: &str) -> IResult<&str, Exp> {
  map(tuple((bool_exp, ws(right_arrow), cond_exp)), |(lhs, _, rhs)| {
    Exp::Cond {
      antecedent: Box::new(lhs),
      consequent: Box::new(rhs),
    }
  })(s)
}

fn iff_exp(s: &str) -> IResult<&str, Exp> {
  map(tuple((bool_exp, ws(left_right_arrow), cond_exp)), |(lhs, _, rhs)| {
    Exp::Iff {
      lhs: Box::new(lhs),
      rhs: Box::new(rhs),
    }
  })(s)
}

fn cond_exp(s: &str) -> IResult<&str, Exp> {
  alt((if_exp, iff_exp, bool_exp))(s)
}

fn and_exp(s: &str) -> IResult<&str, Exp> {
  map(tuple((f, ws(and), bool_exp)), |(lhs, _, rhs)| Exp::And {
    lhs: Box::new(lhs),
    rhs: Box::new(rhs),
  })(s)
}

fn or_exp(s: &str) -> IResult<&str, Exp> {
  map(tuple((f, ws(or), bool_exp)), |(lhs, _, rhs)| Exp::Or {
    lhs: Box::new(lhs),
    rhs: Box::new(rhs),
  })(s)
}

fn bool_exp(s: &str) -> IResult<&str, Exp> {
  alt((and_exp, or_exp, f))(s)
}

fn negate_exp(s: &str) -> IResult<&str, Exp> {
  map(preceded(negation, preceded(multispace0, f)), |e| Exp::Neg(Box::new(e)))(s)
}

fn parenthesesed_exp(s: &str) -> IResult<&str, Exp> {
  delimited(tag("("), ws(exp), tag(")"))(s)
}

fn univ_genr_exp(s: &str) -> IResult<&str, Exp> {
  map(
    pair(delimited(tag("("), ws(var), tag(")")), preceded(multispace0, f)),
    |(v, e)| Exp::UnivGenr {
      variable: v.to_owned(),
      form: Box::new(e),
    },
  )(s)
}

fn exist_genr_exp(s: &str) -> IResult<&str, Exp> {
  map(
    pair(
      delimited(tuple((tag("("), preceded(multispace0, existential))), ws(var), tag(")")),
      preceded(multispace0, f),
    ),
    |(v, e)| Exp::ExistGenr {
      variable: v.to_owned(),
      form: Box::new(e),
    },
  )(s)
}

fn genr_exp(s: &str) -> IResult<&str, Exp> {
  alt((exist_genr_exp, univ_genr_exp))(s)
}

fn falsum_exp(s: &str) -> IResult<&str, Exp> {
  value(Exp::Falsum, falsum)(s)
}

fn f(s: &str) -> IResult<&str, Exp> {
  alt((atom_exp, falsum_exp, negate_exp, genr_exp, parenthesesed_exp))(s)
}

#[cfg(test)]
mod tests {
  use super::*;
  use nom::IResult;

  #[test]
  fn exp_valid() {
    let exp1 = Exp::Iff {
      lhs: Box::new(Exp::ExistGenr {
        variable: "y".to_owned(),
        form: Box::new(Exp::And {
          lhs: Box::new(Exp::Atom {
            predicate: "F".to_owned(),
            individuals: vec!["y".to_owned()],
          }),
          rhs: Box::new(Exp::Atom {
            predicate: "G".to_owned(),
            individuals: vec!["y".to_owned(), "y".to_owned()],
          }),
        }),
      }),
      rhs: Box::new(Exp::ExistGenr {
        variable: "y".to_owned(),
        form: Box::new(Exp::And {
          lhs: Box::new(Exp::Atom {
            predicate: "F".to_owned(),
            individuals: vec!["y".to_owned()],
          }),
          rhs: Box::new(Exp::ExistGenr {
            variable: "x".to_owned(),
            form: Box::new(Exp::And {
              lhs: Box::new(Exp::Atom {
                predicate: "F".to_owned(),
                individuals: vec!["x".to_owned()],
              }),
              rhs: Box::new(Exp::Atom {
                predicate: "G".to_owned(),
                individuals: vec!["y".to_owned(), "x".to_owned()],
              }),
            }),
          }),
        }),
      }),
    };
    assert_eq!(
      exp("(∃y)(Fy & Gyy) ↔ (∃y)(Fy & (∃x)(Fx & Gyx))"),
      IResult::Ok(("", exp1.clone()))
    );
    assert_eq!(
      exp("( ∃ y )(Fy&Gyy) <-> (∃y)( F y & (∃x)( F x & G yx ))"),
      IResult::Ok(("", exp1.clone()))
    );
    assert_eq!(
      exp("--P"),
      IResult::Ok((
        "",
        Exp::Neg(Box::new(Exp::Neg(Box::new(Exp::Atom {
          predicate: "P".to_owned(),
          individuals: vec![]
        }))))
      ))
    );
    assert_eq!(
      exp("(x)-Rx"),
      IResult::Ok((
        "",
        Exp::UnivGenr {
          variable: "x".to_owned(),
          form: Box::new(Exp::Neg(Box::new(Exp::Atom {
            predicate: "R".to_owned(),
            individuals: vec!["x".to_owned()]
          }))),
        }
      ))
    );
  }

  #[test]
  fn atom_exp_valid() {
    assert_eq!(
      atom_exp("P"),
      IResult::Ok((
        "",
        Exp::Atom {
          predicate: "P".to_owned(),
          individuals: vec![]
        }
      ))
    );
    assert_eq!(
      atom_exp("P_10"),
      IResult::Ok((
        "",
        Exp::Atom {
          predicate: "P_10".to_owned(),
          individuals: vec![]
        }
      ))
    );
    assert_eq!(
      atom_exp("R_2^3xya"),
      IResult::Ok((
        "",
        Exp::Atom {
          predicate: "R_2".to_owned(),
          individuals: vec!["x".to_owned(), "y".to_owned(), "a".to_owned()]
        }
      ))
    );
    assert_eq!(
      atom_exp("R_2xy_2a"),
      IResult::Ok((
        "",
        Exp::Atom {
          predicate: "R_2".to_owned(),
          individuals: vec!["x".to_owned(), "y_2".to_owned(), "a".to_owned()]
        }
      ))
    );
    assert_eq!(
      atom_exp("R_2 x y a"),
      IResult::Ok((
        "",
        Exp::Atom {
          predicate: "R_2".to_owned(),
          individuals: vec!["x".to_owned(), "y".to_owned(), "a".to_owned()]
        }
      ))
    );
  }

  #[test]
  fn atom_exp_invalid() {
    assert_eq!(
      atom_exp("P^2AB"),
      IResult::Ok((
        "^2AB",
        Exp::Atom {
          predicate: "P".to_owned(),
          individuals: vec![]
        }
      ))
    );
    assert_eq!(
      atom_exp("P^2x"),
      IResult::Ok((
        "^2x",
        Exp::Atom {
          predicate: "P".to_owned(),
          individuals: vec![]
        }
      ))
    );
    assert_eq!(
      atom_exp("P^2xyz"),
      IResult::Ok((
        "z",
        Exp::Atom {
          predicate: "P".to_owned(),
          individuals: vec!["x".to_owned(), "y".to_owned()]
        }
      ))
    );
    assert_eq!(
      atom_exp("P ^2"),
      IResult::Ok((
        " ^2",
        Exp::Atom {
          predicate: "P".to_owned(),
          individuals: vec![]
        }
      ))
    );
    assert_eq!(
      atom_exp("R^1_2x"),
      IResult::Ok((
        "^1_2x",
        Exp::Atom {
          predicate: "R".to_owned(),
          individuals: vec![]
        }
      ))
    );
  }
}
