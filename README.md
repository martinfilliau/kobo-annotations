# kobo-annotations

_Project status: proof of concept_

Extract annotations (highlights) from Kobo books into a markdown file.

Uses the `.epub.annot` XML file as a source.

## Usage

Run `cargo run -- /full/path/to/file`

It will output a file at `/full/path/to/file.md`.

## Tests

Run `cargo test`.

## TODO

- [x] add tests
- [ ] properly handle XML [namespaces](https://docs.rs/quick-xml/latest/quick_xml/name/index.html) ?! (check [this conversation](https://stackoverflow.com/questions/50963890/xml-format-in-kobo-sqlite-database-for-bookmarks))
- [x] separate into different files w/ own responsibility
- [ ] handle lines return
- [ ] add documentation (ADR, use cases...)
- [ ] templatize output?
- [ ] handle relative file path
- [ ] customize output destination
- [ ] (optionnally) use ebook to get highlight's context (e.g. nearest sub-title) ?
- [ ] ability to use as a library as well as a CLI? (i.e. decouple lib and CLI) / [modules](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
- [ ] version
- [ ] generalize [result pattern](https://doc.rust-lang.org/std/result/)
