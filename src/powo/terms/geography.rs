use super::PowoQuery;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Geography {
  Distribution,
}

impl Into<&'static str> for Geography {
  fn into(self) -> &'static str {
    match self {
      Geography::Distribution => "location",
    }
  }
}

impl Into<PowoQuery> for Geography {
  fn into(self) -> PowoQuery {
    PowoQuery::Geography(self)
  }
}
