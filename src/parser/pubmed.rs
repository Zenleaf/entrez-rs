//! Utilities to parse XML results obtained using [EFetch](super::super::eutils::EFetch) from 
//! [DB::Pubmed](super::super::eutils::DB::Pubmed) database.
 
//! # Description
//! When using [EFetch](super::super::eutils::EFetch), with [DB::Pubmed](super::super::eutils::DB::Pubmed)
//!  in the [DB](super::super::eutils::DB) field, the resulting XML
//! is as follows:
//! ``` xml
//! <?xml version="1.0" ?>
//! <!DOCTYPE PubmedArticleSet PUBLIC "-//NLM//DTD PubMedArticle, 1st January 2019//EN" "https://dtd.nlm.nih.gov/ncbi/pubmed/out/pubmed_190101.dtd">
//! <PubmedArticleSet>
//!     <PubmedArticle>
//!      ...
//!     </PubmedArticle>
//!     <PubmedArticle>
//!      ...
//!     </PubmedArticle>
//! </PubmedArticleSet>
//!  ```
//! The [read](PubmedArticleSet::read) function will parse this XML into a [PubmedArticleSet]
//! struct.
//! 
//! # Example
//! 
//!  ```
//!   use entrez_rs::eutils::{Eutils, EFetch, DB};
//!   use entrez_rs::parser::pubmed::{PubmedArticleSet};
//! 
//!   let xml = EFetch::new(
//!         DB::Pubmed, 
//!         vec!["33246200"]) 
//!         .run().expect("Connection error");
//! 
//!   // Use the read function to parse the xml result.
//!   let parsed = PubmedArticleSet::read(&xml)
//!                .expect("Parsing error");
//! 
//!   assert_eq!(parsed.articles.len(), 1);
//!   ```


use roxmltree::{Node, Document};
use super::super::errors::{Error};


#[derive(Debug, PartialEq)]
pub struct Reference {
    citation: Option<String>,
    article_id_list: ArticleIdList
}

#[derive(Debug, PartialEq)]
pub struct ArticleId {
    id_type: Option<String>,
    id: Option<String>
}

#[derive(Debug, PartialEq)]
pub struct ArticleIdList {
    article_ids: Vec<ArticleId>
}

#[derive(Debug, PartialEq)]
pub struct ReferenceList {
    references: Vec<Reference>
}

#[derive(Debug, PartialEq)]
pub struct PubmedData {
    publication_status: Option<String>,
    article_id_list: Option<ArticleIdList>,
    reference_list: Option<ReferenceList>,
    history: Vec<PubMedPubDate>
}

#[derive(Debug, PartialEq)]
pub struct PubMedPubDate {
    pub_status: Option<String>,
    year: Option<String>,
    month: Option<String>,
    day: Option<String>
}

// MedlineCitation Fields
#[derive(Debug, PartialEq)]
pub struct PMID {
    version: Option<String>,
    value: Option<String>
}

#[derive(Debug, PartialEq)]
pub struct ISSN {
    issn_type: Option<String>,
    value: Option<String>
}

#[derive(Debug, PartialEq)]
pub struct PubDate {
    year: Option<String>,
    month: Option<String>,
    day: Option<String>
}

#[derive(Debug, PartialEq)]
pub struct JournalIssue {
    cited_medium: Option<String>,
    volume: Option<String>,
    issue: Option<String>,
    pub_date: Option<PubDate>
}

#[derive(Debug, PartialEq)]
pub struct Journal {
    issn: Option<ISSN>,
    journal_issue: Option<JournalIssue>,
    title: Option<String>,
    iso_abbr: Option<String>
}

#[derive(Debug, PartialEq)]
pub struct ELocationID {
    eid_type: Option<String>,
    valid_yn: Option<String>,
    value: Option<String>
}

#[derive(Debug, PartialEq)]
pub struct AbstractText {
    label: Option<String>,
    nlm_category: Option<String>,
    value: Option<String>
}

