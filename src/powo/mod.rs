use std::collections::HashMap;

use serde::Deserialize;
use surf::Error;
use urn::Urn;

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

#[derive(Debug, Clone)]
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

  pub async fn lookup(id: Urn, include: Option<Vec<String>>) -> Result<PowoLookup, Error> {
    let params = if let Some(include) = include {
      vec![("fields".into(), include.join(","))]
    } else {
      vec![]
    };

    get(Self::URL, format!("taxon/{}", id), params.into_iter()).await
  }
}

#[derive(Debug, Deserialize)]
pub struct Distribution {
  pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct DistributionMap {
  pub natives: Vec<Distribution>,
  pub introduced: Vec<Distribution>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Image {
  pub thumbnail: String,
  pub fullsize: String,
  pub caption: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Synonym {
  #[serde(rename = "fqId")]
  pub fq_id: Urn,
  pub url: String,
  pub name: String,
  pub accepted: bool,
  pub author: Option<String>,
  pub kingdom: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PowoResult {
  pub accepted: bool,
  pub author: Option<String>,
  pub kingdom: String,
  pub family: String,
  pub name: String,
  pub rank: String,
  pub snippet: Option<String>,
  #[serde(rename = "synonymOf")]
  pub synonym_of: Option<Synonym>,
  pub url: String,
  #[serde(rename = "fqId")]
  pub fq_id: Urn,

  #[serde(default)]
  pub images: Vec<Image>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Taxon {
  #[serde(rename = "fqId")]
  pub fq_id: Urn,
  pub name: String,
  pub author: String,
  pub rank: String,
  #[serde(rename = "taxonomicStatus")]
  pub taxonomic_status: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Description {
  pub description: String,
  pub source: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Descriptions {
  #[serde(rename = "asTaxon")]
  pub as_taxon: String,
  pub source: String,
  #[serde(rename = "fromSynonym")]
  pub from_synonym: bool,
  pub descriptions: HashMap<String, Vec<Description>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DistributionEnvelopeEntry {
  #[serde(deserialize_with = "json_float")]
  pub x: f64,
  #[serde(deserialize_with = "json_float")]
  pub y: f64,
  #[serde(deserialize_with = "json_float")]
  pub z: f64,
}

fn json_float<'de, D>(de: D) -> Result<f64, D::Error>
where
  D: serde::Deserializer<'de>,
{
  let f = match f64::deserialize(de) {
    Ok(f) => f,
    Err(e) => {
      if e
        .to_string()
        .starts_with(r#"invalid type: string "NaN", expected f64"#)
      {
        f64::NAN
      } else {
        Err(e)?
      }
    },
  };

  Ok(f)
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PowoLookup {
  pub modified: String,
  #[serde(rename = "bibliographicCitation")]
  pub bibliographic_citation: String,
  pub genus: String,
  #[serde(rename = "taxonomicStatus")]
  pub taxonomic_status: String,
  pub kingdom: String,
  pub phylum: String,
  pub family: String,
  #[serde(rename = "nomenclaturalCode")]
  pub nomenclatural_code: String,
  pub source: String,
  #[serde(rename = "namePublishedInYear")]
  pub name_published_in_year: u32,
  #[serde(rename = "taxonRemarks")]
  pub taxon_remarks: String,
  #[serde(rename = "nomenclaturalStatus")]
  pub nomenclatural_status: String,
  pub synonym: bool,
  pub plantae: bool,
  pub fungi: bool,
  #[serde(rename = "fqId")]
  pub fq_id: Urn,
  pub name: String,
  pub authors: String,
  pub species: String,
  pub rank: String,
  pub reference: String,

  pub classification: Vec<Taxon>,
  #[serde(rename = "basionymOf")]
  pub basionym_of: Vec<Taxon>,
  pub synonyms: Vec<Taxon>,

  pub distribution: Option<DistributionMap>,
  #[serde(rename = "distributionEnvelope")]
  pub distribution_envelope: Option<Vec<DistributionEnvelopeEntry>>,
  pub descriptions: Option<HashMap<String, Descriptions>>,

  pub locations: Option<Vec<String>>,
}
