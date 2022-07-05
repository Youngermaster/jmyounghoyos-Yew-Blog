use utils::use_style;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps {
    #[prop_or_default]
    pub href: String,
    #[prop_or(String::from(""))]
    pub out_href: String,
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(ExternalLink)]
pub fn link(props: &LinkProps) -> Html {
    let style = use_style!(
        r"
        text-decoration: none;
        transition: 0.3s opacity;
        cursor: pointer;
        
        &:hover {
            opacity: 0.75;
        }
    "
    );
    let onclick_callback = props.onclick.clone();

    html! {
        <a class={style} onclick={onclick_callback}>
            {props.children.clone()}
        </a>
    }
}
