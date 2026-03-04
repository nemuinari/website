use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Hello, Rust Wasm!" }</h1>
            <p>{ "This is my page." }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
