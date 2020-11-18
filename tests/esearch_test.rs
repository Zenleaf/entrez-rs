extern crate entrez_rs;

use entrez_rs::eutils::{Eutils, ESearch, DB};
use entrez_rs::parser::esearch::{ESearchResult};

#[test]
fn eseasrch_run_test() -> Result<(), String>  {
        let xml = ESearch::new(
            DB::Pubmed, 
            "eclamsia")
            .run().expect("Failed to retrieve data from Entrez");

        let parsed = ESearchResult::read(&xml);

        println!("{:#?}", &parsed.expect("Failed to parse XML")
                           .id_list
                           .id);

        Ok(())
}