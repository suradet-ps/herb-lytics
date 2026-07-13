use leptos::prelude::*;

use crate::core::types::HerbSummary;
use crate::core::utils::{format_number, format_thb_full};

#[component]
pub fn SummaryCards(summary: HerbSummary) -> impl IntoView {
  let highest = summary
    .herbs
    .iter()
    .max_by(|a, b| a.total_value.total_cmp(&b.total_value))
    .cloned();

  let count = summary.herbs.len();

  let highest_block = match highest {
    Some(h) => {
      let name = h.name.clone();
      let value = h.total_value;
      view! {
          <p class="text-2xl font-bold text-text-primary truncate" title=name.clone()>
              { name.clone() }
          </p>
          <p class="text-lg text-amber-600 mt-1 font-medium">
              { format_thb_full(value) }
          </p>
      }
      .into_any()
    }
    None => view! { <p class="text-text-muted">"-"</p> }.into_any(),
  };

  view! {
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
          <div class="bg-primary rounded-2xl p-8 text-white shadow-xl relative overflow-hidden group flex flex-col justify-between">
              <div class="absolute top-0 right-0 w-48 h-48 bg-accent/10 rounded-full blur-3xl -mr-10 -mt-10 pointer-events-none" />

              <div class="relative z-10">
                  <p class="text-slate-400 font-medium uppercase tracking-wider text-sm mb-1">
                      "มูลค่าการจัดซื้อยาสมุนไพร"
                  </p>
                  <h3 class="text-xl font-semibold text-white mb-6">"มูลค่ารวมทั้งหมด"</h3>

                  <div class="flex items-baseline gap-2">
                      <span class="text-4xl font-bold tracking-tight text-white tabular-nums">
                          { format_number(summary.grand_total) }
                      </span>
                      <span class="text-lg text-slate-400 font-medium">"บาท"</span>
                  </div>
              </div>

              <div class="mt-6 flex items-center gap-2 text-sm text-emerald-400 bg-emerald-500/10 w-fit px-3 py-1 rounded-full border border-emerald-500/20">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path
                          stroke-linecap="round"
                          stroke-linejoin="round"
                          stroke-width="2"
                          d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"
                      />
                  </svg>
                  <span class="font-medium">"อัปเดตล่าสุด"</span>
              </div>
          </div>

          <div class="bg-white rounded-2xl p-8 shadow-sm border border-border hover-card relative overflow-hidden flex flex-col justify-between">
              <div class="absolute right-0 top-0 w-32 h-32 bg-blue-50 rounded-full blur-2xl -mr-10 -mt-10" />

              <div class="relative z-10">
                  <div class="flex justify-between items-start mb-6">
                      <div>
                          <p class="text-text-secondary text-sm font-medium uppercase tracking-wider">
                              "จำนวนรายการยาสมุนไพร"
                          </p>
                          <h3 class="text-xl font-semibold text-text-primary">"จำนวนรายการ"</h3>
                      </div>
                      <div class="w-10 h-10 bg-blue-50 rounded-lg flex items-center justify-center text-blue-600">
                          <svg
                              xmlns="http://www.w3.org/2000/svg"
                              class="h-6 w-6"
                              fill="none"
                              viewBox="0 0 24 24"
                              stroke="currentColor"
                          >
                              <path
                                  stroke-linecap="round"
                                  stroke-linejoin="round"
                                  stroke-width="2"
                                  d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
                              />
                          </svg>
                      </div>
                  </div>

                  <p class="text-4xl font-bold text-text-primary tabular-nums">{ count }</p>
                  <p class="text-sm text-text-muted mt-2">"รายการยาที่มีการจัดซื้อ"</p>
              </div>
          </div>

          <div class="bg-white rounded-2xl p-8 shadow-sm border border-border hover-card relative overflow-hidden flex flex-col justify-between">
              <div class="absolute right-0 top-0 w-32 h-32 bg-amber-50 rounded-full blur-2xl -mr-10 -mt-10" />

              <div class="relative z-10">
                  <div class="flex justify-between items-start mb-6">
                      <div>
                          <p class="text-text-secondary text-sm font-medium uppercase tracking-wider">
                              "มูลค่าจัดซื้อสูงสุด"
                          </p>
                          <h3 class="text-xl font-semibold text-text-primary">
                              "สมุนไพรมูลค่าจัดซื้อสูงสุด"
                          </h3>
                      </div>
                      <div class="w-10 h-10 bg-amber-50 rounded-lg flex items-center justify-center text-amber-600">
                          <svg
                              xmlns="http://www.w3.org/2000/svg"
                              class="h-6 w-6"
                              fill="none"
                              viewBox="0 0 24 24"
                              stroke="currentColor"
                          >
                              <path
                                  stroke-linecap="round"
                                  stroke-linejoin="round"
                                  stroke-width="2"
                                  d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
                              />
                          </svg>
                      </div>
                  </div>

                  { highest_block }
              </div>
          </div>
      </div>
  }
}
