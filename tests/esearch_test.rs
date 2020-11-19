extern crate entrez_rs;

use entrez_rs::eutils::{Eutils, ESearch, DB};
use entrez_rs::parser::esearch::{ESearchResult};
use entrez_rs::errors::Error;

#[test]
fn esearch_run_test() -> Result<(), Error>  {
        let xml = ESearch::new(
            DB::Pubmed, 
            "eclampsia")
            .run()?;

        let parsed = ESearchResult::read(&xml);

        println!("{:#?}", &parsed?
                          .id_list
                          .id);

        Ok(())
}