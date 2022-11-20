use nom::{branch::alt, bytes::complete::tag, IResult};

pub fn right_arrow(s: &str) -> IResult<&str, &str> {
  alt((tag("→"), tag("->")))(s)
}

pub fn left_right_arrow(s: &str) -> IResult<&str, &str> {
  alt((tag("↔"), tag("<->")))(s)
}

pub fn and(s: &str) -> IResult<&str, &str> {
  tag("&")(s)
}

pub fn or(s: &str) -> IResult<&str, &str> {
  alt((tag("∨"), tag("|")))(s)
}

pub fn negation(s: &str) -> IResult<&str, &str> {
  alt((tag("¬"), tag("-")))(s)
}

pub fn existential(s: &str) -> IResult<&str, &str> {
  alt((tag("∃"), tag("]")))(s)
}

pub fn falsum(s: &str) -> IResult<&str, &str> {
  alt((tag("⊥"), tag("\\bot")))(s)
}
