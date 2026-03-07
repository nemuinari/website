mod base;
mod footer;
mod header;
mod style;

use base::Base;
use footer::Footer;
use header::Header;
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
