# entrez-rs

> A Rust wrapper for the Entrez API 

release: [![Crates.io][ci]][cl] 

[ci]: https://img.shields.io/crates/v/entrez-rs.svg
[cl]: https://crates.io/crates/entrez-rs/

master: ![Build Status](https://travis-ci.org/Zenleaf/entrez-rs.svg?branch=master)

This library helps you access the Entrez API using idiomatic Rust.
It also provides tools to parse the XML results from Entrez.

## Installation

Add the following to your Cargo.toml:
```toml
   [dependencies]
   entrez-rs = "0.1.2-alpha"
```

## Usage
```rust
use entrez_rs::eutils::{Eutils, ESearch, EFetch, DB};
use entrez_rs::parser::esearch::{ESearchResult};
use entrez_rs::parser::pubmed::{PubmedArticleSet};
use entrez_rs::errors::Error;

fn main() -> Result<(), Error>  {
        let xml = ESearch::new(
            DB::Pubmed, 
            "eclampsia")
            .run()?;

        let parsed = ESearchResult::read(&xml);

        println!("{:#?}", &parsed?
                          .id_list
                          .ids);
        
        let pm_xml = EFetch::new(
              DB::Pubmed,
              vec!["33246200", "33243171"])
              .run()?;
        
        let pm_parsed = PubmedArticleSet::read(&pm_xml);

        println!("{}", pm_parsed?.articles);

        Ok(())
}


```
Will add a walkthrough and tutorial of the API as soon as it reaches beta level.

Inspired by Entrez Direct, Entrezpy and BioPython.
