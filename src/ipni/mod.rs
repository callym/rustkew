use std::collections::HashMap;

use serde::Deserialize;
use surf::Error;

use crate::{
  core::{build_params, get, SearchQuery},
  Api,
  SearchResponse,
};

mod filters;
mod terms;
pub use terms::{IpniQuery, Name};

use self::filters::Filters;

#[cfg(test)]
mod test;

#[derive(Clone)]
pub struct Ipni {
  query: Option<HashMap<IpniQuery, String>>,
  filters: Option<Vec<Filters>>,
  cursor: Option<String>,
}

impl_api!(
  Ipni,
  URL: crate::core::IPNI_URL,
  Filters: Filters,
  Ok: IpniResult,
  Query: IpniQuery
);

impl Ipni {
  pub async fn search(query: String) -> Result<SearchResponse<<Self as Api>::Ok>, Error> {
    let params = build_params(
      &Some(SearchQuery::<<Self as Api>::Query>::String(query)),
      &None::<Vec<String>>,
      "*",
    );
    get(Self::URL, "search", params).await
  }

  pub async fn lookup_name(id: String) -> Result<Citation, Error> {
    get(Self::URL, format!("n/{}", id), [].into_iter()).await
  }

  pub async fn lookup_publication(id: String) -> Result<Publication, Error> {
    get(Self::URL, format!("p/{}", id), [].into_iter()).await
  }

  pub async fn lookup_author(id: String) -> Result<Author, Error> {
    get(Self::URL, format!("a/{}", id), [].into_iter()).await
  }
}

#[derive(Deserialize)]
pub struct Citation {
  pub name: String,
  pub id: String,
}

#[derive(Deserialize)]
pub struct Author {
  pub id: String,
  #[serde(rename = "standardForm")]
  pub standard_form: String,
}

#[derive(Deserialize)]
#[serde(tag = "recordType")]
pub struct Publication {
  pub id: String,
  pub title: String,
}

#[derive(Deserialize)]
#[serde(tag = "recordType")]
pub enum IpniResult {
  #[serde(rename = "citation")]
  Citation(Citation),
  #[serde(rename = "author")]
  Author(Author),
  #[serde(rename = "publication")]
  Publication(Publication),
}
