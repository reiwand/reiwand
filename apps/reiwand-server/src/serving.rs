use axum::serve::Listener;
use hyper_util::rt::{TokioExecutor, TokioIo, TokioTimer};
use hyper_util::server::conn::auto::Builder;
use hyper_util::server::graceful::GracefulShutdown;
use hyper_util::service::TowerToHyperService;
use std::net::SocketAddr;
use std::time::Duration;

pub(crate) const HEADER_TIMEOUT: Duration = Duration::from_secs(10);
const DRAIN: Duration = Duration::from_secs(30);

pub(crate) async fn run<L: Listener<Addr = SocketAddr>>(
    mut listener: L,
    router: axum::Router,
    stop: impl Future<Output = ()>,
) {
    let graceful = GracefulShutdown::new();
    tokio::pin!(stop);
    loop {
        let (io, peer) = tokio::select! {
            accepted = listener.accept() => accepted,
            () = &mut stop => break,
        };
        let service = router.clone().layer(axum::Extension(axum::extract::ConnectInfo(peer)));
        let mut builder = Builder::new(TokioExecutor::new());
        builder.http1().timer(TokioTimer::new()).header_read_timeout(HEADER_TIMEOUT);
        let conn =
            builder.serve_connection_with_upgrades(TokioIo::new(io), TowerToHyperService::new(service));
        let conn = graceful.watch(conn.into_owned());
        tokio::spawn(async move {
            let _ = conn.await;
        });
    }
    let _ = tokio::time::timeout(DRAIN, graceful.shutdown()).await;
}
