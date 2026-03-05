mod components;

use components::footer::Footer;
use components::header::Header;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Header />

            <main style="min-height: 50vh; padding: 2rem;">
                <div class="content-body">
                    <h2>{ "Currently under construction." }</h2>
                    <p>{ "Nemuinari's homepage is built with a Rust WebAssembly site." }</p>
                </div>
            </main>

            <Footer />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
// this update is 03051709
