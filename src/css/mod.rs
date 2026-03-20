use stylist::ast::Sheet;
use stylist::yew::Global;
use yew::prelude::*;

pub fn get_app_style() -> stylist::StyleSource {
    let full_css = concat!(
        include_str!("variable.css"),
        "\n",
        include_str!("base.css"),
        "\n",
        include_str!("header.css"),
        "\n",
        include_str!("blog.css"),
        "\n",
        include_str!("profile.css"),
        "\n",
        include_str!("works.css"),
        "\n",
        include_str!("test.css"),
        "\n",
        include_str!("footer.css")
    );

    let sheet = full_css
        .parse::<Sheet>()
        .expect("Failed to parse combined CSS");

    stylist::StyleSource::from(sheet)
}

#[function_component(GlobalStyle)]
pub fn global_style() -> Html {
    let style = get_app_style();
    html! { <Global css={style} /> }
}
