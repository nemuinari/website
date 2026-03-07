use crate::style::{HEADER_HEIGHT, HEADER_IMAGES};
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Header)]
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

    let style = css!(
        r#"
        display: block;

        .header-content {
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            z-index: 1000;
            height: ${h};
            display: flex;
            align-items: center;
            padding: 0 1rem;
            background: #fff;
            border-bottom: 1px solid #ddd;
            margin: 0;
        }

        .nav-logo {
            font-weight: 700;
            font-size: 1.4rem;
        }

        nav {
            margin-left: auto;
            display: flex;
            align-items: center;
        }

        .nav-toggle {
            display: none;
            padding: 0 0.5rem;
            background: transparent;
            border: none;
            font-size: 1.6rem;
            cursor: pointer;
        }

        .nav-menu {
            display: flex;
            margin: 0;
            padding: 0;
            list-style: none;
        }

        .nav-menu a {
            display: inline-flex;
            align-items: center;
            height: ${h}; 
            padding: 0 1rem;
            color: #000;
            text-decoration: none;
            font-size: 1.2rem;
            transition: background-color 0.15s ease;
        }

        .nav-menu a:hover {
            background: #f2f2f2;
        }

        .header-visual-container {
            position: relative;
            width: 100%;
            max-width: 1500px;
            aspect-ratio: 3 / 1;
            margin: 0 auto;
            overflow: hidden;
            background-color: #fff;
            line-height: 0;
            margin-top: ${h}; 
        }

        .layer {
            position: absolute;
            display: block;
            opacity: 0;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
        }

        .main-bg {
            z-index: 1;
            object-fit: cover;
        }

        .title-logo {
            z-index: 2;
            top: 20%;
            left: 4.5%;
            width: 45%;
            height: auto;
        }

        .subtitle-logo {
            z-index: 3;
            top: 60%;
            left: 4.5%;
            width: 45%;
            height: auto;
        }

        &.animate .layer {
            animation: slideInFade 1.5s cubic-bezier(0.25, 0.45, 0.45, 0.95) forwards;
        }

        &.animate .main-bg {
            animation-delay: 0.2s;
        }

        &.animate .title-logo {
            animation-delay: 0.8s;
        }

        &.animate .subtitle-logo {
            animation-delay: 1.4s;
        }

        @keyframes slideInFade {
            from {
                opacity: 0;
                transform: translateX(-3vw);
            }
            to {
                opacity: 1;
                transform: translateX(0);
            }
        }

        /* 
        ============================================================
        Responsive Styles
        ============================================================
        */
        @media (max-width: 768px) {
            .nav-toggle {
                display: block;
                margin-left: auto;
            }

            nav {
                margin-left: 0;
            }

            .nav-menu {
                display: none;
                position: absolute; 
                top: ${h}; 
                left: 0;
                right: 0;
                z-index: 1001;
                flex-direction: column;
                background: #fff;
                border-top: 1px solid #ddd;
                align-items: stretch; 
            }

            .nav-menu.active {
                display: flex;
            }

            .nav-menu a {
                display: block;        
                width: 100%;          
                padding: 0.9rem 1rem;
                text-align: center;    
                border-bottom: 1px solid #eee;
                height: auto; 
                box-sizing: border-box; 
            }

            .title-logo {
                width: 45%;
                top: 20%;
            }
        }
        "#,
        h = HEADER_HEIGHT
    );

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
                <img src={HEADER_IMAGES.base} class="layer main-bg" alt="Main Background" />
                <img src={HEADER_IMAGES.title} class="layer title-logo" alt="Title Logo" />
                <img src={HEADER_IMAGES.subtitle} class="layer subtitle-logo" alt="Subtitle Logo" />
            </div>
        </header>
    }
}
