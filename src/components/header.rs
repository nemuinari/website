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

    // アニメーション発火用のクラス
    let animation_class = if *is_loaded { "animate" } else { "" };

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

            /* 画像コンテナ */
            <div class={classes!("header-visual-container", animation_class)}>
                /* 1. メイン画像 (最背面) */
                <img src="assets/header_1.png" class="layer main-bg" alt="Main Background" />

                /* 2. タイトルロゴ */
                <img src="assets/header_2.png" class="layer title-logo" alt="Title Logo" />

                /* 3. サブタイトルロゴ */
                <img src="assets/header_3.png" class="layer subtitle-logo" alt="Subtitle Logo" />
            </div>
        </header>
    }
}
