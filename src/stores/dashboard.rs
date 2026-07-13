use leptos::prelude::*;
use std::sync::OnceLock;

use crate::core::api::fetch_herb_summary;
use crate::core::error::AppResult;
use crate::core::time::current_thai_fiscal_year;
use crate::core::types::HerbSummary;

static DASHBOARD: OnceLock<DashboardState> = OnceLock::new();

/// Install the dashboard store inside the reactive owner (called from
/// `mount_to_body` so its `RwSignal`s are never disposed).
pub fn install() {
  let _ = DASHBOARD.set(DashboardState::new());
}

pub fn use_dashboard() -> DashboardState {
  *DASHBOARD.get().expect("DashboardState not initialized")
}

#[derive(Debug, Clone, Copy)]
pub struct DashboardState {
  pub selected_year: RwSignal<i32>,
  pub summary: RwSignal<Option<HerbSummary>>,
  pub loading: RwSignal<bool>,
  pub error: RwSignal<Option<String>>,
  pub available_years: RwSignal<Vec<i32>>,
}

impl DashboardState {
  pub fn new() -> Self {
    let current = current_thai_fiscal_year();
    Self {
      selected_year: RwSignal::new(current),
      summary: RwSignal::new(None),
      loading: RwSignal::new(true),
      error: RwSignal::new(None),
      available_years: RwSignal::new((0..5).map(|i| current - i).collect()),
    }
  }

  /// Fetch the summary for the currently selected year, updating the
  /// `loading` / `error` / `summary` signals.
  pub async fn fetch(&self) -> AppResult<()> {
    self.loading.set(true);
    self.error.set(None);

    let year = self.selected_year.get_untracked();
    match fetch_herb_summary(year).await {
      Ok(data) => self.summary.set(Some(data)),
      Err(e) => self.error.set(Some(e.to_string())),
    }

    self.loading.set(false);
    Ok(())
  }
}
