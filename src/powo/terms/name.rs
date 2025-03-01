use super::PowoQuery;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Name {
  FullName,
  CommonName,
  Kingdom,
  Family,
  Genus,
  Species,
  Author,
}

impl From<Name> for &'static str {
  fn from(val: Name) -> Self {
    match val {
      Name::FullName => "name",
      Name::CommonName => "common name",
      Name::Kingdom => "kingdom",
      Name::Family => "family",
      Name::Genus => "genus",
      Name::Species => "species",
      Name::Author => "author",
    }
  }
}

impl From<Name> for PowoQuery {
  fn from(val: Name) -> Self {
    PowoQuery::Name(val)
  }
}
