use rustycrab_model::color::ColorResolvables;
use yew::{ Html, html, function_component, Properties, Callback, MouseEvent };

#[derive(Properties, PartialEq, Default)]
pub struct ButtonProps {
    #[prop_or_default]
    pub color: Option<ColorResolvables>,
    pub label: Option<String>,
    #[prop_or_default]
    pub icon: Option<String>, // assuming icon is represented as a string (e.g., class name or URL)
    #[prop_or_default]
    pub icon_position: Option<IconPosition>, // enum for icon position
    #[prop_or_default]
    pub style: String, // Add this line
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[derive(PartialEq)]
pub enum IconPosition {
    Start,
    End,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let base_style = if let Some(color) = &props.color {
        format!("background-color: #{:06x};", color.as_u32())
    } else {
        format!("background-color: transparent; color: #fff")
    };

    // Concatenate base style with the provided style
    let style = format!("{} {}", base_style, props.style);

    let label = props.label.as_deref().unwrap_or_default();

    let icon_html = if let Some(icon) = &props.icon {
        html! { <img src={icon.clone()}/> }
    } else {
        html! {}
    };

    let content = match props.icon_position {
        Some(IconPosition::Start) =>
            html! {
                <>
                    {icon_html}
                    {label}
                </> 
            },
        Some(IconPosition::End) =>
            html! { 
                <>
                    {label}
                    {icon_html}
                </>
            },
        None =>
            html! { 
                <>
                    {icon_html}
                    {label}
                </>
            },
    };

    html! {
        <button style={style} onclick={props.onclick.clone()}>
            {content}
        </button>
    }
}
