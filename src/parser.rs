use std::io::BufRead;
use quick_xml::Reader;
use quick_xml::events::Event;
use crate::data::Book;

pub fn xml_to_struct(mut reader: Reader<Box<dyn BufRead>>) -> Book {
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

    return book;
}
