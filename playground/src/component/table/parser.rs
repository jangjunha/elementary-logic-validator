use cached::proc_macro::cached;
use language::{ast::exp::Exp, parser::expression::exp as parse_exp_base};
use language_derivation_rule::{ast::rule::Rule, parser::rule::rule as parse_rule_base};

#[cached(size = 64, key = "String", convert = r#"{ format!("{}", s) }"#)]
pub fn parse_exp(s: &str) -> Result<Exp, ()> {
  match parse_exp_base(s) {
    Ok(("", exp)) => Ok(exp),
    Ok(_) | Err(_) => Err(()),
  }
}

#[cached(size = 64, key = "String", convert = r#"{ format!("{}", s) }"#)]
pub fn parse_rule(s: &str) -> Result<Rule, ()> {
  match parse_rule_base(s) {
    Ok(("", rule)) => Ok(rule),
    Ok(_) | Err(_) => Err(()),
  }
}
