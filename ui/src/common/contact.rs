use super::image::Icon;
use crate::link::Link;
use utils::use_style;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ContactsProps {
    pub source: Vec<ContactType>,
}

#[derive(Clone, PartialEq)]
pub enum ContactType {
    Twitter,
    GitHub,
    Email,
    WeChat,
    Discord,
}

impl From<&ContactType> for &'static str {
    fn from(contact: &ContactType) -> &'static str {
        match contact {
            ContactType::Discord => "discord.svg",
            ContactType::Twitter => "twitter.svg",
            ContactType::WeChat => "wechat.svg",
            ContactType::Email => "gmail.svg",
            ContactType::GitHub => "github.svg",
        }
    }
}

impl ContactType {
    pub fn into_lnk(&self) -> &'static str {
        match self {
            ContactType::Discord => "#",
            ContactType::Twitter => "https://twitter.com/jmyounghoyos",
            ContactType::WeChat => "#",
            ContactType::Email => "mailto:juanmanuel12.13.jmyh81@gmail.com.com",
            ContactType::GitHub => "https://github.com/Youngermaster",
        }
    }
}

impl ContactType {
    fn has_theme(&self) -> bool {
        match self {
            ContactType::GitHub => true,
            _ => false,
        }
    }

    fn into_size(&self) -> i32 {
        match self {
            ContactType::GitHub => 30,
            _ => 32,
        }
    }
}

#[function_component(Contacts)]
pub fn contacts(props: &ContactsProps) -> Html {
    let style = use_style!(
        r"
        display: flex;
        align-items: center;
    "
    );

    let render_contacts = props
        .source
        .iter()
        .map(|contact| {
            let source: &'static str = contact.into();
            html! {
                <Link out_href={contact.into_lnk()}>
                    <Icon source={source} size={contact.into_size()} has_theme={contact.has_theme()} />
                </Link>
            }
        })
        .collect::<Html>();

    html! {
        <div class={style}>
            {render_contacts}
        </div>
    }
}
