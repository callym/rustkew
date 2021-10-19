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
pub use terms::{Name, PowoQuery};

use self::filters::Filters;

#[cfg(test)]
mod test;

#[derive(Clone)]
pub struct Powo {
  query: Option<HashMap<PowoQuery, String>>,
  filters: Option<Vec<Filters>>,
  cursor: Option<String>,
}

impl_api!(
  Powo,
  URL: crate::core::POWO_URL,
  Filters: Filters,
  Ok: PowoResult,
  Query: PowoQuery
);

impl Powo {
  pub async fn search(query: String) -> Result<SearchResponse<<Self as Api>::Ok>, Error> {
    let params = build_params(
      &Some(SearchQuery::<<Self as Api>::Query>::String(query)),
      &None::<Vec<String>>,
      "*",
    );
    get(Self::URL, "search", params).await
  }

  pub async fn lookup(id: String, include: Option<Vec<String>>) -> Result<PowoResult, Error> {
    let params = if let Some(include) = include {
      vec![("fields".into(), include.join(","))]
    } else {
      vec![]
    };

    get(Self::URL, format!("taxon/{}", id), params.into_iter()).await
  }
}

#[derive(Deserialize)]
pub struct Distribution {
  pub name: String,
}

#[derive(Deserialize)]
pub struct DistributionMap {
  pub natives: Vec<Distribution>,
  pub introduced: Vec<Distribution>,
}

#[derive(Deserialize)]
pub struct PowoResult {
  #[serde(rename = "fqId")]
  pub fq_id: String,
  pub name: String,
  pub distribution: Option<DistributionMap>,
  pub descriptions: Option<serde_json::Value>,
}
