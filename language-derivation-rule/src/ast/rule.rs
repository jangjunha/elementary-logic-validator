#[derive(Debug, Clone, PartialEq)]
pub enum Rule {
  Premise,
  AndIntro(i32, i32),
  AndExclude(i32),
  OrIntro(i32, Option<i32>),
  OrExclude(i32, (i32, i32), (i32, i32)),
  IfIntro((Option<i32>, i32)),
  IfExclude(i32, i32),
  IffIntro(i32, i32),
  IffExclude(i32),
  Falsum(i32),
  NegIntro((i32, i32)),
  NegExclude((i32, i32)),
  UnivQuntIntro(i32),
  UnivQuntExclude(i32),
  ExisQuntIntro(i32),
  ExisQuntExclude(i32, (i32, i32)),
}
