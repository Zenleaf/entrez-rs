#[derive(Debug)]
pub enum ParsingErrorType {
    QXML(quick_xml::DeError),
    RXML(roxmltree::Error)
}

#[derive(Debug)]
pub enum Error {
    ConnectionError(reqwest::Error),
    ParsingError(ParsingErrorType)
}

impl From<reqwest::Error> for Error {
    fn from(re: reqwest::Error) -> Self {
        Error::ConnectionError(re)
    }
}

impl From<quick_xml::DeError> for Error {
    fn from(e: quick_xml::DeError) -> Self {
        Error::ParsingError(
            ParsingErrorType::QXML(e)
        )
    }
}

impl From<roxmltree::Error> for Error {
    fn from(e: roxmltree::Error) -> Self {
        Error::ParsingError(
            ParsingErrorType::RXML(e)
        )
    }
}