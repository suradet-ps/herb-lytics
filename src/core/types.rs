use serde::Deserialize;

/// A single herb purchase row returned by the API.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct HerbItem {
  pub name: String,
  #[serde(rename = "totalValue")]
  pub total_value: f64,
}

/// The yearly summary returned by `getHerbSummary`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct HerbSummary {
  pub year: i32,
  #[serde(rename = "grandTotal")]
  pub grand_total: f64,
  pub herbs: Vec<HerbItem>,
}

/// Raw envelope returned by the Google Apps Script endpoint.
#[derive(Debug, Deserialize)]
pub struct ApiEnvelope {
  pub status: String,
  #[serde(default)]
  pub message: Option<String>,
  #[serde(default)]
  pub data: Option<HerbSummary>,
}
