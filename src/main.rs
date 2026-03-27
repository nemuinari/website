mod components;
mod css;

use components::arts::Arts;
use components::blog::Blog;
use components::footer::Footer;
use components::header::Header;
use components::profile::Profile;
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
            <Blog />
            <Arts />
            <Works />
            <Profile />
            <Test />
            <Footer />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
