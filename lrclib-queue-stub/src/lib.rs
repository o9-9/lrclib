use std::sync::Arc;

use server::AppState;
use tokio::sync::watch;

pub async fn start_queue(initial_workers: u8, mut rx: watch::Receiver<u8>, _state: Arc<AppState>) {
    tracing::info!(
        message = "queue backend disabled (stub crate)",
        configured_workers = initial_workers,
    );

    loop {
        if rx.changed().await.is_err() {
            break;
        }

        tracing::debug!(
            message = "ignoring workers update because queue backend is disabled",
            workers_count = *rx.borrow(),
        );
    }
}
