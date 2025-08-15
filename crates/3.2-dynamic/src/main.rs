use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;

    view! {
        <button
            on:click=move |_| {
                *set_count.write() += 1;
            }
            style:background-color=move || format!("rgb({}, {}, 100)", 10 * count.get(), 100)
        >
            "Click me "
            {count}
        </button>
        <p>
            "Double Count: " // and again here
            {double_count}
        </p>
        <progress
            max="50"
            // signals are functions, so `value=count` and `value=move || count.get()`
            // are interchangeable.
            value=double_count
        />
    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}