#[derive(Debug, PartialEq)]
pub struct Abstract {
    text: Vec<AbstractText>
}

#[derive(Debug, PartialEq)]
pub struct Article {
    pub_model: Option<String>,
    title: Option<String>,
    journal: Option<Journal>,
    elocation_id: Option<ELocationID>,
    language: Option<String>,
    abstract_text: Option<Abstract>
}

#[derive(Debug, PartialEq)]
pub struct MedlineJournalInfo {
    country: Option<String>,
    medline_ta: Option<String>,
    nlm_unique_id: Option<String>,
    issn_linking: Option<String>
}

#[derive(Debug, PartialEq)]
pub struct MedlineCitation {
    status: Option<String>,
    owner: Option<String>,
    pmid: Option<PMID>,
    date_revised: Option<PubDate>,
    article: Option<Article>,
    medline_journal_info: Option<MedlineJournalInfo>
}

#[derive(Debug, PartialEq)]
pub struct PubmedArticle {
    medline_citation: Option<MedlineCitation>,
    pubmed_data: Option<PubmedData>
}

#[derive(Debug, PartialEq)]
pub struct PubmedArticleSet {
    pub articles: Vec<PubmedArticle>
}

pub trait FromXMLNode {
    fn from_node(node: &Node) -> Self;
}

//PubmedData fields
impl FromXMLNode for ArticleIdList {
    fn from_node(node: &Node) -> Self {
        let mut article_id_list = ArticleIdList {
            article_ids: Vec::new()
        };
        
        for elem in node
                         .children()
                         .filter(|n| n.is_element()) {

            article_id_list
            .article_ids
            .push(
                ArticleId::from_node(&elem)
            )                    

                        
        }

        article_id_list
    }
}
impl FromXMLNode for ArticleId {
    fn from_node(node:&Node) -> Self {
        ArticleId {
            id_type: node.attribute("IdType").map(|n| n.to_string()),
            id: node.text().map(|n| n.to_string())
                     
       }
    } 
}
impl FromXMLNode for Reference {
    fn from_node(node: &Node) -> Self {
        let mut reference = Reference {
            citation: None,
            article_id_list: ArticleIdList {
                article_ids: Vec::new()
            }
        };

        for elem in node.children().filter(|x| x.is_element()) {
            match elem.tag_name().name() {
                "Citation" => reference.citation = elem.text().map(|x| x.to_string()),
                "ArticleIdList"  => reference.article_id_list = ArticleIdList::from_node(&elem),
                _          => ()
            }
        }
        reference
    }
}
impl FromXMLNode for ReferenceList {
    fn from_node(node: &Node) -> Self {
        let mut referene_list = ReferenceList {
            references: Vec::new()
        };

        for elem in node
                        .children()
                        .filter(|n| n.is_element()) {
                            referene_list.references.push(
                                Reference::from_node(&elem)
                            )
                        }
        
        referene_list
    }
}
impl FromXMLNode for PubMedPubDate {
    fn from_node(node: &Node) -> Self {
        let mut pubmed_pub_date = PubMedPubDate {
            pub_status: node.attribute("PubStatus")
                             .map(|s| s.to_string()),
            year: None,
            month: None,
            day: None
        };

        for elem in node
            .children()
            .filter(|e| e.is_element()){
             match elem.tag_name().name() {
                 "Year"  => pubmed_pub_date.year = elem.text().map(|e| e.to_string()),
                 "Month" => pubmed_pub_date.month = elem.text().map(|e| e.to_string()),  
                 "Day"   => pubmed_pub_date.day = elem.text().map(|e| e.to_string()),  
                 _       => ()              
             }
        }
        pubmed_pub_date
    }
}
impl FromXMLNode for PubmedData {
    fn from_node(node: &Node) -> Self {
        let mut pubmed_data = PubmedData {
            publication_status: None,
            article_id_list: None,
            reference_list: None,
            history: Vec::new()
        };
        
        for elem in node
            .children()
            .filter(|n| n.is_element()) {
            
            match elem.tag_name().name() {
                "PublicationStatus" => pubmed_data.publication_status = elem.text().map(|e| e.to_string()),
                "ArticleIdList"     => pubmed_data.article_id_list = Some(ArticleIdList::from_node(&elem)),
                "ReferenceList"     => pubmed_data.reference_list  = Some(ReferenceList::from_node(&elem)),
                "History"           => {
                    for pub_date in elem.children().filter(|e| e.is_element()) {
                        pubmed_data.history.push(
                            PubMedPubDate::from_node(&pub_date)
                        )
                    }
                }
                _                   => ()

            }

        }

        pubmed_data
    }
}

