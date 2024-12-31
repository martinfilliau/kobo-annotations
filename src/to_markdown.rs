use crate::data::Book;
use markdown_gen::markdown::{AsMarkdown, Markdown};
use std::fs::File;

pub fn to_markdown(book: &Book, output_path: &String) -> Result<usize, String> {
    let file = File::create(output_path).unwrap();
    let mut md = Markdown::new(file);

    md.write(book.title.heading(1)).unwrap();

    md.write(book.authors.paragraph()).unwrap();

    md.write("Annotations".heading(2)).unwrap();

    book.quotes.clone().into_iter().for_each(|quote| {
        let formatted = format!(" {quote}");
        md.write(formatted.quote()).unwrap();
    });

    Ok(book.quotes.len())
}
