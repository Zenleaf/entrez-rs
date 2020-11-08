pub mod parser;

use std::fmt::{Display, Formatter};
use parser::*;
use quick_xml::de::{from_str, DeError};


const BASE: &str = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/";
    
#[derive(Debug)]
pub enum DB {
    Pubmed
}

impl Display for DB {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DB::Pubmed => write!(f, "{}", "pubmed")
        }
    }
}

pub enum Eutil<'a> {
        ESearch{db: DB, query: &'a str },
        EFetch {db: DB, id_list: Vec<String> }
}

impl Eutil<'_> {
        pub fn url(&self) -> String {
            match self {
                Eutil::ESearch { db, query } => {
                format!("{}esearch.fcgi?db={}&term={}&usehistory=y&retmax=16", 
                BASE,
                db,
                query)
            },
                Eutil::EFetch { db, id_list } => {
                format!("{}efetch.fcgi?db={}&id={}&retmode=xml",
                BASE,
                db,
                id_list.join(","))
                }
            }
        }
    
}


