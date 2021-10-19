use crate::core::ToKey;
mod characteristic;
mod geography;
mod name;
pub use characteristic::Characteristic;
pub use geography::Geography;
pub use name::Name;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PowoQuery {
  Characteristic(Characteristic),
  Geography(Geography),
  Name(Name),
}

impl Into<&'static str> for PowoQuery {
  fn into(self) -> &'static str {
    match self {
      PowoQuery::Characteristic(characteristic) => characteristic.into(),
      PowoQuery::Geography(geography) => geography.into(),
      PowoQuery::Name(name) => name.into(),
    }
  }
}

impl ToKey for PowoQuery {
  fn to_key(&self) -> &'static str {
    (*self).into()
  }
}