//MedlineCitation fields
impl FromXMLNode for PMID {
    fn from_node(node: &Node) -> Self {
        PMID {
            version: node.attribute("Version")
                             .map(|s| s.to_string()),
            
            value: node.text().map(|s| s.to_string())
        }
    }
}
impl FromXMLNode for PubDate {
    fn from_node(node: &Node) -> Self {
            let mut date = PubDate {
                year: None,
                month: None,
                day: None
            };

            for elem in node
                .children()
                .filter(|e| e.is_element()) {
                
                match elem.tag_name().name() {
                    "Year" => date.year = elem.text().map(|s| s.to_string()),
                    "Month" => date.month = elem.text().map(|s| s.to_string()),
                    "Day" => date.day = elem.text().map(|s| s.to_string()),
                    _ => ()
                }
            }
           
            date
        
    }
}
impl FromXMLNode for ISSN {
    fn from_node(node: &Node) -> Self {
        ISSN {
            issn_type: node.attribute("IssnType")
                       .map(|n| n.to_string()),
            value: node.text()
                   .map(|t| t.to_string())
        }
    }
}
impl FromXMLNode for JournalIssue {
    fn from_node(node: &Node) -> Self {
        let mut journal_issue =  JournalIssue {
            cited_medium: node.attribute("CitedMedium")
                          .map(|e| e.to_string()),
            volume: None,
            issue:  None,
            pub_date: None
        };
        for elem in node
                         .children()
                         .filter(|n| n.is_element()) {
            
            match elem.tag_name().name() {
                "Volume" => {
                    journal_issue.volume = elem.text().map(|e| e.to_string())
                },
                "Issue" => {
                    journal_issue.issue = elem.text().map(|e| e.to_string()) 
                },
                "PubDate" => {
                    journal_issue.pub_date = Some(PubDate::from_node(&elem))
                }
                _ => ()
            }
        }
        journal_issue
    }
}
impl FromXMLNode for Journal {
    fn from_node(node: &Node) -> Self {
        let mut journal = Journal {
            issn: None,
            journal_issue: None,
            title: None,
            iso_abbr: None
        };
        
        for elem in node 
                 .children()
                 .filter(|e| e.is_element()) { 
            match elem.tag_name().name() {
                "ISSN" => {
                    journal.issn = Some(ISSN::from_node(&elem))
                },
                "JournalIssue" => {
                    journal.journal_issue = Some(JournalIssue::from_node(&elem))
                },
                "Title" => {
                    journal.title = elem.text()
                                    .map(|e| e.to_string())
                },
                "ISOAbbreviation" => {
                    journal.iso_abbr = elem.text()
                                       .map(|e| e.to_string())
                },
                _ => ()
            }
        }

        journal
    }
}
impl FromXMLNode for ELocationID {
    fn from_node(node: &Node) -> Self {
        ELocationID {
            eid_type: node.attribute("EIdType")
                      .map(|e| e.to_string()),
            valid_yn: node.attribute("ValidYN")
                      .map(|e| e.to_string()),
            value: node.text().map(|t|t.to_string())
        }
    }
}
impl FromXMLNode for AbstractText {
    fn from_node(node: &Node) -> Self {
        let mut abstract_text = AbstractText {
            label: node.attribute("Label")
                   .map(|e| e.to_string()),
            nlm_category: node.attribute("NlmCategory")
                   .map(|e| e.to_string()),
            value: None
        };
        
        let mut txt = Vec::new();
        for elem in node 
            .children() {
            
            if elem.is_text() {
                match elem.text() {
                    Some(t) => txt.push(t),
                    None => ()
                }
                
            }
            
            if elem.is_element() {
               &txt.push("<");
               if elem.namespaces().len() > 0 {
                   for ns in elem.namespaces() {
                       txt.push(ns.name().unwrap());
                       txt.push(":");
                       txt.push(elem.tag_name().name());
                       txt.push(" xmlns:");
                       txt.push(ns.name().unwrap());
                       txt.push("=");
                       txt.push("\"");
                       txt.push(ns.uri());
                       txt.push("\"");
                   }
               }
               else {
                   &txt.push(elem.tag_name().name());
               }
               &txt.push(">");
                
                if elem.has_children() {
                        for c in elem.children() {
                           
                            if c.is_element() {
                           &txt.push("<");
                           if c.namespaces().len() > 0 {
                           for ns in c.namespaces() {
                           txt.push(ns.name().unwrap());
                           txt.push(":");
                           txt.push(c.tag_name().name());
                           txt.push(" xmlns:");
                           txt.push(ns.name().unwrap());
                           txt.push("=");
                           txt.push("\"");
                           txt.push(ns.uri());
                           txt.push("\"");
                        }
                   }
                   else {
                       &txt.push(c.tag_name().name());
                   }
                   &txt.push(">");
                   match c.text() {
                                    Some(t) => txt.push(t),
                                    None => ()
                               }
                   &txt.push("</");
                      if c.namespaces().len() > 0 {
                          for n in c.namespaces() {
                                  txt.push(n.name().unwrap());
                                  txt.push(":");
                          }
               }
               &txt.push(c.tag_name().name());
               &txt.push(">");
                   }
                   else {
                    match c.text() {
                        Some(t) => &txt.push(&t),
                        None => &txt.push("")
                    };
                }
                        
                    
                    };
                }// ends here
                else {
                    match elem.text() {
                        Some(t) => &txt.push(&t),
                        None => &txt.push("")
                    };
                }
                
               &txt.push("</");
               if elem.namespaces().len() > 0 {
                   for ns in elem.namespaces() {
                           txt.push(ns.name().unwrap());
                           txt.push(":");
                   }
               }
               &txt.push(elem.tag_name().name());
               &txt.push(">");
            }
            abstract_text.value = Some(txt.concat())
        }
        
        abstract_text
    }
}
impl FromXMLNode for Abstract {
    fn from_node(node: &Node) -> Self {
        let mut abs = Abstract {
            text: Vec::new()
        };

        for elem in node 
            .children()
            .filter(|e| e.is_element()) {
                match elem.tag_name().name() {
                    "AbstractText" => abs.text.push(
                        AbstractText::from_node(&elem)
                    ),
                    _ => ()
                }
        }

        abs
        
    }
}
impl FromXMLNode for Article {
    fn from_node(node: &Node) -> Self {
        let mut article = Article {
            pub_model: node.attribute("PubModel")
                            .map(|a| a.to_string()),
            title: None,
            journal: None,
            elocation_id: None,
            language: None,
            abstract_text: None
            
      };
      
      for elem in node 
                       .children()
                       .filter(|e| e.is_element()) {
          
          match elem.tag_name().name() {
              "ArticleTitle" => {
                  article.title = elem.text() 
                                  .map(|e| e.to_string())
              },
              "Journal" => {
                  article.journal = Some(Journal::from_node(&elem))
              },
              "ELocationID" => {
                  article.elocation_id = Some(ELocationID::from_node(&elem))
              },
              "Language" => {
                  article.language = elem.text()
                                         .map(|e| e.to_string())
              },
              "Abstract" => {
                  article.abstract_text = Some(Abstract::from_node(&elem))
              },
              _ => ()
          }
      }
      article
    }
}
impl FromXMLNode for MedlineJournalInfo {
    fn from_node(node: &Node) -> Self {
        let mut med_journ_info = MedlineJournalInfo {
            country: None,
            medline_ta: None,
            nlm_unique_id: None,
            issn_linking: None
        };

        for elem in node 
            .children()
            .filter(|e| e.is_element()) {
                match elem.tag_name().name() {
                    "Country" => {
                        med_journ_info
                        .country = elem.text().map(|t| t.to_string())
                    },
                    "MedlineTA" => {
                        med_journ_info
                        .medline_ta = elem.text().map(|t| t.to_string())
                    },
                    "NlmUniqueID" => {
                        med_journ_info
                        .nlm_unique_id = elem.text().map(|t| t.to_string())
                    },
                    "ISSNLinking" => {
                        med_journ_info
                        .issn_linking = elem.text().map(|t| t.to_string())
                    },
                    _ => ()

                }
            }
        med_journ_info
    }
}
impl FromXMLNode for MedlineCitation {
    fn from_node(node: &Node) -> Self {
        let mut medline_citation = MedlineCitation {
            status: node.attribute("Status")
                             .map(|s| s.to_string()),
            
            owner: node.attribute("Owner")
                             .map(|s| s.to_string()),
            pmid: None,

            date_revised: None,

            article: None,

            medline_journal_info: None
        };

        for elem in node
            .children() 
            .filter(|e| e.is_element()) {
                match elem.tag_name().name() { 
                    "PMID" => medline_citation.pmid =  Some(PMID::from_node(&elem)),
                    "DateRevised" => medline_citation.date_revised = Some(PubDate::from_node(&elem)),
                    "Article" => medline_citation.article = Some(Article::from_node(&elem)),  
                    "MedlineJournalInfo" => medline_citation.medline_journal_info = Some(MedlineJournalInfo::from_node(&elem)),                                      
                    _ => ()
                }
        }

        medline_citation

    }
}

