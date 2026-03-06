use crate::style::HEADER_HEIGHT;
use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;

#[styled_component(Header)]
pub fn header() -> Html {
    let is_menu_open = use_state(|| false);
    let is_loaded = use_state(|| false);

    use_effect_with((), {
        let is_loaded = is_loaded.clone();
        move |_| {
            is_loaded.set(true);
            || ()
        }
    });

    let onclick_toggle = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| is_menu_open.set(!*is_menu_open))
    };

    let raw_css = include_str!("../style/header.css");
    let css_string = raw_css.replace("${h}", HEADER_HEIGHT);

    let sheet = css_string
        .parse::<stylist::ast::Sheet>()
        .expect("Failed to parse header.css");

    let style = Style::new(sheet).expect("Failed to create header style");

    html! {
        <header class={classes!(style, is_loaded.then_some("animate"))}>
            <div class="header-content">
                <div class="nav-logo">{ "N.N.Lab." }</div>
                <button class="nav-toggle" onclick={onclick_toggle}>{ "☰" }</button>
                <nav id="navMenu">
                    <ul class={classes!("nav-menu", is_menu_open.then_some("active"))}>
                        <li><a href="#">{ "Home" }</a></li>
                        <li><a href="#">{ "Blog" }</a></li>
                        <li><a href="#">{ "Profile" }</a></li>
                        <li><a href="#">{ "Works" }</a></li>
                        <li><a href="#">{ "Contact" }</a></li>
                    </ul>
                </nav>
            </div>
            <div class="header-visual-container">
                <img src="assets/header_1.png" class="layer main-bg" alt="Main Background" />
                <img src="assets/header_2.png" class="layer title-logo" alt="Title Logo" />
                <img src="assets/header_3.png" class="layer subtitle-logo" alt="Subtitle Logo" />
            </div>
        </header>
    }
}
