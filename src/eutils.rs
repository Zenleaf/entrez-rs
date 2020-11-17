
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

pub trait Eutils {
    fn build_url(&self) -> String;
}

#[derive(Debug, PartialEq)]
pub struct ESearch {
            // required
            db: DB,
            // required 
            term: String,
            use_history: bool,
            webenv: Option<String>,
            query_key: Option<String>,
            retstart: Option<String>,
            retmax: Option<String>,
            rettype: Option<String>,
            retmode: Option<String>,
            sort: Option<String>,
            field: Option<String>,
            idtype: Option<String>,
            datetype: Option<String>,
            reldate: Option<String>,
            mindate_maxdate: Option<(String,String)>
}

impl Eutils for ESearch {
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

}

impl Default for ESearch {
    fn default() -> Self {
        ESearch {
            db: DB::Pubmed,
            term: "".to_string(),
            use_history: true,
            webenv: None,
            query_key: None,
            retstart: None,
            retmax: None,
            retmode: Some("xml".to_string()),
            rettype: Some("xml".to_string()),
            sort: None,
            field: None,
            idtype: None,
            datetype: None,
            reldate: None,
            mindate_maxdate: None
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct EFetch {
    db: DB, 
    id_list: Vec<String>,
    webenv: Option<String>,
    query_key: Option<String>,  
    retstart: Option<String>,
    retmax: Option<String>,
    rettype: Option<String>,
    retmode: Option<String>,
    strand: Option<String>,
    seq_start: Option<String>,
    seq_stop: Option<String>,
    //TODO: implement as an enum for blobs.
    complexity: Option<String>,
}

impl Eutils for EFetch {
         fn build_url(&self) -> String {
             let mut url_string = format!("{}efetch.fcgi?",BASE);
             url_string.push_str(&(format!("db={}", &self.db)));
             url_string.push_str(&(format!("&idlist={}", &self.id_list.join(","))));
             
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
}
    



