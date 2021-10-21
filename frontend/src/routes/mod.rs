use {yew_router::prelude::Switch, yew_router::switch::Permissive};

#[derive(Switch, Clone, PartialEq)]
pub enum AppRoute {
    #[to = "/articles/{id}"]
    Article { id: i32 },
    #[to = "/articles!"]
    Articles,
    #[to = "/404"]
    PageNotFound(Permissive<String>),
}
