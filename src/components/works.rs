use crate::components::works_data::get_works_data;
use js_sys::Reflect;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::{Element, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};
use yew::prelude::*;

#[function_component(Works)]
pub fn works() -> Html {
    let show_all = use_state(|| false);
    let is_visible = use_state(|| false);
    let container_ref = use_node_ref();

    let works_data = use_memo((), |_| get_works_data());
    let display_count = if *show_all { works_data.len() } else { 3 };
    let visible_works = works_data.iter().take(display_count);

    // Intersection Observer
    {
        let is_visible = is_visible.clone();
        use_effect_with(container_ref.clone(), move |container_ref| {
            let mut observer = None;

            if let Some(element) = container_ref.cast::<Element>() {
                let closure = Closure::wrap(Box::new(move |entries: js_sys::Array| {
                    if entries.iter().any(|entry| {
                        entry
                            .unchecked_into::<IntersectionObserverEntry>()
                            .is_intersecting()
                    }) {
                        is_visible.set(true);
                    }
                }) as Box<dyn FnMut(js_sys::Array)>);

                let options = IntersectionObserverInit::new();

                let _ = Reflect::set(&options, &"rootMargin".into(), &"0px 0px -150px 0px".into());
                let _ = Reflect::set(
                    &options,
                    &"threshold".into(),
                    &js_sys::Array::of1(&0.1.into()),
                );

                if let Ok(obs) = IntersectionObserver::new_with_options(
                    closure.as_ref().unchecked_ref(),
                    &options,
                ) {
                    obs.observe(&element);
                    observer = Some((obs, closure));
                }
            }

            move || {
                if let Some((obs, _)) = observer {
                    obs.disconnect();
                }
            }
        });
    }

    let on_toggle_view = {
        let show_all = show_all.clone();
        Callback::from(move |_| show_all.set(!*show_all))
    };

    html! {
        <main
            ref={container_ref}
            class={classes!("works-layout", if *is_visible { "animate" } else { "" })}
        >
            <div class="works-content-inner">
                <h2 id="works" class="section-title-works">{ "Works" }</h2>

                <div class="works-grid">
                    { for visible_works.enumerate().map(|(i, work)| {
                        html! {
                            <a key={work.id} href={work.url.clone()} class="work-card" style={format!("--i: {};", i)}>
                                <div class="work-thumbnail-container">
                                    if !work.img_path.is_empty() {
                                        <img src={work.img_path.clone()} alt={work.title.clone()} class="work-img" />
                                    } else {
                                        <div class="work-placeholder"></div>
                                    }
                                </div>
                                <p class="work-item-name">{ &work.title }</p>
                            </a>
                        }
                    }) }
                </div>

                <div class="works-button-container">
                    <button onclick={on_toggle_view} class="works-more-button">
                        if *show_all { { "Show Less" } } else { { "View More" } }
                    </button>
                </div>
            </div>
        </main>
    }
}
