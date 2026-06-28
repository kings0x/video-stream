use anyhow::Result;
use axum::{Router, routing::get, serve};
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
    let srv = Router::new().nest_service("/streams", ServeDir::new(".output"));
    srv
}

async fn shutdown() {
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            println!("shutting down after recieveing a sigkil")
        }
    };
}
