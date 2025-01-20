use kobo_annotations::components::App::App;
use kobo_annotations::from_xml::get_xml_stream;
use kobo_annotations::parser::xml_to_struct;
use kobo_annotations::to_markdown::to_markdown;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
    return;

    let path = std::env::args().nth(1).expect("no file given");

    println!("Importing {}", path);

    //let result = from_epub(path);
    //if result.is_ok() {
    //    let chapters = result.ok().unwrap();
    //    for chapter in chapters {
    //        println!("{}", chapter);
    //    }
    //}

    let quotes_path = format!("{}.md", path);

    let xml = get_xml_stream(path);
    if xml.is_err() {
        let message = xml.err().unwrap();
        println!("ERROR => {}", message);
        return;
    }
    let reader = xml.unwrap();

    let result_book = xml_to_struct(reader);
    if result_book.is_err() {
        let message = result_book.err().unwrap();
        println!("ERROR => {}", message);
        return;
    }

    let book = result_book.ok().unwrap();
    let result_markdown = to_markdown(&book, &quotes_path);
    if result_markdown.is_err() {
        let message = result_markdown.err().unwrap();
        println!("ERROR => {}", message);
        return;
    }

    println!(
        "{:?} quotes exported for: {} at {}",
        result_markdown.is_ok(),
        book.title,
        quotes_path
    );
}
