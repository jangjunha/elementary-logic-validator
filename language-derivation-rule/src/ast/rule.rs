#[derive(Debug, Clone, PartialEq)]
pub enum Rule {
  Premise,
  AndIntro(usize, usize),
  AndExclude(usize),
  OrIntro(usize, Option<usize>),
  OrExclude(usize, (usize, usize), (usize, usize)),
  IfIntro((Option<usize>, usize)),
  IfExclude(usize, usize),
  IffIntro(usize, usize),
  IffExclude(usize),
  Falsum(usize),
  NegIntro((usize, usize)),
  NegExclude((usize, usize)),
  UnivQuntIntro(usize),
  UnivQuntExclude(usize),
  ExisQuntIntro(usize),
  ExisQuntExclude(usize, (usize, usize)),
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
