use std::time::Duration;

use reqwest::StatusCode;
use serde::de::DeserializeOwned;

pub(crate) const IPNI_URL: &str = "https://beta.ipni.org/api/1";
pub(crate) const POWO_URL: &str = "https://powo.science.kew.org/api/2";

pub(crate) fn build_params<K: ToKey>(
  query: &Option<SearchQuery<K>>,
  filters: &Option<Vec<String>>,
  cursor: &str,
) -> impl Iterator<Item = (String, String)> {
  let mut params = vec![];
  params.push((String::from("perPage"), String::from("500")));
  params.push((String::from("cursor"), cursor.to_string()));

  if let Some(query) = query {
    params.push((String::from("q"), query.format()));
  }

  if let Some(filters) = filters {
    params.push((String::from("f"), filters.join(",")));
  }

  params.into_iter()
}

pub(crate) async fn get<R: DeserializeOwned>(
  base_url: &'static str,
  method: impl Into<String>,
  params: impl Iterator<Item = (String, String)>,
) -> Result<R, crate::Error> {
  let url = format!("{}/{}", base_url, method.into());
  let url = reqwest::Url::parse_with_params(&url, params)?;

  let res = loop {
    match reqwest::get(url.clone()).await {
      Ok(res) => break res,
      Err(err) if err.status() == Some(StatusCode::TOO_MANY_REQUESTS) => {
        tokio::time::sleep(Duration::from_millis(500)).await;
        continue;
      },
      Err(err) => Err(err)?,
    }
  };

  Ok(res.json().await?)
}

pub trait ToKey: Sized {
  fn to_key(&self) -> &'static str;
}

pub enum SearchQuery<K: ToKey> {
  String(String),
  Vec(Vec<(K, String)>),
}

impl<K: ToKey> SearchQuery<K> {
  pub fn format(&self) -> String {
    match self {
      SearchQuery::String(s) => s.clone(),
      SearchQuery::Vec(vec) => vec
        .iter()
        .map(|(k, v)| format!("{}:{}", k.to_key(), v))
        .collect::<Vec<_>>()
        .join(","),
    }
  }
}
