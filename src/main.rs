mod components;
mod style;

use components::base::Base;
use components::footer::Footer;
use components::header::Header;
use style::GlobalStyle;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <GlobalStyle />
            <Header />
            <Base />
            <Footer />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
