extern crate serde;
extern crate quick_xml;
use serde::{Deserialize};
use quick_xml::de::{from_str, DeError};


#[derive(Debug, Deserialize, PartialEq)]
pub struct IdList {
    #[serde(rename = "Id", default)]
    pub id: Vec<String>
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
