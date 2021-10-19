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

impl Into<&'static str> for Name {
  fn into(self) -> &'static str {
    match self {
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

impl Into<PowoQuery> for Name {
  fn into(self) -> PowoQuery {
    PowoQuery::Name(self)
  }
}
