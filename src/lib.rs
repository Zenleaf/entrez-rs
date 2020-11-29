//! # Introduction
//! The aim of this crate is to be a complete, idiomatic wrapper 
//! for the Entrez API.
//! 
//! The API is designed to be simple but at the same time,
//! flexible enough for advanced use cases.
//! 
//! # Usage
//! There are two categories of tools provided by this library:
//!  - *Wrappers* for the Entrez Eutilities.
//!  - *Parsers* to parse XML results obtained using the Eutilities.
//! ### Wrappers:
//! The [eutils] module contains the wrappers.
//! Currently available wrappers are:
//!  - [ESearch](eutils::ESearch)
//!  - [EFetch](eutils::EFetch) 
//! 
//! ### Parsers:
//! The [parser] module contains the parsers.
//! Currently available parsers are:
//!  - [esearch](parser::esearch)
//!  - [pubmed](parser::pubmed)
pub mod eutils;
pub mod parser;
pub mod errors;

#[cfg(test)]
mod tests {
    use super::eutils::*;
    
    #[test]
    fn build_esearch() {
        let url = ESearch::new(DB::Pubmed, "eclampsia")
        .build_url();
  
    
        assert_eq!(
            &url,
            "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi?db=pubmed&term=eclampsia&usehistory=y&rettype=xml&retmode=xml"
        );
    }

    #[test]
    fn build_efetch() {
        let url = EFetch::new(
            DB::Pubmed,
             vec!["33246200"])
             .build_url();
        
        println!("{:#?}", &url);
    }
}