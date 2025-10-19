use std::{net::SocketAddr, sync::Arc};

use axum::{extract::Path, extract::State, response::Html, routing::get, Router};
use pulldown_cmark::{html, Options, Parser};
use tokio::{fs, net::TcpListener};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct AppState {
    banner_html: String,
}

async fn homepage(State(state): State<Arc<AppState>>) -> Html<String> {
    let body = r#"
        <main style="max-width:760px;margin:24px auto;padding:0 16px;">
          <section>
            <h2>Welcome</h2>
            <p>This is a minimal Rust + axum blog deployed to GCP Cloud Run.</p>
          </section>
          <section>
            <h3>Posts</h3>
            <ul>
              <li><a href="/posts/first-post">My Rust + GCP blog kickoff</a></li>
            </ul>
          </section>
        </main>
    "#;

    Html(format!("{}{}", state.banner_html, body))
}

async fn render_post(Path(slug): Path<String>, State(state): State<Arc<AppState>>) -> Html<String> {
    let path = format!("content/posts/{}.md", slug);
    let md = match fs::read_to_string(&path).await {
        Ok(c) => c,
        Err(_) => {
            let not_found = format!(
                "{}<main style=\"max-width:760px;margin:24px auto;padding:0 16px;\"><h2>Post not found</h2><p>No post for slug: {}</p></main>",
                state.banner_html, slug
            );
            return Html(not_found);
        }
    };

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(&md, options);
    let mut html_out = String::new();
    html::push_html(&mut html_out, parser);

    let page = format!(
        "{}<main style=\"max-width:760px;margin:24px auto;padding:0 16px;\">{}</main>",
        state.banner_html, html_out
    );
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

    let banner_html = fs::read_to_string("content/banner.html")
        .await
        .unwrap_or_else(|_| "<header style=\"padding:16px;background:#0f172a;color:#e2e8f0;border-bottom:4px solid #22d3ee;\"><div style=\"display:flex;align-items:center;justify-content:space-between;gap:16px;\"><div><h1 style=\"margin:0;font-size:28px;\">GCP Rust Blog</h1><p style=\"margin:4px 0 0 0;font-size:14px;opacity:0.9;\">by @dmvianna</p></div><nav><ul style=\"list-style:none;display:flex;gap:16px;margin:0;padding:0;\"><li><a href=\"/\" style=\"color:#e2e8f0;text-decoration:none;\">Home</a></li><li><a href=\"/posts/first-post\" style=\"color:#e2e8f0;text-decoration:none;\">First post</a></li><li><a href=\"https://github.com/dmvianna\" style=\"color:#e2e8f0;text-decoration:none;\" target=\"_blank\" rel=\"noopener noreferrer\">GitHub</a></li></ul></nav></div></header>".to_string());

    let state = Arc::new(AppState { banner_html });

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
