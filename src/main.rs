use axum::{
    Router,
    extract::Path,
    response::Html,
    routing::{get, get_service},
};
use tower_http::services::ServeDir;

mod posts;
mod styles;
mod templates;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(home))
        .route("/post/:slug", get(post))
        .nest_service("/static", get_service(ServeDir::new("static")));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Blog running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn home() -> Html<String> {
    let posts = posts::load_posts();
    let markup = templates::home_page(&posts);
    Html(markup.into_string())
}

async fn post(Path(slug): Path<String>) -> Html<String> {
    match posts::get_post_by_slug(&slug) {
        Some(post) => {
            let markup = templates::post_page(&post);
            Html(markup.into_string())
        }
        None => {
            let markup = templates::page(
                "Not Found",
                maud::html! {
                    div class="prompt" {
                        span class="prompt-symbol" { "$ " }
                        span { "cat posts/" (slug) ".md" }
                    }
                    p { "File not found: No such post exists." }
                    a href="/" { "← Back to posts" }
                },
            );
            Html(markup.into_string())
        }
    }
}
