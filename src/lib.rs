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
}