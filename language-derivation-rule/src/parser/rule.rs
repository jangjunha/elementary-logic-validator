use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::{digit1, multispace1},
  combinator::{
    opt, {map, map_res, value},
  },
  sequence::{pair, preceded, separated_pair, terminated, tuple},
  IResult,
};

use language::parser::{
  symbol::{and, existential, falsum as falsum_tag, left_right_arrow, negation, or, right_arrow},
  util::ws,
};

use crate::ast::rule::Rule;

fn num(s: &str) -> IResult<&str, i32> {
  map_res(digit1, |s: &str| s.parse::<i32>())(s)
}

fn sep(s: &str) -> IResult<&str, &str> {
  ws(tag(","))(s)
}

fn range(s: &str) -> IResult<&str, (i32, i32)> {
  separated_pair(num, ws(tag("-")), num)(s)
}

fn premise(s: &str) -> IResult<&str, Rule> {
  value(Rule::Premise, tag("P"))(s)
}

fn and_intro(s: &str) -> IResult<&str, Rule> {
  map(
    terminated(
      separated_pair(num, sep, num),
      preceded(multispace1, pair(and, tag("I"))),
    ),
    |(k, l)| Rule::AndIntro(k, l),
  )(s)
}

fn and_exclude(s: &str) -> IResult<&str, Rule> {
  map(terminated(num, preceded(multispace1, pair(and, tag("E")))), |k| {
    Rule::AndExclude(k)
  })(s)
}

fn or_intro(s: &str) -> IResult<&str, Rule> {
  map(
    terminated(
      pair(num, opt(preceded(sep, num))),
      preceded(multispace1, pair(or, tag("I"))),
    ),
    |(k, l)| Rule::OrIntro(k, l),
  )(s)
}

fn or_exclude(s: &str) -> IResult<&str, Rule> {
  map(
    terminated(
      tuple((num, sep, range, sep, range)),
      preceded(multispace1, pair(or, tag("E"))),
    ),
    |(k, _, (l1, m1), _, (l2, m2))| Rule::OrExclude(k, (l1, m1), (l2, m2)),
  )(s)
}

fn if_intro(s: &str) -> IResult<&str, Rule> {
  map(
    terminated(
      pair(opt(terminated(num, ws(tag("-")))), num),
      preceded(multispace1, pair(right_arrow, tag("I"))),
    ),
    |(k, l)| Rule::IfIntro((k, l)),
  )(s)
}

fn if_exclude(s: &str) -> IResult<&str, Rule> {
  map(
    terminated(
      separated_pair(num, sep, num),
      preceded(multispace1, pair(right_arrow, tag("E"))),
    ),
    |(k, l)| Rule::IfExclude(k, l),
  )(s)
}

fn iff_intro(s: &str) -> IResult<&str, Rule> {
  map(
    terminated(
      separated_pair(num, sep, num),
      preceded(multispace1, pair(left_right_arrow, tag("I"))),
    ),
    |(k, l)| Rule::IffIntro(k, l),
  )(s)
}

fn iff_exclude(s: &str) -> IResult<&str, Rule> {
  map(
    terminated(num, preceded(multispace1, pair(left_right_arrow, tag("E")))),
    |k| Rule::IffExclude(k),
  )(s)
}

fn falsum(s: &str) -> IResult<&str, Rule> {
  map(terminated(num, preceded(multispace1, falsum_tag)), |k| Rule::Falsum(k))(s)
}

fn neg_intro(s: &str) -> IResult<&str, Rule> {
  map(
    terminated(range, preceded(multispace1, pair(negation, tag("I")))),
    |(k, l)| Rule::NegIntro((k, l)),
  )(s)
}

fn neg_exclude(s: &str) -> IResult<&str, Rule> {
  map(
    terminated(range, preceded(multispace1, pair(negation, tag("E")))),
    |(k, l)| Rule::NegExclude((k, l)),
  )(s)
}

