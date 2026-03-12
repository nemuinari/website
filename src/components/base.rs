use crate::css::get_app_style;
use yew::prelude::*;

#[function_component(Base)]
pub fn base() -> Html {
    let style = get_app_style();
    html! {
        <main class={classes!(style, "base-layout")}>
            <div class="content-body">
                <section class="under-construction">
                    <h2>{ "Currently under construction." }</h2>
                    <p>{ "Nemuinari's homepage is built with a Rust WebAssembly site." }</p>
                </section>
            </div>
        </main>
    }
}
