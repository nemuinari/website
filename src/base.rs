use crate::style::{BORDER_COLOR, PRIMARY_TEXT_COLOR};
use stylist::css;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Base)]
pub fn base() -> Html {
    let layout_style = get_base_style();

    html! {
        <main class={layout_style}>
            <div class="content-body">
                <section class="under-construction">
                    <h2>{ "Currently under construction." }</h2>
                    <p>{ "Nemuinari's homepage is built with a Rust WebAssembly site." }</p>
                </section>
            </div>
        </main>
    }
}

fn get_base_style() -> stylist::StyleSource {
    css!(
        r#"
        padding: 2rem;
        min-height: 70vh;
        max-width: 1200px;
        margin: 0 auto;

        .content-body {
            width: 100%;
        }

        .under-construction {
            margin-top: 3rem;
            padding: 3rem 1.5rem;
            background-color: #fff;
            border-radius: 12px;
            border: 1px solid ${border};
            text-align: center;
        }

        .under-construction h2 {
            color: ${title_color};
            margin-bottom: 1rem;
        }

        .under-construction p {
            color: #666;
        }
        "#,
        border = BORDER_COLOR,
        title_color = PRIMARY_TEXT_COLOR
    )
}
