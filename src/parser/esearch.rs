//! Utilities to parse XML results obtained using [ESearch](super::super::eutils::ESearch).
 
//! # Description
//! When using [ESearch](super::super::eutils::ESearch),
//! the resulting XML
//! is as follows:
//! ``` xml
//! <?xml version="1.0" encoding="UTF-8" ?>
//! <!DOCTYPE eSearchResult PUBLIC "-//NLM//DTD esearch 20060628//EN" "https://eutils.ncbi.nlm.nih.gov/eutils/dtd/20060628/esearch.dtd">
//! <eSearchResult>
//!    ...
//! </eSearchResult>
//!  ```
//! The [read](ESearchResult::read) function will parse this XML into an [ESearchResult]
//! struct.
//! 
//! # Example
//! 
//!  ```
//!   use entrez_rs::eutils::{Eutils, ESearch, DB};
//!   use entrez_rs::parser::esearch::{ESearchResult};
//! 
//!   let xml = ESearch::new(
//!         DB::Pubmed, 
//!         "sars_cov_2") 
//!         .run().expect("Connection error");
//! 
//!   // Use the read function to parse the xml result.
//!   let parsed = ESearchResult::read(&xml)
//!                .expect("Parsing error");
//! 
//!   assert_eq!(parsed.id_list.ids.len(), 20);
//!   ```


extern crate serde;
extern crate quick_xml;
use serde::{Deserialize};
use quick_xml::de::{from_str, DeError};


#[derive(Debug, Deserialize, PartialEq)]
pub struct IdList {
    #[serde(rename = "Id", default)]
    pub ids: Vec<String>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Translation {
    #[serde(rename = "From", default)]
    from: String,
    #[serde(rename = "To", default)]
    to: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TranslationSet {
    #[serde(rename = "Translation")]
    translation: Vec<Translation>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TranslationStack {
    #[serde(rename = "TermSet")]
    term_set: Vec<TermSet>,
    #[serde(rename = "OP")]
    op: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TermSet {
    #[serde(rename = "Term", default)]
    term: String,
    #[serde(rename = "Field", default)]
    field: String,
    #[serde(rename = "Count", default)]
    count: u64,
    #[serde(rename = "Explode", default)]
    explode: String,
}


#[derive(Debug, Deserialize, PartialEq)]
pub struct ESearchResult {
    #[serde(rename = "Count", default)]
    pub count: u64,
    #[serde(rename = "RetMax", default)]
    ret_max: u64,
    #[serde(rename = "RetStart", default)]
    ret_start: u64,
    #[serde(rename = "QueryKey", default)]
    query_key: u64,
    #[serde(rename = "WebEnv", default)]
    web_env: String,
    #[serde(rename = "IdList")]
    pub id_list: IdList,
   // #[serde(rename = "TranslationSet")]
   // translation_set: TranslationSet,
  //  #[serde(rename = "TranslationStack")]
  //  translation_stack: TranslationStack,
    #[serde(rename = "QueryTranslation", default)]
    query_translation: String,
}

impl ESearchResult {
    pub fn read(xml: &str) -> Result<ESearchResult, DeError> {
        from_str(xml)
    }
}
