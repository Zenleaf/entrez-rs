pub mod eutils;
pub mod parser;

extern crate quick_xml;
extern crate serde;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
