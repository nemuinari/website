use crate::components::works_data::{WorkDoc, WorkItem, get_works_data};
use crate::css::get_app_style;
use js_sys::Reflect;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::{Element, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};
use yew::prelude::*;

// ─── PartialEq ───────────────────────────────────────────────────────────────

impl PartialEq for WorkItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq for WorkDoc {
    fn eq(&self, other: &Self) -> bool {
        self.description == other.description
    }
}

// ─── Modal Props ─────────────────────────────────────────────────────────────

#[derive(Properties, PartialEq)]
struct WorkModalProps {
    pub id: u32,
    pub title: String,
    pub url: String,
    pub img_path: String,
    pub doc: WorkDoc,
    pub on_close: Callback<()>,
}

// ─── Modal Component ─────────────────────────────────────────────────────────

#[function_component(WorkModal)]
fn work_modal(props: &WorkModalProps) -> Html {
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

    let doc = &props.doc;

    html! {
        <div class={classes!(style.clone(), "modal-overlay")} onclick={on_overlay_click}>
            <div class={classes!(style.clone(), "modal-panel")} onclick={on_inner_click}>

                // ── Header ──
                <div class="modal-header">
                    <div class="modal-header-left">
                        if !props.img_path.is_empty() {
                            <img src={props.img_path.clone()} alt={props.title.clone()} class="modal-icon" />
                        }
                        <h2 class="modal-title">{ &props.title }</h2>
                    </div>
                    <button class="modal-close-btn" onclick={{
                        let on_close = props.on_close.clone();
                        Callback::from(move |_| on_close.emit(()))
                    }}>{ "×" }</button>
                </div>

                // ── Body ──
                <div class="modal-body">

                    <p class="modal-description">{ doc.description }</p>

                    <section class="modal-section">
                        <h3 class="modal-section-title">{ "Features" }</h3>
                        <ul class="modal-feature-list">
                            { for doc.features.iter().map(|f| html! {
                                <li class="modal-feature-item">{ *f }</li>
                            }) }
                        </ul>
                    </section>

                    <section class="modal-section">
                        <h3 class="modal-section-title">{ "Install" }</h3>
                        <div class="modal-code-block">
                            { for doc.install_steps.iter().map(|step| html! {
                                <code class="modal-code-line">{ *step }</code>
                            }) }
                        </div>
                    </section>

                    <section class="modal-section">
                        <h3 class="modal-section-title">{ "Stack" }</h3>
                        <div class="modal-stack-badges">
                            { for doc.stack.iter().map(|s| html! {
                                <span class="modal-badge">{ *s }</span>
                            }) }
                        </div>
                    </section>

                </div>

                // ── Footer: Download ──
                <div class="modal-footer">
                    <p class="modal-dl-label">{ "Download" }</p>
                    <div class="modal-dl-buttons">
                        { for doc.download_links.iter().map(|dl| html! {
                            <a
                                href={dl.url}
                                target="_blank"
                                rel="noopener noreferrer"
                                class="modal-dl-btn"
                            >
                                <span class="modal-dl-icon">{ "↓" }</span>
                                { dl.label }
                            </a>
                        }) }
                    </div>
                    <a
                        href={props.url.clone()}
                        target="_blank"
                        rel="noopener noreferrer"
                        class="modal-github-link"
                    >
                        { "View on GitHub →" }
                    </a>
                </div>

            </div>
        </div>
    }
}

// ─── Works ───────────────────────────────────────────────────────────────────

#[function_component(Works)]
pub fn works() -> Html {
    let show_all = use_state(|| false);
    let is_visible = use_state(|| false);
    let animate_key = use_state(|| 0u32);
    let container_ref = use_node_ref();
    let selected_id: UseStateHandle<Option<u32>> = use_state(|| None);

    let works_data = use_memo((), |_| get_works_data());
    let display_count = if *show_all { works_data.len() } else { 3 };

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

    let animate_class = if *is_visible {
        format!("animate animate-key-{}", *animate_key)
    } else {
        String::new()
    };

    let modal_html = (*selected_id).and_then(|id| {
        works_data.iter().find(|w| w.id == id).and_then(|w| {
            w.doc.as_ref().map(|doc| {
                let on_close = {
                    let selected_id = selected_id.clone();
                    Callback::from(move |_| selected_id.set(None))
                };
                html! {
                    <WorkModal
                        id={w.id}
                        title={w.title.clone()}
                        url={w.url.clone()}
                        img_path={w.img_path.clone()}
                        doc={WorkDoc {
                            description: doc.description,
                            features: doc.features,
                            install_steps: doc.install_steps,
                            stack: doc.stack,
                            download_links: doc.download_links,
                        }}
                        on_close={on_close}
                    />
                }
            })
        })
    });

    html! {
        <>
            { modal_html.unwrap_or_default() }

            <main
                ref={container_ref}
                class={classes!("works-layout", animate_class)}
            >
                <div class="works-content-inner">
                    <h2 id="works" class="section-title-works">{ "Works" }</h2>

                    <div class="works-grid">
                        { for works_data.iter().take(display_count).enumerate().map(|(i, work)| {
                            let has_doc = work.doc.is_some();
                            let work_id = work.id;
                            let work_url = work.url.clone();
                            let animate_key_val = *animate_key;

                            let card_inner = html! {
                                <>
                                    <div class="work-thumbnail-container">
                                        if !work.img_path.is_empty() {
                                            <img
                                                src={work.img_path.clone()}
                                                alt={work.title.clone()}
                                                class="work-img"
                                            />
                                        } else {
                                            <div class="work-placeholder"></div>
                                        }
                                    </div>
                                    <p class="work-item-name">{ &work.title }</p>
                                </>
                            };

                            if has_doc {
                                let selected_id = selected_id.clone();
                                let onclick = Callback::from(move |e: MouseEvent| {
                                    e.prevent_default();
                                    selected_id.set(Some(work_id));
                                });
                                html! {
                                    <a
                                        key={format!("{}-{}", work_id, animate_key_val)}
                                        href="#"
                                        class="work-card"
                                        style={format!("--i: {};", i)}
                                        onclick={onclick}
                                    >
                                        { card_inner }
                                    </a>
                                }
                            } else {
                                html! {
                                    <a
                                        key={format!("{}-{}", work_id, animate_key_val)}
                                        href={work_url}
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="work-card"
                                        style={format!("--i: {};", i)}
                                    >
                                        { card_inner }
                                    </a>
                                }
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
        </>
    }
}
