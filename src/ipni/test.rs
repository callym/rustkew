use super::{filters::Filters, terms, Author, Citation, Ipni, IpniResult, Publication};
use crate::Api;

#[async_std::test]
async fn basic_search() {
  let res = Ipni::search("Poa Annua".into()).await.unwrap();

  assert_eq!(res.size(), 13);

  assert!(res.results().iter().any(
    |res| if let IpniResult::Citation(Citation { id, .. }) = res {
      id == "320035-2"
    } else {
      false
    }
  ));
}

#[async_std::test]
async fn advanced_name_search() {
  let query = Ipni::new()
    .query(terms::Name::Genus, "Poa")
    .query(terms::Name::Species, "annua")
    .query(terms::Name::Author, "L.");

  let res = query.run().await.unwrap();

  assert_eq!(res.size(), 1);

  assert!(res.results().iter().any(
    |res| if let IpniResult::Citation(Citation { id, .. }) = res {
      id == "320035-2"
    } else {
      false
    }
  ));
}

#[async_std::test]
async fn advanced_author_search() {
  let res = Ipni::new()
    .query(terms::Author::StandardForm, "L.")
    .run()
    .await
    .unwrap();

  assert!(res
    .results()
    .iter()
    .any(|res| if let IpniResult::Author(Author { id, .. }) = res {
      id == "12653-1"
    } else {
      false
    }));
}

#[async_std::test]
async fn advanced_publication_search() {
  let res = Ipni::new()
    .query(terms::Publication::LcNumber, "QK91.S6")
    .run()
    .await
    .unwrap();

  assert!(res.results().iter().any(|res| {
    if let IpniResult::Publication(Publication { id, .. }) = res {
      id == "1071-2"
    } else {
      false
    }
  }));
}

#[async_std::test]
async fn lookup_name() {
  let res = Ipni::lookup_name("320035-2".into()).await.unwrap();

  assert_eq!(res.name, "Poa annua");
}

#[async_std::test]
async fn lookup_publication() {
  let res = Ipni::lookup_publication("1071-2".into()).await.unwrap();

  assert_eq!(res.title, "Species Plantarum");
}

#[async_std::test]
async fn lookup_author() {
  let res = Ipni::lookup_author("12653-1".into()).await.unwrap();

  assert_eq!(res.standard_form, "L.");
}

#[async_std::test]
async fn filter_by_family() {
  let res = Ipni::new()
    .query(terms::Name::Family, "Poaceae")
    .filter(Filters::Familial)
    .run()
    .await
    .unwrap();

  assert_eq!(res.size(), 1);
}

#[async_std::test]
async fn filter_by_infrafamily() {
  let base = Ipni::new().query(terms::Name::Family, "Poaceae");

  let unfiltered = base.clone().run().await.unwrap();
  let filtered = base
    .clone()
    .filter(Filters::Infrafamilial)
    .run()
    .await
    .unwrap();

  assert!(filtered.size() < unfiltered.size());
}

#[async_std::test]
async fn filter_by_generic() {
  let base = Ipni::new().query(terms::Name::Family, "Poaceae");

  let unfiltered = base.clone().run().await.unwrap();
  let filtered = base.clone().filter(Filters::Generic).run().await.unwrap();

  assert!(filtered.size() < unfiltered.size());
}

#[async_std::test]
async fn filter_by_infrageneric() {
  let base = Ipni::new().query(terms::Name::Family, "Poaceae");

  let unfiltered = base.clone().run().await.unwrap();
  let filtered = base
    .clone()
    .filter(Filters::Infrageneric)
    .run()
    .await
    .unwrap();

  assert!(filtered.size() < unfiltered.size());
}

#[async_std::test]
async fn filter_by_specific() {
  let base = Ipni::new().query(terms::Name::Family, "Poaceae");

  let unfiltered = base.clone().run().await.unwrap();
  let filtered = base.clone().filter(Filters::Specific).run().await.unwrap();

  assert!(filtered.size() < unfiltered.size());
}

#[async_std::test]
async fn filter_by_infraspecific() {
  let base = Ipni::new().query(terms::Name::Family, "Poaceae");

  let unfiltered = base.clone().run().await.unwrap();
  let filtered = base
    .clone()
    .filter(Filters::Infraspecific)
    .run()
    .await
    .unwrap();

  assert!(filtered.size() < unfiltered.size());
}
