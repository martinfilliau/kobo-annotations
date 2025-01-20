#[derive(Clone)]
pub struct Quote {
    pub quote: String,
}

pub struct Book {
    pub title: String,
    pub authors: String,
    pub quotes: Vec<Quote>,
}
