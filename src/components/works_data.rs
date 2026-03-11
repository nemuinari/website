pub struct WorkItem {
    pub id: u32,
    pub title: String,
    pub url: String,
    pub img_path: String,
}

pub fn get_works_data() -> Vec<WorkItem> {
    vec![
        WorkItem {
            id: 1,
            title: "RTM - Rust timer".to_string(),
            url: "https://github.com/nemuinari/rtm-rust-timer".to_string(),
            img_path: "assets/rtm-icon.png".to_string(),
        },
        /*
        WorkItem {
            id: 2,
            title: "Project Two".to_string(),
            url: "#".to_string(),
            img_path: "".to_string(),
        },
        WorkItem {
            id: 3,
            title: "Project Three".to_string(),
            url: "#".to_string(),
            img_path: "".to_string(),
        },
        WorkItem {
            id: 4,
            title: "Project Four".to_string(),
            url: "#".to_string(),
            img_path: "".to_string(),
        },
        WorkItem {
            id: 5,
            title: "Project Five".to_string(),
            url: "#".to_string(),
            img_path: "".to_string(),
        },
        WorkItem {
            id: 6,
            title: "Project Six".to_string(),
            url: "#".to_string(),
            img_path: "".to_string(),
        },
        */
    ]
}
