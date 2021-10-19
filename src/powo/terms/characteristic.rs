use super::PowoQuery;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Characteristic {
  Summary,
  Appearance,
  Characteristic,
  Flower,
  Fruit,
  Leaf,
  Inflorescence,
  Seed,
  Cloning,
  Use,
}

impl Into<&'static str> for Characteristic {
  fn into(self) -> &'static str {
    match self {
      Characteristic::Summary => "summary",
      Characteristic::Appearance => "appearance",
      Characteristic::Characteristic => "characteristic",
      Characteristic::Flower => "flower",
      Characteristic::Fruit => "fruit",
      Characteristic::Leaf => "leaf",
      Characteristic::Inflorescence => "inflorescence",
      Characteristic::Seed => "seed",
      Characteristic::Cloning => "cloning",
      Characteristic::Use => "use",
    }
  }
}

impl Into<PowoQuery> for Characteristic {
  fn into(self) -> PowoQuery {
    PowoQuery::Characteristic(self)
  }
}
