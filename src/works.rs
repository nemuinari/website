use crate::components::works_data::get_works_data;
use crate::style::{BORDER_COLOR, PRIMARY_TEXT_COLOR};
use stylist::css;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Works)]
pub fn works() -> Html {
    let layout_style = get_works_style();
    let show_all = use_state(|| false);

    // works_data
    let works_data = use_memo((), |_| get_works_data());

    let display_count = if *show_all { works_data.len() } else { 3 };
    let visible_works = works_data.iter().take(display_count);

    let on_toggle_view = {
        let show_all = show_all.clone();
        Callback::from(move |_| show_all.set(!*show_all))
    };

    html! {
        <main class={layout_style}>
            <div class="content-body">
                <h2 id="works" class="section-title">{ "Works" }</h2>

                <div class="works-grid">
                    { for visible_works.map(|work| {
                        html! {
                            <a key={work.id} href={work.url.clone()} class="work-card">
                                <div class="square-box">
                                    if !work.img_path.is_empty() {
                                        <img src={work.img_path.clone()} alt={work.title.clone()} class="work-icon" />
                                    } else {
                                        <div class="placeholder-icon"></div>
                                    }
                                </div>
                                <p class="work-title">{ &work.title }</p>
                            </a>
                        }
                    }) }
                </div>

                <div class="button-container">
                    <button onclick={on_toggle_view} class="more-button">
                        if *show_all { { "Show Less" } } else { { "View More" } }
                    </button>
                </div>
            </div>
        </main>
    }
}

fn get_works_style() -> stylist::StyleSource {
    css!(
        r#"
        padding: 2rem;
        min-height: 70vh;
        max-width: 1100px;
        margin: 0 auto;

        .content-body {
            width: 100%;
            animation: fadeInLefty 1.5s ease-out forwards;
        }

        @keyframes fadeInLefty {
            0% {
                opacity: 0;
                transform: translateX(-20px); 
            }
            100% {
                opacity: 1;
                transform: translateX(0);    
            }
        } 

        .section-title {
            color: ${title_color};
            font-size: 2rem;
            margin-bottom: 3rem;
            text-align: left;
            font-weight: 300;
            letter-spacing: 0.1em;
            animation: fadeInLefty 1.5s ease-out forwards;
        }

        .works-grid {
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            column-gap: clamp(1rem, 5vw, 5rem);
            row-gap: 4rem;
            margin-bottom: 4rem;
        }

        .work-card {
            text-decoration: none;
            display: flex;
            flex-direction: column;
            transition: transform 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94);
        }

        .work-card:hover {
            transform: translateY(-4px) scale(1.02);
        }

        .square-box {
            width: 100%;
            aspect-ratio: 1 / 1;
            border: 1px solid ${border};
            box-sizing: border-box;
            display: flex;
            align-items: center;
            justify-content: center;
            background-color: #fff;
            overflow: hidden; 
            transition: box-shadow 0.3s ease;
            padding: 20%; 
        }

        .work-card:hover .square-box {
            box-shadow: 0 10px 15px rgba(0,0,0,0.08);
        }

        /* アイコン画像のリサイズ設定 */
        .work-icon {
            width: 100%;
            height: 100%;
            object-fit: contain; 
            display: block;
        }

        .placeholder-icon {
            width: 100%;
            height: 100%;
            background-color: #f5f5f5;
            border-radius: 50%;
        }

        .work-title {
            margin-top: 1.2rem;
            font-size: clamp(0.7rem, 2vw, 0.95rem);
            font-weight: 500;
            color: ${title_color};
            text-align: center;
            letter-spacing: 0.05em;
        }

        .button-container {
            display: flex;
            justify-content: center;
            margin-top: 3rem;  
            padding: 0 1rem;   
        }

        .more-button {
            padding: 0.8rem 3rem; 
            background-color: transparent;
            border: 1px solid ${border};
            color: ${title_color};
            cursor: pointer;
            font-size: 0.85rem;
            letter-spacing: 0.1em;
            transition: all 0.2s ease;
            outline: none;
            -webkit-tap-highlight-color: transparent; 
        }

        /* デスクトップ用 */
        @media (min-width: 769px) {
            .more-button:hover {
                background-color: ${title_color};
                color: #fff;
            }
        }

        /* レスポンシブ */
        @media (max-width: 768px) {
            .button-container {
                margin-top: 2rem;
            }
            .more-button {
                width: 100%;       
                max-width: 300px;  
                font-size: 0.9rem; 
            }
            
            .more-button:active {
                background-color: rgba(0, 0, 0, 0.05);
                transform: scale(0.98);
            }
        }
        "#,
        border = BORDER_COLOR,
        title_color = PRIMARY_TEXT_COLOR
    )
}
