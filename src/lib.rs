
pub mod util;
pub mod group;

#[cfg(feature = "class_group")]
extern crate classygroup;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
