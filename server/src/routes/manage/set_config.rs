use crate::{errors::ApiError, AppState};
use anyhow::{Context, Result};
use axum::{
    extract::{Json, State},
    http::{header::HeaderMap, StatusCode},
};
use axum_macros::debug_handler;
use serde::Deserialize;
use std::sync::{atomic::Ordering, Arc};
use subtle::ConstantTimeEq;

#[derive(Debug, Deserialize)]
pub struct SetConfigRequest {
    workers_count: u8,
}

/// Handler for the /manage/set-config route
/// Updates the workers count in the AppState and sends the new value to the worker supervisor
#[debug_handler]
pub async fn route(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<SetConfigRequest>,
) -> Result<StatusCode, ApiError> {
    let token = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split_whitespace().last())
        .unwrap_or_default();

    // Verify the token against the environment variable
    let expected_token = std::env::var("LRCLIB_MANAGE_TOKEN").unwrap_or_default();
    let are_tokens_equal: bool = token.as_bytes().ct_eq(expected_token.as_bytes()).into();

    if !are_tokens_equal || expected_token.is_empty() {
        return Err(ApiError::InvalidManageTokenError);
    }

    // Update the workers count in AppState
    state
        .workers_count
        .store(payload.workers_count, Ordering::SeqCst);

    // Send the new value to the worker supervisor
    let tx_guard = state.workers_tx.lock().await;
    if let Some(tx) = tx_guard.as_ref() {
        tx.send(payload.workers_count)
            .context("Failed to update workers count")
            .map_err(ApiError::from)?;

        tracing::info!("Workers count updated to {}", payload.workers_count);
        Ok(StatusCode::ACCEPTED)
    } else {
        Err(ApiError::UnknownError(anyhow::anyhow!(
            "Worker supervisor channel not initialized"
        )))
    }
}
