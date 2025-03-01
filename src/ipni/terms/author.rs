use super::IpniQuery;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Author {
  Forename,
  FullName,
  StandardForm,
  Surname,
}

impl From<Author> for &'static str {
  fn from(val: Author) -> Self {
    match val {
      Author::Forename => "author forename",
      Author::FullName => "author name",
      Author::StandardForm => "author std",
      Author::Surname => "author surname",
    }
  }
}

impl From<Author> for IpniQuery {
  fn from(val: Author) -> Self {
    IpniQuery::Author(val)
  }
}
