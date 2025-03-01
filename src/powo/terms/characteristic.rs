use super::PowoQuery;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Characteristic {
  Summary,
  Appearance,
  #[allow(clippy::enum_variant_names)]
  Characteristic,
  Flower,
  Fruit,
  Leaf,
  Inflorescence,
  Seed,
  Cloning,
  Use,
}

impl From<Characteristic> for &'static str {
  fn from(val: Characteristic) -> Self {
    match val {
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

impl From<Characteristic> for PowoQuery {
  fn from(val: Characteristic) -> Self {
    PowoQuery::Characteristic(val)
  }
}
