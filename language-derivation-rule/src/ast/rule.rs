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

impl Rule {
  pub fn to_string(&self) -> String {
    match self {
      Rule::Premise => format!("P"),
      Rule::AndIntro(dep1, dep2) => format!("{}, {} &I", dep1, dep2),
      Rule::AndExclude(dep1) => format!("{} &E", dep1),
      Rule::OrIntro(dep1, Some(dep2)) => format!("{}, {} ∨I", dep1, dep2),
      Rule::OrIntro(dep1, None) => format!("{} ∨I", dep1),
      Rule::OrExclude(dep1, (dep2b, dep2e), (dep3b, dep3e)) => {
        format!("{}, {}-{}, {}-{} ∨E", dep1, dep2b, dep2e, dep3b, dep3e)
      }
      Rule::IfIntro((Some(dep1b), dep1e)) => format!("{}-{} →I", dep1b, dep1e),
      Rule::IfIntro((None, dep1)) => format!("{} →I", dep1),
      Rule::IfExclude(dep1, dep2) => format!("{}, {} →E", dep1, dep2),
      Rule::IffIntro(dep1, dep2) => format!("{}, {} ↔I", dep1, dep2),
      Rule::IffExclude(dep1) => format!("{} ↔E", dep1),
      Rule::Falsum(dep1) => format!("{} ⊥", dep1),
      Rule::NegIntro((dep1b, dep1e)) => format!("{}-{} ¬I", dep1b, dep1e),
      Rule::NegExclude((dep1b, dep1e)) => format!("{}-{} ¬E", dep1b, dep1e),
      Rule::UnivQuntIntro(dep1) => format!("{} ()I", dep1),
      Rule::UnivQuntExclude(dep1) => format!("{} ()E", dep1),
      Rule::ExisQuntIntro(dep1) => format!("{} ∃I", dep1),
      Rule::ExisQuntExclude(dep1, (dep2b, dep2e)) => format!("{}, {}-{} ∃E", dep1, dep2b, dep2e),
    }
  }
}
