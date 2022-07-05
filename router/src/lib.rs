use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum RootRoutes {
    #[at("/home")]
    Home,
    #[at("/posts/:filename")]
    Post { filename: String },
    #[at("/")]
    Root,
    // Compatible with https://github.com/jetli/awesome-yew
    #[at("/technology")]
    Technology,
    #[not_found]
    #[at("/404")]
    NotFound,
}
