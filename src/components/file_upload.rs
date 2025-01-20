use epub::doc::EpubDoc;
use leptos::prelude::{signal, ElementChild, GlobalAttributes, OnAttribute, Set};
use leptos::*;
use std::io::Cursor;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::FileReader;

#[component]
pub fn FileUpload() -> impl IntoView {
    let (feedback, set_feedback) = signal(String::new());
    let (epub_title, set_epub_title) = signal(String::new());

    let handle_file_change = move |event: web_sys::Event| {
        if let Some(input) = event
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        {
            if let Some(file_list) = input.files() {
                if let Some(file) = file_list.item(0) {
                    let file_name = file.name();
                    let file_type = file.type_();

                    // Check for XML file (basic check: MIME type or file extension)
                    if file_type == "application/epub+zip" || file_name.ends_with(".epub") {
                        set_feedback.set(format!("File '{}' is valid.", file_name));

                        let reader = FileReader::new().unwrap_throw();
                        let reader_clone = reader.clone();
                        let set_feedback_clone = set_feedback.clone();
                        let set_epub_title_clone = set_epub_title.clone();

                        let onloadend = Closure::wrap(Box::new(move || {
                            if let Ok(content) = reader_clone.result() {
                                if let Some(array_buffer) =
                                    content.dyn_into::<js_sys::ArrayBuffer>().ok()
                                {
                                    // Convert ArrayBuffer to Vec<u8>
                                    let bytes = js_sys::Uint8Array::new(&array_buffer);
                                    let file_data = bytes.to_vec();

                                    let mut cursor = Cursor::new(file_data);
                                    match EpubDoc::from_reader(&mut cursor) {
                                        Ok(mut doc) => {
                                            if let Some(title) = doc.mdata("title") {
                                                set_epub_title_clone
                                                    .set(format!("EPUB Title: {}", title));
                                            } else {
                                                set_feedback_clone.set(
                                                    "Could not extract title from the EPUB."
                                                        .to_string(),
                                                );
                                            }
                                        }
                                        Err(err) => {
                                            set_feedback_clone
                                                .set(format!("Failed to parse EPUB: {}", err));
                                        }
                                    }
                                }
                            }
                        }) as Box<dyn Fn()>);

                        reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
                        reader.read_as_array_buffer(&file).unwrap_throw();
                        onloadend.forget(); // Prevent memory leaks
                    } else {
                        set_feedback
                            .set("Invalid file type. Please upload an XML file.".to_string());
                    }
                }
            }
        }
    };

    view! {
        <div>
            <label for="file-upload">"Upload an XML file:"</label>
            <input
                id="file-upload"
                type="file"
                accept=".epub"
                on:change=handle_file_change
            />
            <p>{feedback}</p>
            <p>{epub_title}</p>
        </div>
    }
}
