use stylist::yew::Global;
use yew::prelude::*;

pub const HEADER_HEIGHT: &str = "64px";

#[function_component(GlobalStyle)]
pub fn global_style() -> Html {
    let raw_css = include_str!("global.css");
    let css_string = raw_css.replace("${height}", HEADER_HEIGHT);

    let sheet = css_string
        .parse::<stylist::ast::Sheet>()
        .expect("Failed to parse global.css");

    html! {
        <Global css={sheet} />
    }
}
