pub trait Summary {
    fn headline (&self) -> String;
    fn authors (&self) -> String;
}

pub struct NewsAtricle {
    pub heading: String,
    body: String,
    authors: Vec<String>,
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

    fn authors (&self) -> String {
        format!("{:#?}", self.authors)
    }
}

impl NewsAtricle {
    pub fn data (heading: String) -> NewsAtricle{
        NewsAtricle { heading: heading,
        body: ("Blame the aliens").to_string(),
        authors: vec![("Sahil Singh").to_string()],
        date: ("16 - 11 - 2025").to_string()}
    }
}

impl Summary for ResearchArticle {
    fn headline (&self) -> String {
        format!("{}", self.title)
    }

    fn authors (&self) -> String {
        format!("{:#?}", self.authors)
    }
}

// trait bound syntax -> usefull when contraining the types on the parameters
pub fn notify_bound<T: Summary> (item: &T) {
    println!("breaking news: {}", item.headline());
}

// &impl method -> to keep the type independence
pub fn notify_impl(item: &impl Summary) {
    println!("breaking news: {}", item.headline());
}

// + symbol to use multiple traits ni parameters and bounds -> pub fn notify (item: &(impl Summary + Display) {}
