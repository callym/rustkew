use super::IpniQuery;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Name {
  Added,
  Author,
  Basionym,
  BasionymAuthor,
  BibliographicReference,
  CitationType,
  CollectionNumber,
  Collectors,
  Distribution,
  Family,
  FullName,
  Genus,
  InPowo,
  Infrafamily,
  Infragenus,
  Infraspecies,
  Modified,
  NameStatus,
  Published,
  PublishedIn,
  PublishingAuthor,
  Rank,
  ScientificName,
  Species,
  SpeciesAuthor,
  Version,
}

impl From<Name> for &'static str {
  fn from(val: Name) -> Self {
    match val {
      Name::Added => "added",
      Name::Author => "name author",
      Name::Basionym => "basionym",
      Name::BasionymAuthor => "basionym author",
      Name::BibliographicReference => "bibliographic reference",
      Name::CitationType => "citation type",
      Name::CollectionNumber => "collection number",
      Name::Collectors => "collector team",
      Name::Distribution => "distribution",
      Name::Family => "family",
      Name::FullName => "full name",
      Name::Genus => "genus",
      Name::InPowo => "in powo",
      Name::Infrafamily => "infrafamily",
      Name::Infragenus => "infragenus",
      Name::Infraspecies => "infraspecies",
      Name::Modified => "modified",
      Name::NameStatus => "name status",
      Name::Published => "published",
      Name::PublishedIn => "published in",
      Name::PublishingAuthor => "publishing author",
      Name::Rank => "rank",
      Name::ScientificName => "scientific name",
      Name::Species => "species",
      Name::SpeciesAuthor => "species author",
      Name::Version => "version",
    }
  }
}

impl From<Name> for IpniQuery {
  fn from(val: Name) -> Self {
    IpniQuery::Name(val)
  }
}
