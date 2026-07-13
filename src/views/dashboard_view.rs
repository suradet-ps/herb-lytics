use leptos::prelude::*;

use crate::components::dashboard_layout::DashboardLayout;
use crate::components::herb_chart::HerbChart;
use crate::components::herb_table::HerbTable;
use crate::components::summary_cards::SummaryCards;
use crate::core::types::HerbSummary;
use crate::stores::use_dashboard;

#[component]
pub fn DashboardView() -> impl IntoView {
  let state = use_dashboard();

  // Fetch on mount and whenever the selected fiscal year changes.
  Effect::new(move |_| {
    let _ = state.selected_year.get();
    leptos::task::spawn_local(async move {
      let _ = state.fetch().await;
    });
  });

  let selected_year = state.selected_year;
  let loading = Signal::derive(move || state.loading.get());
  let error = Signal::derive(move || state.error.get());
  let summary: Signal<Option<HerbSummary>> = Signal::derive(move || state.summary.get());
  let years = Signal::derive(move || state.available_years.get());

  let on_year_change = move |ev| {
    let value = event_target_value(&ev)
      .parse::<i32>()
      .unwrap_or_else(|_| selected_year.get_untracked());
    selected_year.set(value);
  };

  view! {
      <DashboardLayout
          controls=Box::new(move || {
              view! {
                  <div class="flex items-center gap-3">
                      <label
                          for="year-select"
                          class="text-sm font-medium text-text-secondary hidden sm:block"
                      >
                          "ปีงบประมาณ (พ.ศ.):"
                      </label>
                      <div class="relative group">
                          <select
                              id="year-select"
                              class="appearance-none bg-surface border border-border text-text-primary text-sm rounded-lg focus:ring-2 focus:ring-primary/20 focus:border-primary block w-full py-2 pl-4 pr-10 cursor-pointer font-medium shadow-sm hover:border-primary/50 transition-colors"
                              on:change=on_year_change
                          >
                              <For
                                  each=move || years.get()
                                  key=|y| *y
                                  children=move |y| {
                                      view! {
                                          <option
                                              value=y.to_string()
                                              selected=move || y == selected_year.get()
                                          >
                                              { y }
                                          </option>
                                      }
                                  }
                              />
                          </select>
                          <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-3 text-text-muted group-hover:text-primary transition-colors">
                              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                  <path
                                      stroke-linecap="round"
                                      stroke-linejoin="round"
                                      stroke-width="2"
                                      d="M19 9l-7 7-7-7"
                                  />
                              </svg>
                          </div>
                      </div>
                  </div>
              }
              .into_any()
          })
      >
          <Show
              when=move || loading.get()
              fallback=move || {
                  view! {
                      <Show
                          when=move || error.get().is_some()
                          fallback=move || {
                              view! {
                                  <DashboardContent summary=summary />
                              }
                          }
                      >
                          <div class="bg-error-bg border border-error/20 text-error rounded-2xl p-8 text-center shadow-sm max-w-md mx-auto mt-10 animate-fade-in">
                              <div class="w-12 h-12 mx-auto mb-4 bg-error/10 rounded-full flex items-center justify-center">
                                  <svg
                                      class="w-6 h-6 text-error"
                                      fill="none"
                                      stroke="currentColor"
                                      viewBox="0 0 24 24"
                                  >
                                      <path
                                          stroke-linecap="round"
                                          stroke-linejoin="round"
                                          stroke-width="2"
                                          d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                                      />
                                  </svg>
                              </div>
                              <h3 class="font-bold text-lg mb-2 text-text-primary">
                                  "เกิดข้อผิดพลาด"
                              </h3>
                              <p class="text-text-secondary mb-6 text-sm">
                                  { error.get().unwrap_or_default() }
                              </p>
                              <button
                                  class="inline-flex items-center gap-2 px-4 py-2 bg-white border border-border hover:bg-bg-alt text-text-primary rounded-lg text-sm font-medium transition-colors shadow-sm"
                                  on:click=move |_| {
                                      leptos::task::spawn_local(async move {
                                          let _ = state.fetch().await;
                                      });
                                  }
                              >
                                  <svg
                                      class="w-4 h-4"
                                      fill="none"
                                      stroke="currentColor"
                                      viewBox="0 0 24 24"
                                  >
                                      <path
                                          stroke-linecap="round"
                                          stroke-linejoin="round"
                                          stroke-width="2"
                                          d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
                                      />
                                  </svg>
                                  "ลองใหม่อีกครั้ง"
                              </button>
                          </div>
                      </Show>
                  }
              }
          >
              <div class="flex flex-col items-center justify-center h-96">
                  <div class="relative">
                      <div class="w-12 h-12 rounded-full border-4 border-border-muted"></div>
                      <div class="absolute inset-0 w-12 h-12 rounded-full border-4 border-transparent border-t-primary animate-spin">
                      </div>
                  </div>
                  <p class="mt-4 text-text-muted font-medium text-sm animate-pulse">
                      "กำลังโหลดข้อมูล..."
                  </p>
              </div>
          </Show>
      </DashboardLayout>
  }
}

#[component]
fn DashboardContent(summary: Signal<Option<HerbSummary>>) -> impl IntoView {
  view! {
      <Show
          when=move || summary.get().is_some()
          fallback=|| view! { <div></div> }
      >
          {move || {
              let s = summary.get().expect("guarded by Show");
              view! {
                  <SummaryCards summary=s.clone() />
                  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                      <HerbChart herbs=s.herbs.clone() />
                      <HerbTable herbs=s.herbs.clone() />
                  </div>
              }
          }}
      </Show>
  }
}
