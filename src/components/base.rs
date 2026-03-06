use stylist::Style;
use yew::prelude::*;

#[function_component(Base)]
pub fn base() -> Html {
    let sheet = include_str!("../style/base.css")
        .parse::<stylist::ast::Sheet>()
        .expect("Failed to parse base.css");

    let layout_style = Style::new(sheet).expect("Failed to create layout style");

    html! {
        <main class={classes!(layout_style)}>
            <div class="content-body">
                <section class="under-construction">
                    <h2>{ "Currently under construction." }</h2>
                    <p>{ "Nemuinari's homepage is built with a Rust WebAssembly site." }</p>
                </section>
            </div>
        </main>
    }
}
