pub fn result<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut slices = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            slices.push(line);
        }
    }
    slices 
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

        assert_eq!(vec!["my name is Sahil Singh"], result(query, contents));
    }
}