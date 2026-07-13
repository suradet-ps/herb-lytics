use leptos::prelude::*;

use crate::core::types::HerbItem;
use crate::core::utils::format_thb;

#[component]
pub fn HerbTable(herbs: Vec<HerbItem>) -> impl IntoView {
  let mut sorted = herbs;
  sorted.sort_by(|a, b| b.total_value.total_cmp(&a.total_value));
  let count = sorted.len();
  let ranked: Vec<(usize, HerbItem)> = sorted.into_iter().enumerate().collect();

  let badge_class = |i: usize| -> &'static str {
    match i {
      0 => "bg-linear-to-br from-yellow-400 to-amber-600 text-white",
      1 => "bg-linear-to-br from-slate-300 to-slate-500 text-white",
      _ => "bg-linear-to-br from-amber-700 to-amber-900 text-white",
    }
  };

  view! {
      <div class="bg-white rounded-2xl shadow-sm border border-border flex flex-col h-[500px] overflow-hidden">
          <div class="p-6 border-b border-border flex items-center justify-between bg-white">
              <div>
                  <h2 class="font-bold text-text-primary text-lg tracking-tight flex items-center gap-2">
                      <svg
                          xmlns="http://www.w3.org/2000/svg"
                          class="h-5 w-5 text-accent"
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
                      "จัดลำดับมูลค่าการจัดซื้อยาสมุนไพร"
                  </h2>
                  <p class="text-sm text-text-muted mt-1">
                      "เรียงลำดับตามมูลค่าการจัดซื้อสูงสุด"
                  </p>
              </div>

              <div class="px-3 py-1 bg-slate-100 rounded-full border border-slate-200">
                  <span class="text-xs font-semibold text-slate-600">
                      { format!("{count} รายการ") }
                  </span>
              </div>
          </div>

          <div class="overflow-y-auto flex-1 scrollbar-thin">
              <Show
                  when=move || count == 0
                  fallback=move || {
                      let ranked = ranked.clone();
                      view! {
                          <table class="w-full text-left border-collapse">
                              <thead class="sticky top-0 z-10 bg-slate-50 shadow-sm">
                                  <tr>
                                      <th class="px-6 py-4 text-xs font-bold text-slate-500 uppercase tracking-wider border-b border-border w-16 text-center">
                                          "อันดับ"
                                      </th>
                                      <th class="px-6 py-4 text-xs font-bold text-slate-500 uppercase tracking-wider border-b border-border">
                                          "ชื่อรายการยา"
                                      </th>
                                      <th class="px-6 py-4 text-xs font-bold text-slate-500 uppercase tracking-wider border-b border-border text-right">
                                          "มูลค่าจัดซื้อรวม"
                                      </th>
                                  </tr>
                              </thead>
                              <tbody class="text-text-primary text-sm divide-y divide-border-muted">
                                  <For
                                      each=move || ranked.clone()
                                      key=|(_, h)| h.name.clone()
                                      children=move |(i, h)| {
                                          let rank = i + 1;
                                          let badge = badge_class(i);
                                          view! {
                                              <tr class="group hover:bg-slate-50 transition-colors">
                                                  <td class="px-6 py-4 text-center">
                                                      <Show
                                                          when=move || i < 3
                                                          fallback=move || {
                              view! {
                                  <span class="text-slate-400 font-medium text-sm">{ rank }</span>
                              }
                          }
                                                      >
                                                          <div class=format!(
                              "inline-flex items-center justify-center w-8 h-8 rounded-lg text-sm font-bold shadow-sm {badge}",
                          )>
                                                              { rank }
                                                          </div>
                                                      </Show>
                                                  </td>
                                                  <td class="px-6 py-4">
                                                      <span class="font-semibold text-text-primary group-hover:text-primary transition-colors block">
                                                          { h.name }
                                                      </span>
                                                  </td>
                                                  <td class="px-6 py-4 text-right">
                                                      <span class="font-bold text-text-primary tabular-nums text-base">
                                                          { format_thb(h.total_value) }
                                                      </span>
                                                  </td>
                                              </tr>
                                          }
                                      }
                                  />
                              </tbody>
                          </table>
                      }
                  }
              >
                  <div class="flex flex-col items-center justify-center py-16 text-text-muted">
                      <div class="w-16 h-16 bg-slate-50 rounded-full flex items-center justify-center mb-4">
                          <svg
                              class="w-8 h-8 text-slate-300"
                              fill="none"
                              stroke="currentColor"
                              viewBox="0 0 24 24"
                          >
                              <path
                                  stroke-linecap="round"
                                  stroke-linejoin="round"
                                  stroke-width="1.5"
                                  d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4"
                              />
                          </svg>
                      </div>
                      <p class="text-sm font-medium">"ไม่พบข้อมูล"</p>
                  </div>
              </Show>
          </div>
      </div>
  }
}
