use js_sys::Reflect;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::{Element, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};
use yew::prelude::*;

struct ProfileImage {
    back: &'static str,
    front: &'static str,
}

struct ProfileEntry {
    label: &'static str,
    value: &'static str,
}

const PROFILE_IMAGES: ProfileImage = ProfileImage {
    back: "assets/profile_1.png",
    front: "assets/profile_2.png",
};

const PROFILE_DATA: &[ProfileEntry] = &[
    ProfileEntry {
        label: "Name",
        value: "Nemui Nari (Nora)",
    },
    ProfileEntry {
        label: "Title",
        value: "Master of my own time.",
    },
    ProfileEntry {
        label: "Manifesto",
        value: "The time you enjoy wasting is not wasted time.",
    },
    ProfileEntry {
        label: "Stack",
        value: "#Anime #Game #Art #Books #Nightcap",
    },
    ProfileEntry {
        label: "Origin",
        value: "Japan",
    },
];

#[function_component(Profile)]
pub fn profile() -> Html {
    let is_visible = use_state(|| false);
    let container_ref = use_node_ref();

    {
        let is_visible = is_visible.clone();
        use_effect_with(container_ref.clone(), move |container_ref| {
            let mut observer = None;

            if let Some(element) = container_ref.cast::<Element>() {
                // コールバック
                let closure = Closure::wrap(Box::new(move |entries: js_sys::Array| {
                    if entries.iter().any(|entry| {
                        entry
                            .unchecked_into::<IntersectionObserverEntry>()
                            .is_intersecting()
                    }) {
                        is_visible.set(true);
                    }
                }) as Box<dyn FnMut(js_sys::Array)>);

                // オプション設定
                let options = IntersectionObserverInit::new();
                let _ = Reflect::set(&options, &"rootMargin".into(), &"0px 0px -150px 0px".into());
                let _ = Reflect::set(
                    &options,
                    &"threshold".into(),
                    &js_sys::Array::of1(&0.1.into()),
                );

                // Observer初期化
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

    html! {
        <section
            ref={container_ref}
            class={classes!("prof-main", if *is_visible { "animate" } else { "" })}
        >
            <h2 id="profile" class="prof-title">{ "Profile" }</h2>

            <div class="prof-flex-container">
                <div class="prof-visual-container">
                    <img src={PROFILE_IMAGES.back} class="prof-layer prof-bg" alt="Background" />
                    <img src={PROFILE_IMAGES.front} class="prof-layer prof-img" alt="Frontimage" />
                </div>

                <div class="prof-text-container">
                    {
                        PROFILE_DATA.iter().enumerate().map(|(i, entry)| {
                            html! {
                                <div class="prof-entry" style={format!("--i: {};", i)}>
                                    <span class="prof-label">{ entry.label }</span>
                                    <div class="prof-value">{ entry.value }</div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
        </section>
    }
}
