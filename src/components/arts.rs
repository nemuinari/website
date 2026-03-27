use crate::css::get_app_style;
use js_sys::Reflect;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::{Element, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};
use yew::prelude::*;

struct ArtEntry {
    thumbnail: &'static str,
    full: &'static str,
}

const INITIAL_DISPLAY: usize = 6;

const ARTS_IMAGES: &[ArtEntry] = &[ArtEntry {
    thumbnail: "assets/arts/251029_1.png",
    full: "assets/arts/251029_2.png",
}];

// ─── Modal ───────────────────────────────────────────────────────────────────

#[derive(Properties, PartialEq)]
struct ArtModalProps {
    pub src: String,
    pub on_close: Callback<()>,
}

#[function_component(ArtModal)]
fn art_modal(props: &ArtModalProps) -> Html {
    let style = get_app_style();
    let on_close = props.on_close.clone();

    let on_overlay_click = {
        let on_close = on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    let on_inner_click = Callback::from(|e: MouseEvent| {
        e.stop_propagation();
    });

    // Esc キーで閉じる
    {
        let on_close = on_close.clone();
        use_effect_with((), move |_| {
            let closure = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                if e.key() == "Escape" {
                    on_close.emit(());
                }
            }) as Box<dyn FnMut(_)>);
            web_sys::window()
                .unwrap()
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
                .ok();
            closure.forget();
        });
    }

    html! {
        <div class={classes!(style.clone(), "art-modal-overlay")} onclick={on_overlay_click}>
            <div class={classes!(style.clone(), "art-modal-panel")} onclick={on_inner_click}>
                <button
                    class="art-modal-close-btn"
                    onclick={{
                        let on_close = props.on_close.clone();
                        Callback::from(move |_| on_close.emit(()))
                    }}
                >{ "×" }</button>
                <img src={props.src.clone()} alt="art" class="art-modal-img" />
            </div>
        </div>
    }
}

// ─── Arts ────────────────────────────────────────────────────────────────────

#[function_component(Arts)]
pub fn arts() -> Html {
    let show_all = use_state(|| false);
    let is_visible = use_state(|| false);
    let animate_key = use_state(|| 0u32);
    let container_ref = use_node_ref();
    let selected_src: UseStateHandle<Option<String>> = use_state(|| None);

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
        let animate_key = animate_key.clone();
        Callback::from(move |_| {
            show_all.set(!*show_all);
            animate_key.set(*animate_key + 1);
        })
    };

    let display_count = if *show_all { ARTS_IMAGES.len() } else { INITIAL_DISPLAY };

    let animate_class = if *is_visible {
        format!("animate animate-key-{}", *animate_key)
    } else {
        String::new()
    };

    let modal_html = (*selected_src).as_ref().map(|full_src| {
        let on_close = {
            let selected_src = selected_src.clone();
            Callback::from(move |_| selected_src.set(None))
        };
        html! {
            <ArtModal src={full_src.clone()} on_close={on_close} />
        }
    });

    html! {
        <>
            { modal_html.unwrap_or_default() }

            <section
                ref={container_ref}
                class={classes!("arts-layout", animate_class)}
            >
                <div class="arts-content-inner">
                    <h2 id="arts" class="section-title-arts">{ "Arts" }</h2>

                    <div class="arts-grid">
                        { for ARTS_IMAGES.iter().take(display_count).enumerate().map(|(i, entry)| {
                            let selected_src = selected_src.clone();
                            let full = entry.full.to_string();
                            let animate_key_val = *animate_key;
                            let onclick = Callback::from(move |_| {
                                selected_src.set(Some(full.clone()));
                            });
                            html! {
                                <div
                                    key={format!("{}-{}", entry.thumbnail, animate_key_val)}
                                    class="art-card"
                                    style={format!("--i: {};", i)}
                                    onclick={onclick}
                                >
                                    <div class="art-thumbnail-container">
                                        <img src={entry.thumbnail} alt="art" class="art-img" />
                                    </div>
                                </div>
                            }
                        }) }
                    </div>

                    <div class="arts-button-container">
                        <button onclick={on_toggle_view} class="arts-more-button">
                            if *show_all { { "Show Less" } } else { { "View More" } }
                        </button>
                    </div>
                </div>
            </section>
        </>
    }
}
