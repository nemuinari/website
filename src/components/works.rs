use crate::components::works_data::get_works_data;
use crate::css::get_app_style;
use yew::prelude::*;

#[function_component(Works)]
pub fn works() -> Html {
    let show_all = use_state(|| false);
    let works_data = use_memo((), |_| get_works_data());

    let display_count = if *show_all { works_data.len() } else { 3 };
    let visible_works = works_data.iter().take(display_count);

    let on_toggle_view = {
        let show_all = show_all.clone();
        Callback::from(move |_| show_all.set(!*show_all))
    };

    let style = get_app_style();
    html! {
        <main class={classes!(style, "works-layout")}>
            <div class="content-body">
                <h2 id="works" class="section-title">{ "Works" }</h2>

                <div class="works-grid">
                    { for visible_works.map(|work| {
                        html! {
                            <a key={work.id} href={work.url.clone()} class="work-card">
                                <div class="square-box">
                                    if !work.img_path.is_empty() {
                                        <img src={work.img_path.clone()} alt={work.title.clone()} class="work-icon" />
                                    } else {
                                        <div class="placeholder-icon"></div>
                                    }
                                </div>
                                <p class="work-title">{ &work.title }</p>
                            </a>
                        }
                    }) }
                </div>

                <div class="button-container">
                    <button onclick={on_toggle_view} class="more-button">
                        if *show_all { { "Show Less" } } else { { "View More" } }
                    </button>
                </div>
            </div>
        </main>
    }
}
