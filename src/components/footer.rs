use crate::css::get_app_style;
use yew::prelude::*;

/* Footer */
struct FooterConfig {
    pub copyright: &'static str,
}
const FOOTER_CONFIG: FooterConfig = FooterConfig {
    copyright: "© Nemui Nari's WebSite 2026.",
};

#[function_component(Footer)]
pub fn footer() -> Html {
    let style = get_app_style();
    html! {
        <footer class={classes!(style, "footer-main")}>
            <div class="footer-content">
                <p>{ FOOTER_CONFIG.copyright }</p>
            </div>
        </footer>
    }
}
