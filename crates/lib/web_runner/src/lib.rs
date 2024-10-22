pub mod axum_app_error;
pub mod axum_server;

pub use axum_app_error::AppError;
use listenfd::ListenFd;
use tokio::net::TcpListener;

pub async fn start_listenfd(addr: &str) -> anyhow::Result<TcpListener> {
    let mut listenfd = ListenFd::from_env();
    if let Some(listener) = listenfd.take_tcp_listener(0)? {
        let _ = listener.set_nonblocking(true);
        Ok(TcpListener::from_std(listener)?)
    } else {
        Ok(TcpListener::bind(addr).await?)
    }
}
