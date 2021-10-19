#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Filters {
  Familial,
  Infrafamilial,
  Generic,
  Infrageneric,
  Specific,
  Infraspecific,
}

impl Into<&'static str> for Filters {
  fn into(self) -> &'static str {
    match self {
      Filters::Familial => "f_familial",
      Filters::Infrafamilial => "f_infrafamilial",
      Filters::Generic => "f_generic",
      Filters::Infrageneric => "f_infrageneric",
      Filters::Specific => "f_specific",
      Filters::Infraspecific => "f_infraspecific",
    }
  }
}
