use gloo_net::http::Request;
use js_sys::Reflect;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};
use yew::prelude::*;

const ARTICLES_JSON: &str = "assets/articles.json";

#[derive(Deserialize, Clone, PartialEq)]
struct ArticlesJson {
    articles: Vec<Article>,
}

#[derive(Deserialize, Clone, PartialEq)]
struct Article {
    title: String,
    url: String,
    #[serde(default)]
    date: String,
    #[serde(default)]
    thumbnail: String,
}

const INITIAL_DISPLAY: usize = 3;

#[function_component(Blog)]
pub fn blog() -> Html {
    let show_all = use_state(|| false);
    let is_visible = use_state(|| false);
    let animate_key = use_state(|| 0u32);
    let container_ref = use_node_ref();
    let articles: UseStateHandle<Option<Vec<Article>>> = use_state(|| None);
    let fetch_error = use_state(|| false);

    // articles.json を fetch（マウント時に1回）
    {
        let articles = articles.clone();
        let fetch_error = fetch_error.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match Request::get(ARTICLES_JSON).send().await {
                    Ok(resp) => match resp.json::<ArticlesJson>().await {
                        Ok(data) => articles.set(Some(data.articles)),
                        Err(_) => fetch_error.set(true),
                    },
                    Err(_) => fetch_error.set(true),
                }
            });
        });
    }

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

    html! {
        <section
            ref={container_ref}
            class={classes!("blog-layout", animate_class)}
        >
            <div class="blog-content-inner">
                <h2 id="blog" class="section-title-blog">{ "Blog" }</h2>

                {
                    if *fetch_error {
                        html! {
                            <p class="blog-error">{ "記事の読み込みに失敗しました。" }</p>
                        }
                    } else if let Some(items) = (*articles).clone() {
                        let display_count = if *show_all {
                            items.len()
                        } else {
                            INITIAL_DISPLAY.min(items.len())
                        };
                        let has_more = items.len() > INITIAL_DISPLAY;
                        let visible = items.into_iter().take(display_count).collect::<Vec<_>>();
                        let key = *animate_key;

                        html! {
                            <>
                                <div class="blog-grid">
                                    { for visible.into_iter().enumerate().map(|(i, article)| {
                                        let has_thumb = !article.thumbnail.is_empty();
                                        html! {
                                            <a
                                                key={format!("{}-{}", article.url, key)}
                                                href={article.url.clone()}
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                class="blog-card"
                                                style={format!("--i: {};", i)}
                                            >
                                                <div class="blog-thumbnail-container">
                                                    if has_thumb {
                                                        <img
                                                            src={article.thumbnail.clone()}
                                                            alt={article.title.clone()}
                                                            class="blog-img"
                                                        />
                                                    } else {
                                                        <div class="blog-placeholder"></div>
                                                    }
                                                </div>
                                                <div class="blog-card-body">
                                                    <p class="blog-card-title">{ &article.title }</p>
                                                    <span class="blog-card-date">{ &article.date }</span>
                                                </div>
                                            </a>
                                        }
                                    }) }
                                </div>

                                if has_more {
                                    <div class="blog-button-container">
                                        <button onclick={on_toggle_view} class="blog-more-button">
                                            if *show_all { { "Show Less" } } else { { "View More" } }
                                        </button>
                                    </div>
                                }
                            </>
                        }
                    } else {
                        html! {
                            <div class="blog-loading">
                                <span class="blog-loading-dot"></span>
                                <span class="blog-loading-dot"></span>
                                <span class="blog-loading-dot"></span>
                            </div>
                        }
                    }
                }
            </div>
        </section>
    }
}