//PubmedArticle
impl FromXMLNode for PubmedArticle {
    fn from_node(node: &Node) -> Self {
        let mut pub_art = PubmedArticle {
            medline_citation: None,
            pubmed_data: None
        };
        
        for elem in node
            .children() 
            .filter(|e| e.is_element()) {
                match elem.tag_name().name() { 
                    "MedlineCitation" => pub_art.medline_citation =  Some(MedlineCitation::from_node(&elem)),
                    "PubmedData" => pub_art.pubmed_data = Some(PubmedData::from_node(&elem)),
                    _ => ()
                }
        }

        pub_art
    }
}

//PubmedArticleSet

impl FromXMLNode for PubmedArticleSet {
    fn from_node(node: &Node) -> Self {
        let mut pm_set = PubmedArticleSet {
            articles: Vec::new()
        };
        
        for elem in node
            .children() 
            .filter(|e| e.is_element()) {
                match elem.tag_name().name() { 
                    "PubmedArticle" => pm_set.articles.push(
                        PubmedArticle::from_node(&elem)
                    ),
                    _ => ()
                }
        }

        pm_set
    }
}

impl PubmedArticleSet {
    pub fn read(xml: &str) -> Result<Self, Error> {

        let pm_parsed = Document::parse(&xml)?;
        
        let pm_art_node = &pm_parsed 
              .descendants()
              .find(|n| n.has_tag_name("PubmedArticleSet")).unwrap();
        
        let res = PubmedArticleSet::from_node(&pm_art_node);
        
        Ok(res)
    }
}
