pub struct DownloadLink {
    pub label: &'static str,
    pub url: &'static str,
}

pub struct WorkDoc {
    pub description: &'static str,
    pub features: &'static [&'static str],
    pub install_steps: &'static [&'static str],
    pub stack: &'static [&'static str],
    pub download_links: &'static [DownloadLink],
}

pub struct WorkItem {
    pub id: u32,
    pub title: String,
    pub url: String,
    pub img_path: String,
    pub doc: Option<WorkDoc>,
}

pub fn get_works_data() -> Vec<WorkItem> {
    vec![
        WorkItem {
            id: 1,
            title: "RTM - Rust Timer".to_string(),
            url: "https://github.com/nemuinari/rtm-rust-timer".to_string(),
            img_path: "assets/rtm-icon.png".to_string(),
            doc: Some(WorkDoc {
                description: "
                Windows 向けデスクトップタイマー
                ターミナルから $ rtm で起動可能
                キーボード操作に対応しており、.exe から直接起動することもできます",
                features: &[
                    "[S] — タイマーの開始 / 停止 (START / STOP)",
                    "[R] — タイマーのリセット (RESET)",
                    "[Space] — ウィンドウの最小化",
                    "[Esc] — アプリケーションの終了",
                    "[Win + (Alt) + 矢印] — ウィンドウ移動（Windows 標準機能）",
                ],
                install_steps: &[
                    "# MSI インストーラーをダウンロードして実行",
                    "# インストール後はターミナルから起動",
                    "$ rtm",
                ],
                stack: &["Rust", "Windows", "MSI", "egui / tauri"],
                download_links: &[DownloadLink {
                    label: "Windows installer (.msi)",
                    url: "https://github.com/nemuinari/rtm-rust-timer/releases/download/v0.1.1/rtm-rust-timer-0.1.1-x86_64.msi",
                }],
            }),
        },
        /*
        WorkItem {
            id: 2,
            title: "Project Two".to_string(),
            url: "#".to_string(),
            img_path: "".to_string(),
            doc: None,
        },
        */
    ]
}
