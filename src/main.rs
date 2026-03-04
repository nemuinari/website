use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Rust Wasm サイト、起動成功！" }</h1>
            <p>{ "GitHub Actions で自動デプロイされました。" }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
