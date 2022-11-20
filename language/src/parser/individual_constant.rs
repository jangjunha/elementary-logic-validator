use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::{digit1, satisfy},
  combinator::{map_res, opt, recognize},
  sequence::{pair, preceded},
  IResult,
};

fn dim_num(s: &str) -> IResult<&str, u8> {
  map_res(digit1, |s: &str| s.parse::<u8>())(s)
}

fn subscr(s: &str) -> IResult<&str, &str> {
  recognize(pair(tag("_"), digit1))(s)
}

pub fn dim(s: &str) -> IResult<&str, u8> {
  preceded(tag("^"), dim_num)(s)
}

fn var_ch(s: &str) -> IResult<&str, char> {
  satisfy(|c| c >= 'u' && c <= 'z')(s)
}

fn ind_ch(s: &str) -> IResult<&str, char> {
  satisfy(|c| c >= 'a' && c <= 't')(s)
}

pub fn var(s: &str) -> IResult<&str, &str> {
  recognize(pair(var_ch, opt(subscr)))(s)
}

pub fn ind(s: &str) -> IResult<&str, &str> {
  recognize(pair(ind_ch, opt(subscr)))(s)
}

// 개체문자(개체상항 + 변항)
pub fn ind_sym(s: &str) -> IResult<&str, &str> {
  alt((var, ind))(s)
}

pub fn pre(s: &str) -> IResult<&str, &str> {
  recognize(pair(satisfy(|c| c >= 'A' && c <= 'Z'), opt(subscr)))(s)
}

#[cfg(test)]
mod tests {
  use super::*;
  use nom::IResult;

  #[test]
  fn subscr_valid() {
    assert_eq!(subscr("_5"), IResult::Ok(("", "_5")));
    assert_eq!(subscr("_12"), IResult::Ok(("", "_12")));
  }

  #[test]
  fn subscr_invalid() {
    assert!(subscr("^5").is_err());
    assert!(subscr("_ 12").is_err());
    assert!(subscr("12").is_err());
    assert!(subscr("_").is_err());
    assert!(subscr("_a").is_err());
    assert!(subscr("_-1").is_err());
    assert!(subscr("_(-1)").is_err());
  }

  #[test]
  fn dim_valid() {
    assert_eq!(dim("^5"), IResult::Ok(("", 5u8)));
    assert_eq!(dim("^12"), IResult::Ok(("", 12u8)));
  }

  #[test]
  fn dim_invalid() {
    assert!(dim("_5").is_err());
    assert!(dim("^ 12").is_err());
    assert!(dim("12").is_err());
    assert!(dim("^").is_err());
    assert!(dim("^a").is_err());
    assert!(dim("^-1").is_err());
    assert!(dim("^(-1)").is_err());
  }

  #[test]
  fn pre_valid() {
    assert_eq!(pre("P"), IResult::Ok(("", "P")));
    assert_eq!(pre("P_10"), IResult::Ok(("", "P_10")));
    assert_eq!(pre("R_2"), IResult::Ok(("", "R_2")));
  }

  #[test]
  fn pre_invalid() {
    assert!(pre("피").is_err());
    assert_eq!(pre("P'"), IResult::Ok(("'", "P")));
  }
}
