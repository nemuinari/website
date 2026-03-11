mod base;
mod components;
mod footer;
mod header;
mod style;
mod works;

use base::Base;
use footer::Footer;
use header::Header;
use style::GlobalStyle;
use works::Works;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <GlobalStyle />
            <Header />
            <Works />
            <Base />
            <Footer />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
