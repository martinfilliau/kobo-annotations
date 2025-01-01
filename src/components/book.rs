use crate::components::quote::Quote;
use crate::domain::book::Book;
use leptos::prelude::*;

/// Display a Book as HTML
#[component]
fn Book(book: Book) -> impl IntoView {
    view! {
        <p>Title: {{ book.title }}</p>
        <p>Authors: {{ book.authors }}</p>
         {book.quotes.into_iter()
            .map(|q| view! { <Quote quote=q></Quote>})
            .collect_view()}
    }
}
