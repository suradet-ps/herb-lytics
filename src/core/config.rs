use gloo_storage::{LocalStorage, Storage};

use crate::core::error::{AppError, AppResult};

/// localStorage key used as a runtime override for the API base URL.
/// Lets local development point at a script without rebuilding.
const STORAGE_KEY: &str = "herb_lytics_api_url";

/// Resolve the Google Apps Script base URL.
///
/// Order:
/// 1. Compile-time `GOOGLE_API_URL` (baked in by `trunk build`), or
/// 2. a `herb_lytics_api_url` value stored in `localStorage` at runtime.
pub fn api_base_url() -> AppResult<String> {
  if let Some(url) = option_env!("GOOGLE_API_URL") {
    if !url.is_empty() {
      return Ok(url.to_string());
    }
  }

  LocalStorage::get::<String>(STORAGE_KEY).map_err(|_| {
    AppError::config(
      "GOOGLE_API_URL is not configured. Set the build-time env var, or set the \
             'herb_lytics_api_url' localStorage key in the browser console.",
    )
  })
}
