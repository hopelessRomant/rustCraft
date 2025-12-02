#[cfg(test)]
mod tests;

pub fn cs_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut slices = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            slices.push(line);
        }
    }
    slices 
}

pub fn ci_search<'a>(_query: &str, _contents: &'a str) -> Vec<&'a str> {
    vec![""]
}
