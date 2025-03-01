use super::PowoQuery;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Geography {
  Distribution,
}

impl From<Geography> for &'static str {
  fn from(val: Geography) -> Self {
    match val {
      Geography::Distribution => "location",
    }
  }
}

impl From<Geography> for PowoQuery {
  fn from(val: Geography) -> Self {
    PowoQuery::Geography(val)
  }
}
