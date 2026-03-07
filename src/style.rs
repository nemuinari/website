use stylist::css;
use stylist::yew::Global;
use yew::prelude::*;

pub const HEADER_HEIGHT: &str = "64px";

// header images
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

#[function_component(GlobalStyle)]
pub fn global_style() -> Html {
    let style = css!(
        r#"
        * {
            box-sizing: border-box;
        }

        body {
            font-family: "Noto Sans JP", sans-serif;
            margin: 0;
            background: #fff;
            padding-top: 0; 
        }

        h1, h2, h3 {
            margin: 0;
        }

        a {
            text-decoration: none;
            color: inherit;
        }
        "#
    );

    html! {
        <Global css={style} />
    }
}
