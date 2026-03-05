use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <div class="footer-content">
                <p>{ "© Nemui Nari's WebSite 2025." }</p>
            </div>
        </footer>
    }
}
