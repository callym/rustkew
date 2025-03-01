use crate::core::ToKey;
mod author;
mod name;
mod publication;
pub use author::Author;
pub use name::Name;
pub use publication::Publication;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum IpniQuery {
  Author(Author),
  Name(Name),
  Publication(Publication),
}

impl From<IpniQuery> for &'static str {
  fn from(val: IpniQuery) -> Self {
    match val {
      IpniQuery::Author(author) => author.into(),
      IpniQuery::Name(name) => name.into(),
      IpniQuery::Publication(publication) => publication.into(),
    }
  }
}

impl ToKey for IpniQuery {
  fn to_key(&self) -> &'static str {
    (*self).into()
  }
}
