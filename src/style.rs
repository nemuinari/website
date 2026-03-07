use stylist::css;
use stylist::yew::Global;
use yew::prelude::*;

// サイト全体の定数
pub const HEADER_HEIGHT: &str = "64px";
pub const PRIMARY_TEXT_COLOR: &str = "#333";
pub const BORDER_COLOR: &str = "#ddd";

// Header用アセット
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

// Footer用データ
pub struct FooterConfig {
    pub copyright: &'static str,
}
pub const FOOTER_CONFIG: FooterConfig = FooterConfig {
    copyright: "© Nemui Nari's WebSite 2025.",
};

#[function_component(GlobalStyle)]
pub fn global_style() -> Html {
    let style = css!(
        r#"
        * { box-sizing: border-box; }

        body {
            font-family: "Noto Sans JP", sans-serif;
            margin: 0;
            background: #fff;
            padding-top: 0; 
            color: ${text_color};
        }

        h1, h2, h3 { margin: 0; }
        a { text-decoration: none; color: inherit; }
        "#,
        text_color = PRIMARY_TEXT_COLOR
    );

    html! { <Global css={style} /> }
}
