use crate::css::{FOOTER_CONFIG, get_app_style};
use yew::prelude::*;

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
