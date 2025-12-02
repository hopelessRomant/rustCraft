use super::*;

#[test]
fn search_search() {
    let query = "is";
    let content = "\
Rust:
Hello!
my name is Sahil Singh
nice to meet you.";

    assert_eq!(vec!["my name is Sahil Singh"], cs_search(query, content));
}

#[test]
fn no_case() {
    let query = "kInG";
    let content = "/
KiNg:
handeling
sulkIng
STnDing
forKING";

    assert_eq!(vec!["KiNg:", "sulkIng", "forKING"], ci_search(query, content));

}
