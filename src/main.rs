use kobo_annotations::from_xml::get_xml_stream;
use kobo_annotations::parser::xml_to_struct;
use kobo_annotations::to_markdown::to_markdown;

fn main() {
    let path = std::env::args().nth(1).expect("no file given");

    println!("Importing {}", path);   // XXX TODO handle errors if file does not exist

    let quotes_path = format!("{}.md", path);

    let xml = get_xml_stream(path);
    if xml.is_err() {
        let message = xml.err().unwrap();
        println!("=> {}", message);
        return;
    }
    let reader = xml.unwrap();

    let book = xml_to_struct(reader);
    let quotes_count = to_markdown(&book, &quotes_path);

    println!("{} quotes exported for: {} at {}", quotes_count, book.title, quotes_path);
}
