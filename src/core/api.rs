use gloo_net::http::Request;

use crate::core::config::api_base_url;
use crate::core::error::{AppError, AppResult};
use crate::core::types::{ApiEnvelope, HerbSummary};

/// Fetch the herb purchase summary for a given Thai fiscal year (พ.ศ.).
///
/// Maps the Google Apps Script REST response (`{ status, message?, data? }`)
/// onto a typed `HerbSummary`, mirroring the zod validation in the old Vue
/// service.
pub async fn fetch_herb_summary(year: i32) -> AppResult<HerbSummary> {
  let base = api_base_url()?;
  let url = format!("{base}?path=getHerbSummary&year={year}");

  let resp = Request::get(&url).send().await?;
  if !resp.ok() {
    return Err(AppError::http(
      resp.status(),
      "Failed to reach the herb summary API",
    ));
  }

  let text = resp.text().await?;
  let envelope: ApiEnvelope = serde_json::from_str(&text)
    .map_err(|e| AppError::other(format!("Invalid API response: {e}")))?;

  if envelope.status != "success" {
    return Err(AppError::other(
      envelope
        .message
        .unwrap_or_else(|| "Unknown API error".to_string()),
    ));
  }

    envelope
        .data
        .ok_or_else(|| AppError::other("API returned no data"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{
        "status": "success",
        "data": {
            "year": 2568,
            "grandTotal": 123456,
            "herbs": [
                { "name": "กระชาย", "totalValue": 50000 },
                { "name": "ขิง", "totalValue": 30000 }
            ]
        }
    }"#;

    #[test]
    fn parses_success_envelope() {
        let envelope: ApiEnvelope = serde_json::from_str(SAMPLE).expect("valid json");
        assert_eq!(envelope.status, "success");
        let data = envelope.data.expect("has data");
        assert_eq!(data.year, 2568);
        assert_eq!(data.grand_total, 123456.0);
        assert_eq!(data.herbs.len(), 2);
        assert_eq!(data.herbs[0].name, "กระชาย");
        assert_eq!(data.herbs[0].total_value, 50000.0);
    }

    #[test]
    fn rejects_error_status() {
        let bad = r#"{"status":"error","message":"no data"}"#;
        let envelope: ApiEnvelope = serde_json::from_str(bad).expect("valid json");
        assert_eq!(envelope.status, "error");
        assert!(envelope.data.is_none());
    }
}
