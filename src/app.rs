use yew::prelude::*;

struct Item {
    id: usize,
    name: String,
    price: f32,
    quantity: i32,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
        <ul>
            <li><a href="/">{"Home"}</a></li>
            <li><a href="news">{"News"}</a></li>
            <li><a href="contact">{"Contact"}</a></li>
            <li><a href="about">{"About"}</a></li>
        </ul>
        </main>
    }
}
