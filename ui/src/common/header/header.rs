use crate::contact::ContactType;
use crate::container::Container;
use crate::external_link::ExternalLink;
use crate::header::drawer::Drawer;
use crate::header::drawer_item::DrawerItem;
use crate::image::Icon;
use crate::link::Link;
use router::RootRoutes;
use utils::theme::only_render_on_mobile;
use utils::use_style;
use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let style = use_style!(
        r"
        width: 100%;

        .wrapper {
            height: 56px;
            justify-content: space-between;
        }

        .wrapper, .tabs, .left, .right, .setting-icon {
            display: flex;
            align-items: center;
        }

        .left img {
            height: 16px;
        }

        .tabs {
            margin-left: 88px;
        }

        .tabs a {
            font-size: 14px;
            margin: 0 15px;
            font-weight: 900;
        }

        .setting-icon {
            margin-right: 19px;
        }

        .header {
            height:56px;
            width: 100%;
            background: var(--base-color);
            position: relative;
            z-index: 6;
        }

        .header-name {
            font-size: 2 rem;
            color: white;
            font-weight: 900;
        }

        @media (max-width: 600px) {
            .tabs {
                display: none;
            }
        }
    "
    );

    let is_open_drawer_handle = use_state_eq(|| false);
    let handle_drawer_click = {
        let is_open_drawer_handle = is_open_drawer_handle.clone();

        Callback::from(move |_| is_open_drawer_handle.set(!*is_open_drawer_handle))
    };

    html! {
        <div class={style}>
            <Drawer is_open={is_open_drawer_handle}>
                <DrawerItem lnk={RootRoutes::Home}>{"Posts"}</DrawerItem>
            </Drawer>
            <div class="header">
                <Container>
                    <div class="wrapper">
                        <div class="left">
                            <Link href={RootRoutes::Home}>
                                <a class="header-name">{"Juan Manuel Young Hoyos's Blog"}</a>
                            </Link>
                            <div class="tabs">
                                <Link href={RootRoutes::Home}>{"Posts"}</Link>
                                <ExternalLink href={"https://jmyounghoyos.com/".to_string()}>{"About me"}</ExternalLink>
                            </div>
                        </div>
                        <div class="right">
                            <Link out_href={ContactType::GitHub.into_lnk()}>
                                <Icon source="github.svg" size=26 />
                            </Link>
                            {only_render_on_mobile(html! {
                                <Icon source="drawer.svg" size=26 onclick={handle_drawer_click} />
                            })}
                        </div>
                    </div>
                </Container>
            </div>
        </div>
    }
}
