use epub::doc::{EpubDoc, NavPoint};

pub fn from_epub(epub_path: String) -> Result<Vec<String>, &'static str> {
    let result_epub = EpubDoc::new(epub_path);
    if result_epub.is_err() {
        return Err("Cannot open epub");
    }

    let epub = result_epub.unwrap();
    let mut results = Vec::new();
    let toc = epub.toc.clone();

    // XXX TODO recurse + handle ref point
    for item in toc {
        results.push(item.label.clone());
        results.push(from_chapter(&item, "- ").concat());
    }

    Ok(results)
}

fn from_chapter(toc: &NavPoint, prefix: &str) -> Vec<String> {
    let mut results = Vec::new();

    for item in &toc.children {
        results.push(format!("{}{}", prefix, item.label))
    }

    results
}
