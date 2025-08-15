use leptos::prelude::*;

/// Shows progress toward a goal.
#[component]
fn ProgressBar<F>(
    /// How much progress should be displayed.
    progress: F,
    /// The maximum value of the progress bar.
    #[prop(default = 20)]
    max: u16,
) -> impl IntoView
where
    F: Fn() -> i32 + Send + Sync + 'static,
{
    view! { <progress max=max value=progress /> }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;

    view! {
        <button on:click=move |_| *set_count.write() += 1>"Click me"</button>
        <ProgressBar progress=count />
        <ProgressBar progress=double_count max=10 />
    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}
