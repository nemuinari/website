use stylist::StyleSource;
use stylist::ast::Sheet;
use stylist::yew::Global;
use yew::prelude::*;

/* --- Header --- */
pub struct HeaderAssets {
    pub base: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
}
pub const HEADER_IMAGES: HeaderAssets = HeaderAssets {
    base: "assets/header_1.png",
    title: "assets/header_2.png",
    subtitle: "assets/header_3.png",
};

/* --- Footer --- */
pub struct FooterConfig {
    pub copyright: &'static str,
}
pub const FOOTER_CONFIG: FooterConfig = FooterConfig {
    copyright: "© Nemui Nari's WebSite 2025.",
};

/* --- get_app_style --- */
const CSS_RAW: &str = include_str!("style.css");
pub fn get_app_style() -> StyleSource {
    let sheet = CSS_RAW.parse::<Sheet>().expect("Failed to parse style.css");
    StyleSource::from(sheet)
}

#[function_component(GlobalStyle)]
pub fn global_style() -> Html {
    let style = get_app_style();
    html! { <Global css={style} /> }
}
