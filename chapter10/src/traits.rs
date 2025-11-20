pub trait Summary {
    fn headline (&self) -> String;
}

pub trait Authors {
    fn authors<T> (&self) -> &T;
}

pub struct NewsAtricle {
    pub heading: String,
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
        format!("{}", self.heading)
    }
}

impl NewsAtricle {
    pub fn data (heading: String) -> NewsAtricle{
        NewsAtricle { heading: heading,
        body: ("Blame the aliens").to_string(),
        author: ("Sahil Singh").to_string(),
        date: ("16 - 11 - 2025").to_string()}
    }
}

impl Summary for ResearchArticle {
    fn headline (&self) -> String {
        format!("{}", self.title)
    }
}


// traint bound syntax -> usefull when contraining the types on the parameters
pub fn notify_bound<T: Summary> (item: &T) {
    println!("breaking news: {}", item.headline());
}

// &impl method -> to keep the type independence
pub fn notify_impl(item: &impl Summary) {
    println!("breaking news: {}", item.headline());
}