use super::*;

#[test]
fn case_sensitive() {
    let query = "is";
    let content = "\
Rust:
Hello!
my name is Sahil Singh
nice to meet you.";

    assert_eq!(vec!["my name is Sahil Singh"], cs_search(query, content));
}

#[test]
fn case_insensitive() {
    let query = "kInG";
    let content = "/
KiNg:
handeling
sulkIng
STnDing
forKING";

    assert_eq!(vec!["KiNg:", "sulkIng", "forKING"], ci_search(query, content));
}
