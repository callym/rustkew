#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Filters {
  Accepted,
  HasImages,
  Families,
  Genera,
  Species,
  Infraspecies,
}

impl Into<&'static str> for Filters {
  fn into(self) -> &'static str {
    match self {
      Filters::Accepted => "accepted_names",
      Filters::HasImages => "has_images",
      Filters::Families => "families_f",
      Filters::Genera => "genus_f",
      Filters::Species => "species_f",
      Filters::Infraspecies => "infraspecific_f",
    }
  }
}
