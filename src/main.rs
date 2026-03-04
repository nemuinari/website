use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Currently under construction." }</h1>
            <p>{ "Nemuinari's homepage is built a Rust WebAssembly site and automated its deployment to AWS using GitHub Actions." }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
