pub(crate) mod core;

macro_rules! impl_api {
  ($ty:ty, URL: $url:path, Filters: $filters:path, Ok: $ok:path, Query: $query:path) => {
    #[async_trait::async_trait]
    impl Api for $ty {
      type Filters = $filters;
      type Ok = $ok;
      type Query = $query;

      const URL: &'static str = $url;

      fn new() -> Self {
        Self {
          query: None,
          filters: None,
          cursor: None,
        }
      }

      fn query(mut self, key: impl Into<Self::Query>, value: impl Into<String>) -> Self {
        let mut query = self.query.unwrap_or_default();
        query.insert(key.into(), value.into());
        self.query = Some(query);

        self
      }

      fn filter(mut self, filter: Filters) -> Self {
        let mut filters = self.filters.unwrap_or_default();
        filters.push(filter);
        self.filters = Some(filters);

        self
      }

      fn cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);

        self
      }

      async fn run(&self) -> Result<SearchResponse<Self::Ok>, Error> {
        let query = self.query.clone().map(|q| SearchQuery::Map(q));
        let filters = self.filters.clone().map(|f| {
          f.into_iter()
            .map(|f| {
              let f: &'static str = f.into();
              String::from(f)
            })
            .collect()
        });
        let cursor = self.cursor.as_deref().unwrap_or("*");
        let params = build_params(&query, &filters, &cursor);

        get(Self::URL, "search", params).await
      }
    }
  };
}

pub mod ipni;
pub mod powo;

use serde::{de::DeserializeOwned, Deserialize};
pub use surf::Error;

#[async_trait::async_trait]
pub trait Api {
  const URL: &'static str;
  type Ok: DeserializeOwned;
  type Query;
  type Filters;

  fn new() -> Self;
  fn query(self, key: impl Into<Self::Query>, value: impl Into<String>) -> Self;
  fn filter(self, filter: Self::Filters) -> Self;
  fn cursor(self, cursor: String) -> Self;
  async fn run(&self) -> Result<SearchResponse<Self::Ok>, Error>;
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse<R> {
  #[serde(rename = "totalResults")]
  total_results: Option<i32>,
  #[serde(default = "crate::default_cursor")]
  cursor: String,
  #[serde(default = "crate::default_results")]
  results: Vec<R>,
}

impl<R> SearchResponse<R> {
  pub fn cursor(&self) -> &str {
    &self.cursor
  }

  pub fn size(&self) -> i32 {
    self.total_results.unwrap_or(0)
  }

  pub fn results(&self) -> &[R] {
    &self.results
  }
}

fn default_cursor() -> String {
  "*".into()
}

fn default_results<R>() -> Vec<R> {
  Vec::new()
}
