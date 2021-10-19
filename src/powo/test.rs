use super::{filters::Filters, terms, Powo};
use crate::Api;

#[async_std::test]
async fn basic_search() {
  let res = Powo::search("Poa Annua".into()).await.unwrap();

  assert_eq!(res.size(), 3);

  assert_eq!(res.results()[0].fq_id, "urn:lsid:ipni.org:names:320035-2");
}

#[async_std::test]
async fn advanced_name_search() {
  let query = Powo::new()
    .query(terms::Name::Genus, "Poa")
    .query(terms::Name::Species, "annua")
    .query(terms::Name::Author, "L.");

  let res = query.run().await.unwrap();

  assert_eq!(res.size(), 1);

  assert_eq!(res.results()[0].fq_id, "urn:lsid:ipni.org:names:320035-2");
}

#[async_std::test]
async fn advanced_characteristic_search() {
  let res = Powo::new()
    .query(terms::Characteristic::Flower, "yellow")
    .query(terms::Characteristic::Leaf, "alternate")
    .run()
    .await
    .unwrap();

  assert!(res.size() > 0);
}

#[async_std::test]
async fn advanced_geography_search() {
  let res = Powo::new()
    .query(terms::Geography::Distribution, "Africa")
    .run()
    .await
    .unwrap();

  assert!(res.size() > 0);
}

#[async_std::test]
async fn lookup() {
  let res = Powo::lookup("urn:lsid:ipni.org:names:320035-2".into(), None)
    .await
    .unwrap();

  assert_eq!(res.name, "Poa annua");
}

#[async_std::test]
async fn lookup_with_extra_fields() {
  let res = Powo::lookup(
    "urn:lsid:ipni.org:names:320035-2".into(),
    Some(vec!["distribution".into(), "descriptions".into()]),
  )
  .await
  .unwrap();

  assert_eq!(res.name, "Poa annua");
  assert_eq!(res.distribution.unwrap().natives[0].name, "Afghanistan");
  assert!(res.descriptions.is_some());
}

#[async_std::test]
async fn filters() {
  let query = Powo::new().query(terms::Name::Family, "Poaceae");

  let unfiltered = query.clone().run().await.unwrap();
  let filtered = query.clone().filter(Filters::Accepted).run().await.unwrap();

  assert!(filtered.size() < unfiltered.size());
}
