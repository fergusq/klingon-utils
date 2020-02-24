pub mod klingon;
pub mod morpho;
pub mod zrajm;

extern crate regex;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
