use post::Post;
use ui::layout::BaseLayout;
use yew::prelude::*;
use yew_router::prelude::*;

use home::Home;
use not_found::NotFound;
use router::RootRoutes;

fn switch(routes: &RootRoutes) -> Html {
    match routes {
        RootRoutes::Home | RootRoutes::Root => html! { <Home /> },
        RootRoutes::Post { filename } => html! {<Post filename={filename.clone()} />},
        RootRoutes::NotFound => html! { <NotFound />},
        RootRoutes::Technology => html! {
            <Redirect<RootRoutes> to={RootRoutes::Home}/>
        },
    }
}

#[function_component(RouteOutlet)]
pub fn route_outlet() -> Html {
    html! {
        <BrowserRouter>
            <BaseLayout>
                <Switch<RootRoutes> render={Switch::render(switch)} />
            </BaseLayout>
        </BrowserRouter>
    }
}
