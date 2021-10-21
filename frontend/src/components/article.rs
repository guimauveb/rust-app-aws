use {
    crate::{
        entities::interfaces::{IArticle, Status},
        service::{articles::get_article, future::handle_future},
        utils::date::format_date,
    },
    yew::{html, Properties},
    yew_functional::{function_component, use_effect_with_deps, use_state},
};

#[derive(Properties, Clone, PartialEq)]
pub struct ArticleProps {
    pub id: i32,
}

#[function_component(Article)]
pub fn article(ArticleProps { id }: &ArticleProps) -> Html {
    let id = *id;

    let (is_loading, set_loading) = use_state(|| false);
    let (article, set_article) = use_state(move || IArticle::default());

    use_effect_with_deps(
        move |_| {
            set_loading(true);
            let future = async move { get_article(&id).await };
            handle_future(future, move |data: Result<IArticle, Status>| {
                match data {
                    Ok(article) => set_article(article),
                    Err(_) => (),
                };
                set_loading(false);
            });
            || {}
        },
        (),
    );

    html! {
        {if *is_loading {
            html! {}
        } else {
            html! {
                <div style="display: flex; flex-direction: column; flex: 1;">
                    <div style="display: flex; max_width: 1024px; flex:1 0 100%; flex-direction: column;">
                        <h1 style="margin-bottom: 8px;">{&article.title}</h1>
                        <div style="margin-top: 12px; margin-bottom: 12px;">
                            {match format_date(&article.pub_date) {
                                Ok(date) => html! {<p>{&date}</p>},
                                Err(_) => html! {<p>{"An error occured!"}</p>},
                            }}
                        </div>
                        <img style="margin-top: 8px; margin-bottom:8px; align-self: center;" width="77%" src={&article.image} />
                        <h3 style="margin-top:8px; margin-bottom:8px;">{&article.preview} </h3>
                        <p>{&article.content}</p>
                    </div>
                </div>
            }
        }}
    }
}
