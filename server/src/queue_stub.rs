use std::sync::Arc;
use tokio::sync::watch;
use crate::AppState;

#[derive(Debug)]
pub struct ScrapedData {
  pub plain_lyrics: Option<String>,
  pub synced_lyrics: Option<String>,
  pub instrumental: bool,
}

pub async fn start_queue(initial_workers: u8, mut rx: watch::Receiver<u8>, _state: Arc<AppState>) {
  tracing::info!(
    message = "queue backend disabled (stub mode)",
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
