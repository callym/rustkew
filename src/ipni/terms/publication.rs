use super::IpniQuery;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Publication {
  StandardForm,
  BphNumber,
  Date,
  Isbn,
  Issn,
  LcNumber,
  PrecededBy,
  SupercededBy,
  Title,
  Tl2Author,
  Tl2Number,
}

impl Into<&'static str> for Publication {
  fn into(self) -> &'static str {
    match self {
      Publication::StandardForm => "publication std",
      Publication::BphNumber => "bph number",
      Publication::Date => "date",
      Publication::Isbn => "isbn",
      Publication::Issn => "issn",
      Publication::LcNumber => "lc number",
      Publication::PrecededBy => "preceded by",
      Publication::SupercededBy => "superceded by",
      Publication::Title => "publication title",
      Publication::Tl2Author => "tl2 author",
      Publication::Tl2Number => "tl2 number",
    }
  }
}

impl Into<IpniQuery> for Publication {
  fn into(self) -> IpniQuery {
    IpniQuery::Publication(self)
  }
}
