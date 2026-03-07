use crate::style::{BORDER_COLOR, FOOTER_CONFIG, PRIMARY_TEXT_COLOR};
use stylist::css;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Footer)]
pub fn footer() -> Html {
    let style = get_footer_style();

    html! {
        <footer class={style}>
            <div class="footer-content">
                <p>{ FOOTER_CONFIG.copyright }</p>
            </div>
        </footer>
    }
}

fn get_footer_style() -> stylist::StyleSource {
    css!(
        r#"
        text-align: center;
        padding: 1rem;
        background: #fff;
        border-top: 1.2px solid ${border};

        .footer-content p {
            margin: 0;
            color: ${color};
            font-size: 0.9rem;
        }
        "#,
        border = BORDER_COLOR,
        color = PRIMARY_TEXT_COLOR
    )
}
