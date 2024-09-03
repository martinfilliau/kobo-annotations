use markdown_gen::markdown::{AsMarkdown, Markdown};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Book {
    title: String,
    quotes: Vec<String>
}

fn get_xml_stream(source: String) -> Result<Reader<Box<dyn BufRead>>, String> {
    let local_path = Path::new(&source);

    if local_path.is_file() {
        let file = File::open(local_path);
        if file.is_err() {
            return Err(String::from("Unable to open file"));
        }
        let reader = BufReader::new(file.unwrap());

        return Ok(Reader::from_reader(Box::new(reader)))
    }
    Err(String::from("File not found"))
}

fn to_markdown(book: &Book, output_path: &String) -> i32 {
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

fn main() {
    let path = std::env::args().nth(1).expect("no file given");

    println!("{}", path);   // XXX TODO handle errors if file does not exist

    let quotes_path = format!("{}.md", path);

    let xml = get_xml_stream(path);
    if xml.is_err() {
        let message = xml.err().unwrap();
        println!("=> {}", message);
        return;
    }
    let mut reader = xml.unwrap();

    reader.config_mut().trim_text(true);

    let mut buffer = Vec::new();
    let mut title = String::new();
    let mut highlights = Vec::new();
    let mut is_title = false;
    let mut is_highlight = false;

    loop {
        match reader.read_event_into(&mut buffer) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"dc:title"=> is_title = true,  // XXX TODO extract as const
                    b"text" => is_highlight = true,
                    _ => (),
                }
            },

            Ok(Event::End(e)) => {
                match e.name().as_ref() {
                    b"dc:title" => is_title = false,
                    b"text" => is_highlight = false,
                    _ => (),
                }
            }

            Ok(Event::Text(e)) => {
                if is_title {
                    title = e.unescape().unwrap().into_owned();
                }
                if is_highlight {
                    highlights.push(e.unescape().unwrap().into_owned())
                }
            },

            // There are several other `Event`s we do not consider here
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buffer.clear();
    }

    let mut book = Book { title, quotes: Vec::new() };
    book.quotes = highlights.into_iter().map(|text| text).collect();

    let quotes_count = to_markdown(&book, &quotes_path);

    println!("{} quotes exported for: {} at {}", quotes_count, book.title, quotes_path);
}
