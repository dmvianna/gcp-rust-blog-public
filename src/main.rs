use std::{net::SocketAddr, sync::Arc};

use axum::{extract::Path, extract::State, response::Html, routing::get, Router};
use pulldown_cmark::{html, Options, Parser};
use tokio::{fs, net::TcpListener};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct AppState {
    banner_html: String,
    layout_html: String,
    home_html: String,
    not_found_html: String, // supports {{slug}} placeholder
}

async fn homepage(State(state): State<Arc<AppState>>) -> Html<String> {
    let page_body = state
        .layout_html
        .replace("{{ content }}", &state.home_html);
    Html(format!("{}{}", state.banner_html, page_body))
}

async fn render_post(Path(slug): Path<String>, State(state): State<Arc<AppState>>) -> Html<String> {
    let path = format!("content/posts/{}.md", slug);
    let md = match fs::read_to_string(&path).await {
        Ok(c) => c,
        Err(_) => {
            let body = state.not_found_html.replace("{{slug}}", &slug);
            let page = state.layout_html.replace("{{ content }}", &body);
            return Html(format!("{}{}", state.banner_html, page));
        }
    };

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(&md, options);
    let mut html_out = String::new();
    html::push_html(&mut html_out, parser);

    let wrapped = state.layout_html.replace("{{ content }}", &html_out);
    let page = format!("{}{}", state.banner_html, wrapped);
    Html(page)
}

#[tokio::main]
async fn main() {
    // logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Strictly load HTML from content/ files (no inline HTML fallbacks)
    let banner_html = fs::read_to_string("content/banner.html")
        .await
        .expect("Missing content/banner.html");

    let layout_html = fs::read_to_string("content/layout.html")
        .await
        .expect("Missing content/layout.html");

    let home_html = fs::read_to_string("content/home.html")
        .await
        .expect("Missing content/home.html");

    let not_found_html = fs::read_to_string("content/not_found.html")
        .await
        .expect("Missing content/not_found.html");

    let state = Arc::new(AppState {
        banner_html,
        layout_html,
        home_html,
        not_found_html,
    });

    let app = Router::new()
        .route("/", get(homepage))
        .route("/posts/:slug", get(render_post))
        .with_state(state);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::info!(%addr, "listening");
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
