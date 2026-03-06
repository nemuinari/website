use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;

#[styled_component(Footer)]
pub fn footer() -> Html {
    let sheet = include_str!("../style/footer.css")
        .parse::<stylist::ast::Sheet>()
        .expect("Failed to parse footer.css");
    let style = Style::new(sheet).expect("Failed to create style for footer");

    html! {
        <footer class={classes!(style)}>
            <div class="footer-content">
                <p>{ "© Nemui Nari's WebSite 2025." }</p>
            </div>
        </footer>
    }
}
