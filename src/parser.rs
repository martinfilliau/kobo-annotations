use std::io::BufRead;
use quick_xml::Reader;
use quick_xml::events::Event;
use crate::data::Book;

pub fn xml_to_struct<R: BufRead>(mut reader: Reader<R>) -> Book {
    reader.config_mut().trim_text(true);

    let mut buffer = Vec::new();
    let mut title = String::new();
    let mut authors = String::new();
    let mut highlights = Vec::new();
    let mut is_title = false;
    let mut is_authors = false;
    let mut is_highlight = false;
    let mut in_publication = false;

    loop {
        match reader.read_event_into(&mut buffer) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"dc:title" => is_title = true,  // XXX TODO extract as const
                    b"dc:creator" => is_authors = if in_publication { true } else { false },
                    b"text" => is_highlight = true,
                    b"publication" => in_publication = true,
                    _ => (),
                }
            },

            Ok(Event::End(e)) => {
                match e.name().as_ref() {
                    b"dc:title" => is_title = false,
                    b"dc:creator" => is_authors = false,
                    b"text" => is_highlight = false,
                    b"publication" => in_publication = false,
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
                if in_publication && is_authors {
                    authors = e.unescape().unwrap().into_owned();
                }
            },

            // There are several other `Event`s we do not consider here
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buffer.clear();
    }

    let mut book = Book { title, authors, quotes: Vec::new() };
    book.quotes = highlights.into_iter().map(|text| text).collect();

    return book;
}

#[cfg(test)]
mod test {
    use quick_xml::reader::Reader;

    use crate::parser::xml_to_struct;

    #[test]
    fn read_bad_xml_file() {
        let xml = r#"<tag1 att1 = "test">
                        <tag2><!--Test comment-->Test</tag2>
                        <tag2>Test 2</tag2>
                     </tag1>"#;
        let reader = Reader::from_str(xml);

        let book = xml_to_struct(reader);

        // XXX TODO should return a result w/ error
        assert_eq!(book.title, String::new());
    }

    #[test]
    fn read_bad_file() {
        let xml = r#"boo"#;
        let reader = Reader::from_str(xml);

        let book = xml_to_struct(reader);

        // XXX TODO should return a result w/ error
        assert_eq!(book.title, String::new());
    }

    #[test]
    fn read_valid_wo_highlights() {
        let xml = r#"<annotationSet xmlns:xhtml="http://www.w3.org/1999/xhtml" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns="http://ns.adobe.com/digitaleditions/annotations">
                <publication>
                    <dc:title>Samouraï</dc:title>
                    <dc:creator>Fabrice Caro</dc:creator>
                    <dc:description>SFC</dc:description>
                    <dc:language>fr</dc:language>
                </publication>
            </annotationSet>"#;
        let reader = Reader::from_str(xml);

        let book = xml_to_struct(reader);

        assert_eq!(book.title, "Samouraï");
        assert_eq!(book.authors, "Fabrice Caro");
        assert_eq!(book.quotes.is_empty(), true);
    }

    #[test]
    fn read_valid_w_highlights() {
        let xml = r#"
    <annotationSet xmlns:xhtml="http://www.w3.org/1999/xhtml" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns="http://ns.adobe.com/digitaleditions/annotations">
        <publication>
            <dc:identifier>amnt</dc:identifier>
            <dc:title>Architecture Modernization: Socio-technical alignment of software, strategy, and structure</dc:title>
            <dc:creator>Nick Tune</dc:creator>
            <dc:language>en</dc:language>
        </publication>
        <annotation>
            <dc:identifier>urn:uuid:497c97ff-efea-440d-bc24-2715f8043c27</dc:identifier>
            <dc:date>2024-08-07T13:15:32Z</dc:date>
            <dc:creator>urn:uuid:156adb1c-9d3a-4a2d-830a-63a9831c5a7c</dc:creator>
            <target>
                <fragment start="OEBPS/Text/07.htm#point(/1/4/184/8:228)" end="OEBPS/Text/07.htm#point(/1/4/185:1)" progress="0.311828" color="4">
                    <text>One way to check if you have suitable pivotal events is to ask: Do the pivotal events alone tell the high-level story of the domain?
        </text>
                </fragment>
            </target>
        </annotation>
        <annotation>
            <dc:identifier>urn:uuid:7fbad2bc-6a03-4e31-9ced-b1a6a39d4f0f</dc:identifier>
            <dc:date>2024-08-09T07:58:08Z</dc:date>
            <dc:creator>urn:uuid:156adb1c-9d3a-4a2d-830a-63a9831c5a7c</dc:creator>
            <target>
                <fragment start="OEBPS/Text/08.htm#point(/1/4/91:4)" end="OEBPS/Text/08.htm#point(/1/4/92/2:154)" progress="0.35914" color="4">
                    <text>Sometimes, engineers don’t talk to users because of cultural perspectives. For instance, their only value is perceived as sitting at their desks coding.</text>
                </fragment>
            </target>
        </annotation>
    </annotationSet>"#;
        let reader = Reader::from_str(xml);

        let book = xml_to_struct(reader);

        assert_eq!(book.title, "Architecture Modernization: Socio-technical alignment of software, strategy, and structure");
        assert_eq!(book.authors, "Nick Tune");
        assert_eq!(book.quotes.len(), 2);
    }
}
