use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    // ナビゲーションメニューの開閉状態を管理するステート
    let is_menu_open = use_state(|| false);
    // ヘッダー画像の読み込み状態を管理するステート
    let is_loaded = use_state(|| false);

    // コンポーネントが表示されたら実行される
    {
        let is_loaded = is_loaded.clone();
        use_effect_with((), move |_| {
            is_loaded.set(true);
            || () // クリーンアップ
        });
    }

    let header_image_class = if *is_loaded {
        "header-image loaded"
    } else {
        "header-image"
    };

    let onclick_toggle = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| {
            is_menu_open.set(!*is_menu_open);
        })
    };

    let nav_menu_class = if *is_menu_open {
        "nav-menu active"
    } else {
        "nav-menu"
    };

    html! {
        <>
            <header>
                <div class="header-content">
                    <div class="nav-logo">{ "N.N.Lab." }</div>

                    <button
                        id="navToggle"
                        class="nav-toggle"
                        aria-controls="navMenu"
                        aria-expanded={is_menu_open.to_string()}
                        aria-label="Open navigation menu"
                        onclick={onclick_toggle}
                    >
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
                // ヘッダー画像
                <div class={header_image_class}>
                    <img src="/assets/header_1.png" alt="header images" />
                </div>
            </header>

            <main style="min-height: 50vh; padding: 2rem;">
                <div class="content-body">
                    <h2>{ "Currently under construction." }</h2>
                    <p>{ "Nemuinari's homepage is built with a Rust WebAssembly site." }</p>
                </div>
            </main>

            <footer>
                <div class="footer-content">
                    <p>{ "© Nemui Nari's WebSite 2025." }</p>
                </div>
            </footer>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