fn univ_qunt_intro(s: &str) -> IResult<&str, Rule> {
  map(terminated(num, preceded(multispace1, pair(tag("()"), tag("I")))), |k| {
    Rule::UnivQuntIntro(k)
  })(s)
}

fn univ_qunt_exclude(s: &str) -> IResult<&str, Rule> {
  map(terminated(num, preceded(multispace1, pair(tag("()"), tag("E")))), |k| {
    Rule::UnivQuntExclude(k)
  })(s)
}

fn exis_qunt_intro(s: &str) -> IResult<&str, Rule> {
  map(
    terminated(num, preceded(multispace1, pair(existential, tag("I")))),
    |k| Rule::ExisQuntIntro(k),
  )(s)
}

fn exis_qunt_exclude(s: &str) -> IResult<&str, Rule> {
  map(
    terminated(
      tuple((num, sep, range)),
      preceded(multispace1, pair(existential, tag("E"))),
    ),
    |(k, _, (l, m))| Rule::ExisQuntExclude(k, (l, m)),
  )(s)
}

pub fn rule(s: &str) -> IResult<&str, Rule> {
  alt((
    premise,
    and_intro,
    and_exclude,
    or_intro,
    or_exclude,
    if_intro,
    if_exclude,
    iff_intro,
    iff_exclude,
    falsum,
    neg_intro,
    neg_exclude,
    univ_qunt_intro,
    univ_qunt_exclude,
    exis_qunt_intro,
    exis_qunt_exclude,
  ))(s)
}

#[cfg(test)]
mod tests {
  use super::*;
  use nom::IResult;

  #[test]
  fn rule_valid() {
    assert_eq!(rule("P"), IResult::Ok(("", Rule::Premise)));
    assert_eq!(rule("1,3 &I"), IResult::Ok(("", Rule::AndIntro(1, 3))));
    assert_eq!(rule("5 &E"), IResult::Ok(("", Rule::AndExclude(5))));
    assert_eq!(rule("1 |I"), IResult::Ok(("", Rule::OrIntro(1, None))));
    assert_eq!(rule("1,2 |I"), IResult::Ok(("", Rule::OrIntro(1, Some(2)))));
    assert_eq!(
      rule("1, 3-4, 6-7 |E"),
      IResult::Ok(("", Rule::OrExclude(1, (3, 4), (6, 7))))
    );
    assert_eq!(rule("3 →I"), IResult::Ok(("", Rule::IfIntro((None, 3)))));
    assert_eq!(rule("2-3 →I"), IResult::Ok(("", Rule::IfIntro((Some(2), 3)))));
    assert_eq!(rule("7-14 →I"), IResult::Ok(("", Rule::IfIntro((Some(7), 14)))));
    assert_eq!(rule("1,3 ->E"), IResult::Ok(("", Rule::IfExclude(1, 3))));
    assert_eq!(rule("1,3 <->I"), IResult::Ok(("", Rule::IffIntro(1, 3))));
    assert_eq!(rule("1 <->E"), IResult::Ok(("", Rule::IffExclude(1))));
    assert_eq!(rule("1 \\bot"), IResult::Ok(("", Rule::Falsum(1))));
    assert_eq!(rule("1-2 -I"), IResult::Ok(("", Rule::NegIntro((1, 2)))));
    assert_eq!(rule("1-2 -E"), IResult::Ok(("", Rule::NegExclude((1, 2)))));
    assert_eq!(rule("1 ()I"), IResult::Ok(("", Rule::UnivQuntIntro(1))));
    assert_eq!(rule("1 ()E"), IResult::Ok(("", Rule::UnivQuntExclude(1))));
    assert_eq!(rule("1 ]I"), IResult::Ok(("", Rule::ExisQuntIntro(1))));
    assert_eq!(rule("1, 2-3 ]E"), IResult::Ok(("", Rule::ExisQuntExclude(1, (2, 3)))));
  }
}
