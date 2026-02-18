use yew::prelude::*;
mod pages;
mod components;
// yew component start
#[function_component(App)]
fn app() -> Html {
    html! {
        <crate::pages::login::Login />
     }
}

// yew component finish

// component prompt start
// component prompt finish
fn main() {
    yew::Renderer::<App>::new().render();
}
