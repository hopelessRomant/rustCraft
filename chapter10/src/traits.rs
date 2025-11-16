pub trait Summary {
    fn headline (&self) -> String;
}

pub struct NewsAtricle {
    heading: String,
    body: String,
    author: String,
    date: String,
}

pub struct ResearchArticle {
    title: String,
    intro: String,
    body: String,
    authors: Vec<String>,
    references: Vec<String>,
}

impl Summary for NewsAtricle {
    fn headline (&self) -> String {
        format!("Headline to the given article is: {}", self.heading)
    }
}