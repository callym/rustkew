use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use urn::Urn;

use crate::Error;

pub type Id = Urn;

use crate::{
  Api,
  SearchResponse,
  core::{SearchQuery, build_params, get},
};

mod filters;
mod terms;
pub use terms::{Name, PowoQuery};

use self::filters::Filters;

#[cfg(test)]
mod test;

#[derive(Debug, Clone)]
pub struct Powo {
  query: Option<Vec<(PowoQuery, String)>>,
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
    let query = Some(SearchQuery::<<Self as Api>::Query>::String(query));
    let params = build_params(&query, &None::<Vec<String>>, "*");
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Distribution {
  pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DistributionMap {
  pub natives: Vec<Distribution>,
  pub introduced: Option<Vec<Distribution>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Image {
  pub thumbnail: String,
  pub fullsize: String,
  pub caption: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Synonym {
  #[serde(rename(deserialize = "fqId"))]
  pub fq_id: Urn,
  pub url: String,
  pub name: String,
  pub accepted: bool,
  pub author: Option<String>,
  pub kingdom: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PowoResult {
  pub accepted: bool,
  pub author: Option<String>,
  pub kingdom: String,
  pub family: String,
  pub name: String,
  pub rank: String,
  pub snippet: Option<String>,
  #[serde(rename(deserialize = "synonymOf"))]
  pub synonym_of: Option<Synonym>,
  pub url: String,
  #[serde(rename(deserialize = "fqId"))]
  pub fq_id: Urn,

  #[serde(default)]
  pub images: Vec<Image>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Taxon {
  #[serde(rename(deserialize = "fqId"))]
  pub fq_id: Urn,
  pub name: String,
  pub author: String,
  pub rank: String,
  #[serde(rename(deserialize = "taxonomicStatus"))]
  pub taxonomic_status: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Description {
  pub description: String,
  pub source: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Descriptions {
  #[serde(rename(deserialize = "asTaxon"))]
  pub as_taxon: String,
  pub source: String,
  #[serde(rename(deserialize = "fromSynonym"))]
  pub from_synonym: bool,
  pub descriptions: HashMap<String, Vec<Description>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PowoLookup {
  pub modified: Option<String>,
  #[serde(rename(deserialize = "bibliographicCitation"))]
  pub bibliographic_citation: String,
  pub genus: String,
  #[serde(rename(deserialize = "taxonomicStatus"))]
  pub taxonomic_status: String,
  pub kingdom: String,
  pub phylum: String,
  #[serde(rename(deserialize = "clazz"))]
  pub class: String,
  pub subclass: String,
  pub order: String,
  pub family: String,
  #[serde(rename(deserialize = "nomenclaturalCode"))]
  pub nomenclatural_code: String,
  pub source: String,
  #[serde(rename(deserialize = "namePublishedInYear"))]
  pub name_published_in_year: Option<u32>,
  #[serde(rename(deserialize = "taxonRemarks"))]
  pub taxon_remarks: Option<String>,
  #[serde(rename(deserialize = "nomenclaturalStatus"))]
  pub nomenclatural_status: String,
  pub lifeform: String,
  pub climate: String,
  pub hybrid: bool,
  pub accepted: Option<Taxon>,
  #[serde(rename(deserialize = "paftolId"))]
  pub paftol_id: Option<String>,
  pub synonym: bool,
  pub plantae: bool,
  pub fungi: bool,
  #[serde(rename(deserialize = "fqId"))]
  pub fq_id: Urn,
  pub name: String,
  pub authors: Option<String>,
  pub species: String,
  pub infraspecies: Option<String>,
  pub rank: String,
  pub reference: Option<String>,

  pub classification: Vec<Taxon>,
  #[serde(rename(deserialize = "basionymOf"))]
  pub basionym_of: Option<Vec<Taxon>>,
  pub basionym: Option<Taxon>,
  pub synonyms: Option<Vec<Taxon>>,

  pub distribution: Option<DistributionMap>,
  #[serde(rename(deserialize = "distributionEnvelope"))]
  pub distribution_envelope: Option<Vec<DistributionEnvelopeEntry>>,
  pub descriptions: Option<HashMap<String, Descriptions>>,

  pub locations: Option<Vec<String>>,
}
