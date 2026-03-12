mod components;
mod css;

use components::footer::Footer;
use components::header::Header;
use components::test::Test;
use components::works::Works;
use css::GlobalStyle;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <GlobalStyle />
            <Header />
            <Works />
            <Test />
            <Footer />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
