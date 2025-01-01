use leptos::prelude::*;
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::*;

#[component]
fn FileUpload() -> impl IntoView {
    let file_input: NodeRef<html::Input> = NodeRef::new();

    let upload_file = move |_| {
        if let Some(input) = file_input.get() {
            if let Some(file) = input
                .dyn_into::<HtmlInputElement>()
                .ok()
                .and_then(|input| input.files())
                .and_then(|files| files.get(0))
            {}
        }
    };

    view! { cx,
        <div>
            <input type="file" node_ref=file_input/>
            <button on:click=upload_file>"Upload"</button>
        </div>
    }
}
