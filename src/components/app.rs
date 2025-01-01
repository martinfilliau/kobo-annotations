use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button
            on:click=move |_| {
                *set_count.write() += 1;
            }
        >"Increment"
        </button>
        <button
            on:click=move |_| {
                *set_count.write() -= 1;
            }
        >"Decrement"
        </button>
        <p
            class:red=move || is_odd(count.get())
        >{count}</p>
        <p>
            "Double count: "
            {move || count.get() * 2}
        </p>
    }
}

fn is_odd(count: usize) -> bool {
    count % 2 == 1
}
