use super::IpniQuery;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Author {
  Forename,
  FullName,
  StandardForm,
  Surname,
}

impl Into<&'static str> for Author {
  fn into(self) -> &'static str {
    match self {
      Author::Forename => "author forename",
      Author::FullName => "author name",
      Author::StandardForm => "author std",
      Author::Surname => "author surname",
    }
  }
}

impl Into<IpniQuery> for Author {
  fn into(self) -> IpniQuery {
    IpniQuery::Author(self)
  }
}
