use super::*;

#[test]
fn search_search() {
    let query = "is";
    let contents = "\
Rust:
Hello!
my name is Sahil Singh
nice to meet you.";

    assert_eq!(vec!["my name is Sahil Singh"], search
    (query, contents));
}
