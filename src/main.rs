use axum::{
    Router,
    extract::Path,
    response::Html,
    routing::{get, get_service},
};
use clap::Parser;
use tower_http::services::ServeDir;

mod posts;
mod styles;
mod templates;

#[derive(Parser, Debug)]
#[command(name = "devblog")]
#[command(about = "A simple personal blog server", long_about = None)]
struct Args {
    /// Port to run the server on
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(home))
        .route("/post/:slug", get(post))
        .nest_service("/static", get_service(ServeDir::new("static")));

    let addr = format!("0.0.0.0:{}", args.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Blog running on http://localhost:{}", args.port);

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

                    article class="post-content" {
                        p { "File not found: No such post exists." }
                    }

                    div class="bottom-prompt" {
                        span class="prompt-symbol" { "$ " }
                        span class="cursor" { }
                    }
                    div class="navigation" {
                        a href="/" { "← Back to posts" }
                    }
                },
            );
            Html(markup.into_string())
        }
    }
}
