use crate::css::{HEADER_IMAGES, get_app_style};
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
        href: "#",
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

#[function_component(Header)]
pub fn header() -> Html {
    let is_menu_open = use_state(|| false);
    let is_loaded = use_state(|| false);

    {
        let is_loaded = is_loaded.clone();
        use_effect_with((), move |_| {
            is_loaded.set(true);
            || ()
        });
    }

    let onclick_toggle = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| is_menu_open.set(!*is_menu_open))
    };

    let style = get_app_style();
    html! {
        <header class={classes!(style, "header-main", is_loaded.then_some("animate"))}>
            <div class="header-content">
                <div class="nav-logo">{ "N.N.Lab." }</div>
                <button class="nav-toggle" onclick={onclick_toggle}>{ "☰" }</button>
                <nav id="navMenu">
                    <ul class={classes!("nav-menu", is_menu_open.then_some("active"))}>
                        { for NAV_ITEMS.iter().map(|item| html! {
                            <li><a href={item.href}>{ item.name }</a></li>
                        }) }
                    </ul>
                </nav>
            </div>

            <div class="header-visual-container">
                <img src={HEADER_IMAGES.base} class="layer main-bg" alt="Main Background" />
                <img src={HEADER_IMAGES.title} class="layer title-logo" alt="Title Logo" />
                <img src={HEADER_IMAGES.subtitle} class="layer subtitle-logo" alt="Subtitle Logo" />
            </div>
        </header>
    }
}
