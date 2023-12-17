use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;
    let values = vec![0, 1, 2];
    view! {
        <button
            on:click=move |_| { set_count.update(|n| *n += 1) }
            class:red=move || count() % 2 == 1
        >
            "Click me: "
            {count}
        </button>

        <ProgressBar progress=count/>

        <ProgressBar progress=Signal::derive(double_count)/>

        <ul>{values.into_iter().map(|n| view! { <li>{n}</li> }).collect_view()}</ul>
    }
}
/// Shows progress toward a goal.
#[component]
fn ProgressBar(
    /// How much progress should be displayed.
    #[prop(into)]
    progress: Signal<i32>,
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
) -> impl IntoView {
    view! { <progress max=max value=progress></progress> }
}

