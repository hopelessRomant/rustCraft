#[cfg(test)]
mod tests;

pub fn cs_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|lines| lines.contains(query))
        .collect()
}

pub fn ci_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|lines| lines.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}
