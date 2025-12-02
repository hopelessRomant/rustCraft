pub fn search<'a>(_query: &str, _contents: &'a str) -> Vec<&'a str> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_result() {
        let query = "is";
        let contents = "\
        Rust:
        Hello!
        my name is Sahil Singh
        nice to meet you.";

        assert_eq!(vec!["my name is Sahil Singh"], search(query, contents));
    }
}