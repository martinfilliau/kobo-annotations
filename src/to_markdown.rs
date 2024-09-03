use std::fs::File;
use markdown_gen::markdown::{AsMarkdown, Markdown};
use crate::data::Book;

pub fn to_markdown(book: &Book, output_path: &String) -> i32 {
    let file = File::create(output_path).unwrap();
    let mut md = Markdown::new(file);

    md.write(book.title.heading(1)).unwrap();

    let mut quotes_count = 0;
    book.quotes.clone().into_iter().for_each(|quote| {
        md.write(quote.as_str()).unwrap();
        quotes_count = quotes_count + 1; // XXX TODO book.quotes.len()
    });

    return quotes_count;
}
