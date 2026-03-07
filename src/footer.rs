use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Footer)]
pub fn footer() -> Html {
    let style = css!(
        r#"
        text-align: center;
        padding: 1rem;
        background: #fff;
        border-top: 1.2px solid #ddd;

        .footer-content p {
            margin: 0;
            color: #333;
            font-size: 0.9rem;
        }
        "#
    );

    html! {
        <footer class={style}>
            <div class="footer-content">
                <p>{ "© Nemui Nari's WebSite 2025." }</p>
            </div>
        </footer>
    }
}
