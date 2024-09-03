use std::{fs::File, io::{BufRead, BufReader}, path::Path};
use quick_xml::Reader;

pub fn get_xml_stream(source: String) -> Result<Reader<Box<dyn BufRead>>, String> {
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
