use yew::prelude::*;

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

    let nav_menu_class = if *is_menu_open {
        "nav-menu active"
    } else {
        "nav-menu"
    };
    let header_image_class = if *is_loaded {
        "header-image loaded"
    } else {
        "header-image"
    };

    html! {
        <header>
            <div class="header-content">
                <div class="nav-logo">{ "N.N.Lab." }</div>
                <button class="nav-toggle" onclick={onclick_toggle}>
                    { "☰" }
                </button>
                <nav id="navMenu">
                    <ul class={nav_menu_class}>
                        <li><a href="#">{ "Home" }</a></li>
                        <li><a href="#">{ "Profile" }</a></li>
                        <li><a href="#">{ "Works" }</a></li>
                        <li><a href="#">{ "Contact" }</a></li>
                    </ul>
                </nav>
            </div>
            <div class={header_image_class}>
                <img src="/assets/header_1.png" alt="header images" />
            </div>
        </header>
    }
}
