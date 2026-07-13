use leptos::prelude::*;

use crate::core::types::HerbItem;
use crate::core::utils::{format_axis, format_thb};

/// Self-contained SVG bar chart of the top-10 herbs by purchase value.
///
/// Replaces the old Chart.js component: rendering in SVG keeps the bundle
/// fully self-contained (no JS charting dependency) and avoids `canvas`/WASM
/// interop.
#[component]
pub fn HerbChart(herbs: Vec<HerbItem>) -> impl IntoView {
  let mut sorted = herbs;
  sorted.sort_by(|a, b| b.total_value.total_cmp(&a.total_value));
  sorted.truncate(10);

  let max_val = sorted
    .iter()
    .map(|h| h.total_value)
    .fold(0.0_f64, f64::max)
    .max(1.0);

  let width = 640.0_f64;
  let height = 400.0_f64;
  let pad_l = 56.0_f64;
  let pad_r = 12.0_f64;
  let pad_t = 16.0_f64;
  let pad_b = 110.0_f64;
  let chart_w = width - pad_l - pad_r;
  let chart_h = height - pad_t - pad_b;
  let n = sorted.len().max(1) as f64;
  let slot = chart_w / n;
  let bar_w = (slot * 0.7).max(4.0);
  let ticks: usize = 4;

  let grid_color = "#F1F5F9";
  let label_color = "#64748B";

  let y_for = move |v: f64| pad_t + chart_h * (1.0 - v / max_val);

  view! {
      <div class="bg-surface p-6 rounded-2xl shadow-sm border border-border h-[500px] flex flex-col">
          <div class="mb-6 flex-none">
              <div class="flex items-center justify-between">
                  <div>
                      <h2 class="font-bold text-text-primary text-lg tracking-tight">
                          "10 อันดับสมุนไพรที่มีมูลค่าจัดซื้อสูงสุด"
                      </h2>
                      <p class="text-sm text-text-muted mt-1">
                          "แสดงข้อมูลเรียงลำดับตามมูลค่าการจัดซื้อ"
                      </p>
                  </div>

                  <div class="flex items-center gap-2 px-3 py-1 bg-primary-muted rounded-full border border-primary/10">
                      <span class="w-2 h-2 rounded-full bg-primary"></span>
                      <span class="text-xs font-semibold text-primary">"มูลค่า (บาท)"</span>
                  </div>
              </div>
          </div>

          <div class="flex-1 w-full relative min-h-0">
              <svg
                  viewBox=format!("0 0 {width} {height}")
                  class="w-full h-full"
                  preserveAspectRatio="xMidYMid meet"
              >
                  <defs>
                      <linearGradient id="herbBar" x1="0" y1="0" x2="0" y2="1">
                          <stop offset="0%" stop-color="#059669" />
                          <stop offset="100%" stop-color="#047857" />
                      </linearGradient>
                  </defs>

                  {move || {
                      (0..=ticks)
                          .map(|i| {
                              let val = max_val * (i as f64 / ticks as f64);
                              let y = y_for(val);
                              view! {
                                  <line
                                      x1=pad_l
                                      y1=y
                                      x2=width - pad_r
                                      y2=y
                                      stroke=grid_color
                                      stroke-width="1"
                                  />
                                  <text
                                      x=pad_l - 8.0
                                      y=y + 4.0
                                      text-anchor="end"
                                      fill=label_color
                                      font-size="11"
                                  >
                                      { format_axis(val) }
                                  </text>
                              }
                          })
                          .collect_view()
                  }}

                  {move || {
                      sorted
                          .iter()
                          .enumerate()
                          .map(|(i, h)| {
                              let x = pad_l + slot * i as f64 + (slot - bar_w) / 2.0;
                              let bh = (chart_h * (h.total_value / max_val)).max(0.0);
                              let y = pad_t + chart_h - bh;
                              let label_y = pad_t + chart_h + 12.0;
                              view! {
                                  <rect x=x y=y width=bar_w height=bh rx="6" fill="url(#herbBar)">
                                      <title>
                                          { format!("{}: {}", h.name, format_thb(h.total_value)) }
                                      </title>
                                  </rect>
                                  <text
                                      transform=format!(
                                          "translate({} {}) rotate(45)",
                                          x + bar_w / 2.0,
                                          label_y,
                                      )
                                      text-anchor="start"
                                      fill=label_color
                                      font-size="11"
                                  >
                                      { h.name.clone() }
                                  </text>
                              }
                          })
                          .collect_view()
                  }}
              </svg>
          </div>
      </div>
  }
}
