use serde::Deserialize;

use crate::core::get;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Suggestion {
  pub term: String,
  pub weight: f32,
  pub payload: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Suggestions {
  pub scientific_name: Vec<Suggestion>,
  pub author: Vec<Suggestion>,
  pub publication: Vec<Suggestion>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SuggestedTerms {
  pub scientific_name: Vec<String>,
  pub author: Vec<String>,
  pub publication: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuggestResult {
  pub suggestions: Suggestions,
  pub suggested_terms: SuggestedTerms,
}

pub async fn suggest(query: String) -> Result<SuggestResult, surf::Error> {
  get(
    crate::core::IPNI_URL,
    "suggest",
    [("query".to_owned(), query)].into_iter(),
  )
  .await
}
