use crate::domain::book::Quote;
use leptos::prelude::*;

#[component]
pub fn Quote(quote: Quote) -> impl IntoView {
    view! {
        <p>{{quote.quote}}</p>
    }
}
