#[cfg(test)]
mod tests;

pub fn cs_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut slices = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            slices.push(line);
        }
    }
    slices 
}

pub fn ci_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut slices = Vec::new();
    let query_low = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&query_low) {
            slices.push(line);
        }
    }
    slices
}
