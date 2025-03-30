use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <div>
            <h1>{ "Hello, Yew!" }</h1>
            <p>{ "This is a simple Yew application." }</p>
        </div>
        <div>
            <h2>{ "Yew is a modern Rust framework for creating web applications." }</h2>
            <p>{ "It allows you to build fast and reliable web apps using Rust." }</p>
        </div>
        </>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}