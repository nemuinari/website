use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};
use yew::prelude::*;

struct NavItem {
    name: &'static str,
    href: &'static str,
}

const NAV_ITEMS: &[NavItem] = &[
    NavItem {
        name: "Home",
        href: "#",
    },
    NavItem {
        name: "Blog",
        href: "#",
    },
    NavItem {
        name: "Profile",
        href: "#profile",
    },
    NavItem {
        name: "Works",
        href: "#works",
    },
    NavItem {
        name: "Contact",
        href: "#",
    },
];

struct HeaderItem {
    base: &'static str,
    title: &'static str,
    subtitle: &'static str,
}

const HEADER_ITEMS: HeaderItem = HeaderItem {
    base: "assets/header_1.png",
    title: "assets/header_2.png",
    subtitle: "assets/header_3.png",
};

#[function_component(Header)]
pub fn header() -> Html {
    let is_menu_open = use_state(|| false);
    let is_visible = use_state(|| false);
    let hero_ref = use_node_ref();

    // Intersection Observer for Hero Visual
    {
        let is_visible = is_visible.clone();
        let hero_ref = hero_ref.clone();
        use_effect_with(hero_ref.clone(), move |hero_ref| {
            let mut observer = None;
            if let Some(element) = hero_ref.cast::<web_sys::Element>() {
                let is_visible = is_visible.clone();
                let closure = Closure::wrap(Box::new(move |entries: js_sys::Array| {
                    for entry in entries.iter() {
                        let entry = entry.unchecked_into::<IntersectionObserverEntry>();
                        if entry.is_intersecting() {
                            is_visible.set(true);
                        }
                    }
                }) as Box<dyn FnMut(js_sys::Array)>);

                let options = IntersectionObserverInit::new();
                let obs = IntersectionObserver::new_with_options(
                    closure.as_ref().unchecked_ref(),
                    &options,
                )
                .unwrap();
                obs.observe(&element);
                observer = Some((obs, closure));
            }
            move || {
                if let Some((obs, _)) = observer {
                    obs.disconnect();
                }
            }
        });
    }

    let onclick_toggle = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| is_menu_open.set(!*is_menu_open))
    };

    html! {
        <header class="header-main">
            <div class="nav-bar">
                <div class="nav-logo">{ "N.N.Lab." }</div>
                <nav class="nav-container">
                    <button class="nav-toggle-btn" onclick={onclick_toggle}>{ "☰" }</button>
                    <ul class={classes!("nav-menu-list", if *is_menu_open { "active" } else { "" })}>
                        { for NAV_ITEMS.iter().map(|item| html! {
                            <li><a href={item.href} class="nav-link">{ item.name }</a></li>
                        }) }
                    </ul>
                </nav>
            </div>
            <div ref={hero_ref} class={classes!("hero-visual-area", if *is_visible { "animate" } else { "" })}
            >
                <img src={HEADER_ITEMS.base} class="hero-layer hero-bg" alt="Main Background" />
                <img src={HEADER_ITEMS.title} class="hero-layer hero-logo-title" alt="Title Logo" />
                <img src={HEADER_ITEMS.subtitle} class="hero-layer hero-logo-subtitle" alt="Subtitle Logo" />
            </div>
        </header>
    }
}
