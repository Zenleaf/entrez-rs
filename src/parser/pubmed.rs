extern crate quick_xml;
extern crate serde;

use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::de::{ DeError};


#[derive(Debug, PartialEq)]
pub struct PubmedArticleSet {
    pub pubmed_articles: Vec<PubmedArticle>
}

#[derive(Debug, PartialEq)]
pub struct PubmedArticle {
    pub medline_citation: MedlineCitation
}

#[derive(Debug, PartialEq)]
pub struct MedlineCitation{
    pub article: Article
}

#[derive(Debug, PartialEq)]
pub struct Article{
    pub title: String,
    pub abstracts: Vec<AbstractText>
}

#[derive(Debug, PartialEq)]
pub struct AbstractText {
    pub text: String,
}

/*
 Need this this parser to manually parse PubmedArticle results 
 returned in XML format. 

 Once it is possible to  do the same using Serde will use
 the conventional serde way to deserialize XML pubmed article.

*/
pub fn parse_pubmed(xml: &str) -> Result<PubmedArticle, DeError> {
    let mut pub_art_set = PubmedArticle {
        medline_citation: MedlineCitation {
            article: Article {
                title: "".to_string(),
                abstracts: Vec::new()
            }
        }
    };
    
    let mut buf = Vec::new();
    let mut reader = Reader::from_str(&xml);

    'outer:loop {
        match &reader.read_event(&mut buf) {
            Ok(Event::Start(e)) => match e.name() {
                b"ArticleTitle" => {
                   match reader.read_event(&mut buf) {
                       Ok(Event::Text(e)) => {
                           pub_art_set
                           .medline_citation
                           .article
                           .title = e.unescape_and_decode(&reader)?;
                   },
                   Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => ()
                }
            },
                b"Abstract" => {
                    'inner:loop {
                        let abs = &pub_art_set
                                  .medline_citation
                                  .article
                                  .abstracts.len() ;

                        match &reader.read_event(&mut buf) {
                            Ok(Event::Text(e)) => {
                                if *abs != 0 {
                                    pub_art_set
                                .medline_citation
                                .article
                                .abstracts[*abs - 1]
                                .text
                                .push_str(
                                    &e.unescape_and_decode(&reader)?
                                )
                                }
                            },
                            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                            Ok(Event::Start(e)) => match e.name() {
                                b"b" => {
                                    pub_art_set
                                    .medline_citation
                                    .article
                                    .abstracts[*abs - 1]
                                   .text
                                   .push_str("<b>");

                                   match &reader.read_event(&mut buf) {
                                    Ok(Event::Text(e)) => {
                                        pub_art_set
                                        .medline_citation
                                        .article
                                        .abstracts[*abs - 1]
                                        .text
                                        .push_str(
                                            &e.unescape_and_decode(&reader)?
                                        )},
                                    _ => ()
                                    }
                                
                            
                            },
                            b"mml:math" => {
                                pub_art_set
                                .medline_citation
                                .article
                                .abstracts[*abs - 1]
                               .text
                               .push_str("<mml:math>");

                               match &reader.read_event(&mut buf) {
                                Ok(Event::Text(e)) => {
                                    pub_art_set
                                    .medline_citation
                                    .article
                                    .abstracts[*abs - 1]
                                    .text
                                    .push_str(
                                        &e.unescape_and_decode(&reader)?
                                    )},
                                _ => ()
                                }},
                                b"mml:mo" => {
                                    pub_art_set
                                    .medline_citation
                                    .article
                                    .abstracts[*abs - 1]
                                   .text
                                   .push_str("<mml:mo>");
    
                                   match &reader.read_event(&mut buf) {
                                    Ok(Event::Text(e)) => {
                                        pub_art_set
                                        .medline_citation
                                        .article
                                        .abstracts[*abs - 1]
                                        .text
                                        .push_str(
                                            &e.unescape_and_decode(&reader)?
                                        )},
                                    _ => ()
                                    }},
                            b"sup" => {
                                pub_art_set
                                .medline_citation
                                .article
                                .abstracts[*abs - 1]
                               .text
                               .push_str("<sup>");

                               match &reader.read_event(&mut buf) {
                                Ok(Event::Text(e)) => {
                                    pub_art_set
                                    .medline_citation
                                    .article
                                    .abstracts[*abs - 1]
                                    .text
                                    .push_str(
                                        &e.unescape_and_decode(&reader)?
                                    )},
                                _ => ()
                                }},
                            b"sub" => {
                                pub_art_set
                                .medline_citation
                                .article
                                .abstracts[*abs - 1]
                               .text
                               .push_str("<sub>");

                               match &reader.read_event(&mut buf) {
                                Ok(Event::Text(e)) => {
                                    pub_art_set
                                    .medline_citation
                                    .article
                                    .abstracts[*abs - 1]
                                    .text
                                    .push_str(
                                        &e.unescape_and_decode(&reader)?
                                    )},
                                _ => ()
                                }
                            },
                            b"i" => {
                                pub_art_set
                                .medline_citation
                                .article
                                .abstracts[*abs - 1]
                                .text
                                .push_str(
                                    "<i>"
                                );

                                match &reader.read_event(&mut buf) {
                                    Ok(Event::Text(e)) => {
                                        pub_art_set
                                        .medline_citation
                                        .article
                                        .abstracts[*abs - 1]
                                        .text
                                        .push_str(
                                            &e.unescape_and_decode(&reader)?
                                        )},
                                    _ => ()
                                    }
                               },
                                b"AbstractText" => {
                                    pub_art_set
                                    .medline_citation
                                    .article
                                    .abstracts
                                    .push(
                                        AbstractText {
                                            text: "".to_string()
                                        }
                                    );
                                    

                                    match &reader.read_event(&mut buf) {
                                        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                                        Ok(Event::Text(e)) => {
                                            pub_art_set
                                            .medline_citation
                                            .article
                                            .abstracts[*abs]
                                            .text
                                            .push_str(
                                                &e.unescape_and_decode(&reader)?
                                            )
                                        },
                                        _ => ()
                                    }
                                },
                               
                                _ => ()
                                
    
                            },
                            
                            Ok(Event::End(e)) => match e.name() {
                                b"i" => {
                                    pub_art_set
                                    .medline_citation
                                    .article
                                    .abstracts[*abs - 1]
                                    .text
                                    .push_str(
                                        "</i>"
                                    )
                                   },
                                b"sup" => {
                                    pub_art_set
                                    .medline_citation
                                    .article
                                    .abstracts[*abs - 1]
                                    .text
                                    .push_str(
                                        "</sup>"
                                    )
                                   },
                                   b"sub" => {
                                    pub_art_set
                                    .medline_citation
                                    .article
                                    .abstracts[*abs - 1]
                                    .text
                                    .push_str(
                                        "</sub>"
                                    )
                                   },
                                   b"b" => {
                                    pub_art_set
                                    .medline_citation
                                    .article
                                    .abstracts[*abs - 1]
                                    .text
                                    .push_str(
                                        "</b>"
                                    )
                                   },
                                b"Abstract" => break 'inner,
                                _           => ()
                            }
                            _ => ()
                        }
                    }
                    },
                _ => ()
    },
           
            Ok(Event::End(e)) => match e.name() {
                b"AuthorList" => break 'outer,
                _ => ()
            },
            _               => ()
    }}
    Ok(pub_art_set)

}

//-------------------------------------------------------

