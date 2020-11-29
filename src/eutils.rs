
use std::fmt::{Display, Formatter};
use std::default::Default;

const BASE: &str = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/";
    
#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum XMLResult{
    ESearchResultXML(String),
    PubmedArticleSetXML(String)
}

pub trait Eutils {
    fn build_url(&self) -> String;

    fn run(&self) -> Result<String, reqwest::Error>;
    

}

#[derive(Debug, PartialEq)]
pub struct ESearch<'a> {
            // required
            pub db: DB,
            // required 
            pub term: &'a str,
            pub use_history: bool,
            pub webenv: Option<&'a str>,
            pub query_key: Option<&'a str>,
            pub retstart: Option<&'a str>,
            pub retmax: Option<&'a str>,
            pub rettype: Option<&'a str>,
            pub retmode: Option<&'a str>,
            pub sort: Option<&'a str>,
            pub field: Option<&'a str>,
            pub idtype: Option<&'a str>,
            pub datetype: Option<&'a str>,
            pub reldate: Option<&'a str>,
            pub mindate_maxdate: Option<(&'a str,&'a str)>
}


impl<'a> Default for ESearch<'a> {
    fn default() -> Self {
        ESearch {
            db: DB::Pubmed,
            term: "",
            use_history: true,
            webenv: None,
            query_key: None,
            retstart: None,
            retmax: None,
            retmode: Some("xml"),
            rettype: Some("xml"),
            sort: None,
            field: None,
            idtype: None,
            datetype: None,
            reldate: None,
            mindate_maxdate: None
        }
    }
}

impl<'a> ESearch<'a> {
    pub fn new(db: DB, term: &'a str) -> Self {
        ESearch {
            db, 
            term,
            .. Default::default()
        }
    }
}


impl<'a> Eutils for ESearch<'a> {
    fn build_url(&self) -> String {
        let mut url_str = format!("{}esearch.fcgi?", BASE);
        url_str.push_str(&(format!("db={}", self.db)));
        url_str.push_str(&(format!("&term={}", self.term)));
        match self.use_history {
            true => url_str.push_str("&usehistory=y"),
            false => url_str.push_str("&usehistory=n")
        }
        if let Some(i) = &self.webenv {
            url_str.push_str(&(format!("&WebEnv={}", i)));
        } 
        if let Some(i) = &self.query_key {
            url_str.push_str(&(format!("&query_key={}", i)));
        } 
        if let Some(i) = &self.retstart {
            url_str.push_str(&(format!("&WebEnv={}", i)));
        } 
        if let Some(i) = &self.retmax {
            url_str.push_str(&(format!("&retmax={}", i)));
        } 
        if let Some(i) = &self.rettype {
            url_str.push_str(&(format!("&rettype={}", i)));
        } 
        if let Some(i) = &self.retmode {
            url_str.push_str(&(format!("&retmode={}", i)));
        } 
        if let Some(i) = &self.sort {
            url_str.push_str(&(format!("&sort={}", i)));
        }
        if let Some(i) = &self.field {
            url_str.push_str(&(format!("&field={}", i)));
        }
        if let Some(i) = &self.idtype {
            url_str.push_str(&(format!("&idtype={}", i)));
        }
        if let Some(i) = &self.datetype {
            url_str.push_str(&(format!("&datetype={}", i)));
        }
        if let Some(i) = &self.reldate {
            url_str.push_str(&(format!("&reldate={}", i)));
        } 
        if let Some((min, max)) = &self.mindate_maxdate {
            url_str.push_str(&(format!("&mindate={}", min)));
            url_str.push_str(&(format!("&mindate={}", max)));
        }
        
        return url_str;
    }

    fn run(&self) -> Result<String, reqwest::Error> {
        
        let url = self.build_url();
        let res = reqwest::blocking::get(&url)?
                  .text();

        res
        
    }
}

#[derive(Debug, PartialEq)]
pub struct EFetch<'a> {
    pub db: DB, 
    pub id_list: Vec<&'a str>,
    pub webenv: Option<&'a str>,
    pub query_key: Option<&'a str>,  
    pub retstart: Option<&'a str>,
    pub retmax: Option<&'a str>,
    pub rettype: Option<&'a str>,
    pub retmode: Option<&'a str>,
    pub strand: Option<&'a str>,
    pub seq_start: Option<&'a str>,
    pub seq_stop: Option<&'a str>,
    //TODO: implement as an enum for blobs.
    pub complexity: Option<&'a str>,
}

impl<'a> Default for EFetch<'a> {
    fn default() -> Self {
        EFetch {
            db: DB::Pubmed,
            id_list: Vec::new(),
            webenv: None,
            query_key: None,
            retstart: None,
            retmax: None,
            retmode: Some("xml"),
            rettype: Some("xml"),
            strand: None,
            seq_start: None,
            seq_stop: None,
            complexity: None
        }
    }
}

impl<'a> EFetch<'a> {

    pub fn new(db: DB, id_list: Vec<&'a str>) -> Self {
        EFetch {
            db,
            id_list,
            ..Default::default()
        }
    }
}


impl<'a> Eutils for EFetch<'a> {
    fn build_url(&self) -> String {
             let mut url_string = format!("{}efetch.fcgi?",BASE);
             url_string.push_str(&(format!("db={}", &self.db)));
             url_string.push_str(&(format!("&id={}", &self.id_list.join(","))));
             
             if let Some(s) = &self.webenv {
                 url_string.push_str(&(format!("&WebEnv={}", s)))
             }
             if let Some(i) = &self.query_key {
            url_string.push_str(&(format!("&query_key={}", i)));
             } 
            if let Some(i) = &self.retstart {
            url_string.push_str(&(format!("&WebEnv={}", i)));
            } 
            if let Some(i) = &self.retmax {
            url_string.push_str(&(format!("&retmax={}", i)));
            } 
            if let Some(i) = &self.rettype {
            url_string.push_str(&(format!("&rettype={}", i)));
            } 
            if let Some(i) = &self.retmode {
            url_string.push_str(&(format!("&retmode={}", i)));
            } 
             if let Some(i) = &self.query_key {
            url_string.push_str(&(format!("&query_key={}", i)));
             } 
            if let Some(i) = &self.strand {
            url_string.push_str(&(format!("&strand={}", i)));
            } 
            if let Some(i) = &self.seq_start {
            url_string.push_str(&(format!("&seq_start={}", i)));
            } 
            if let Some(i) = &self.seq_stop {
            url_string.push_str(&(format!("&seq_stop={}", i)));
            } 
            if let Some(i) = &self.complexity {
            url_string.push_str(&(format!("&complexity={}", i)));
            } 
             url_string
    }
    fn run(&self) -> Result<String, reqwest::Error> {
        
        let url = self.build_url();
        let res = reqwest::blocking::get(&url)?
                  .text();
        
        res
        
    }
}
