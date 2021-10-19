use std::{collections::HashMap, time::Duration};

use serde::de::DeserializeOwned;
use surf::StatusCode;

pub(crate) const IPNI_URL: &'static str = "https://beta.ipni.org/api/1";
pub(crate) const POWO_URL: &'static str = "http://www.plantsoftheworldonline.org/api/2";

pub(crate) fn build_params<K: ToKey>(
  query: &Option<SearchQuery<K>>,
  filters: &Option<Vec<String>>,
  cursor: &str,
) -> impl Iterator<Item = (String, String)> {
  let mut params = HashMap::new();
  params.insert(String::from("perPage"), String::from("500"));
  params.insert(String::from("cursor"), cursor.to_string());

  if let Some(query) = query {
    params.insert(String::from("q"), query.format());
  }

  if let Some(filters) = filters {
    params.insert(String::from("f"), filters.join(","));
  }

  params.into_iter()
}

pub(crate) async fn get<R: DeserializeOwned>(
  base_url: &'static str,
  method: impl Into<String>,
  params: impl Iterator<Item = (String, String)>,
) -> Result<R, surf::Error> {
  let url = format!("{}/{}", base_url, method.into());
  let url = surf::Url::parse_with_params(&url, params)?;

  dbg!(url.to_string());

  let mut res = loop {
    match surf::get(&url).await {
      Ok(res) => break res,
      Err(err) if err.status() == StatusCode::TooManyRequests => {
        async_std::task::sleep(Duration::from_millis(500)).await;
        continue;
      },
      Err(err) => Err(err)?,
    }
  };

  Ok(res.body_json().await?)
}

pub trait ToKey {
  fn to_key(&self) -> &'static str;
}

pub enum SearchQuery<K: ToKey> {
  String(String),
  Map(HashMap<K, String>),
}

impl<K: ToKey> SearchQuery<K> {
  pub fn format(&self) -> String {
    match self {
      SearchQuery::String(s) => s.clone(),
      SearchQuery::Map(map) => map
        .iter()
        .map(|(k, v)| format!("{}:{}", k.to_key(), v))
        .collect::<Vec<_>>()
        .join(","),
    }
  }
}
