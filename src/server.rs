use anyhow::Result;
use axum::{Router, serve};
use axum::{
    extract::Request,
    http::{HeaderValue, header},
    middleware::{self, Next},
    response::Response,
};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub struct Server {
    listener: TcpListener,
    router: Router,
}

impl Server {
    pub async fn new(addr: &str) -> Result<Self> {
        let router = build_router();
        let listener = TcpListener::bind(addr).await?;
        let server = Self { listener, router };

        Ok(server)
    }

    pub async fn run(self) -> Result<()> {
        serve(self.listener, self.router)
            .with_graceful_shutdown(shutdown())
            .await?;

        Ok(())
    }
}

pub fn build_router() -> Router {
    let srv = Router::new()
        .nest_service("/streams", ServeDir::new("./output"))
        .layer(middleware::from_fn(fix_mime_types));
    srv
}

async fn shutdown() {
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            println!("shutting down after recieveing a sigkil")
        }
    };
}

async fn fix_mime_types(req: Request, next: Next) -> Response {
    let path = req.uri().path().to_string();

    let mut response = next.run(req).await;

    if path.ends_with(".m3u8") {
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/vnd.apple.mpegurl"),
        );
    } else if path.ends_with(".ts") {
        response
            .headers_mut()
            .insert(header::CONTENT_TYPE, HeaderValue::from_static("video/mp2t"));
    }

    response
}
