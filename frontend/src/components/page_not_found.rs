use {
    yew::{html, Properties},
    yew_functional::function_component,
};

#[derive(Properties, Clone, PartialEq)]
pub struct PageNotFoundProps {
    #[prop_or(String::from("Page not found."))]
    pub page_name: String,
}

#[function_component(PageNotFound)]
pub fn page_not_found(PageNotFoundProps { page_name }: &PageNotFoundProps) -> Html {
    html! {
        <div style="display: flex; flex: 1; justify-content: center;">
            <p>{page_name}</p>
        </div>
    }
}
