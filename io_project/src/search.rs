pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results 
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