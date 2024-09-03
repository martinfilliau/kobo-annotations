# kobo-annotations

_Project status: proof of concept_

Extract annotations (highlights) from Kobo books into a markdown file.

Uses the `.epub.annot` XML file as a source.

## Usage

Run `cargo run -- /full/path/to/file`

It will output a file at `/full/path/to/file.md`.

## TODO

- [ ] add tests
- [ ] properly handle namespaces ?! (check [this conversation](https://stackoverflow.com/questions/50963890/xml-format-in-kobo-sqlite-database-for-bookmarks))
- [ ] move into modules
- [ ] handle lines return
- [ ] add documentation (ADR, use cases...)
- [ ] templatize output?
- [ ] customize output destination
- [ ] (optionnally) use ebook to get highlight's context (e.g. nearest sub-title) ?
- [ ] ability to use as a library as well as a CLI? (i.e. decouple lib and CLI)
