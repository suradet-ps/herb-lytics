use leptos::prelude::*;

use chrono::Datelike;

use crate::core::time::today_thai_long;

#[component]
pub fn DashboardLayout(
  /// Top-right slot for controls (e.g. the fiscal-year selector).
  controls: Children,
  children: Children,
) -> impl IntoView {
  let year = chrono::Utc::now().year();

  view! {
      <div class="min-h-screen bg-bg font-sans text-text-primary flex flex-col">
          <header class="bg-linear-to-r from-slate-900 via-slate-800 to-slate-900 text-white shadow-xl sticky top-0 z-50 border-b-2 border-amber-500 overflow-hidden">
              <div class="absolute inset-0 opacity-10 bg-[radial-gradient(#ffffff33_1px,transparent_1px)] bg-size-[16px_16px]" />
              <div class="w-full mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
                  <div class="flex justify-between h-24 items-center">
                      <div class="flex items-center gap-3 sm:gap-4">
                          <div class="p-1.5 sm:p-2 bg-white/5 rounded-lg border border-white/10 backdrop-blur-sm shadow-inner">
                              <svg
                                  xmlns="http://www.w3.org/2000/svg"
                                  class="h-5 w-5 sm:h-6 sm:w-6 text-amber-400"
                                  fill="none"
                                  viewBox="0 0 24 24"
                                  stroke="currentColor"
                              >
                                  <path
                                      stroke-linecap="round"
                                      stroke-linejoin="round"
                                      stroke-width="2"
                                      d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                                  />
                              </svg>
                          </div>
                          <div>
                              <h1 class="font-bold text-xl sm:text-2xl tracking-tight leading-tight text-white drop-shadow-sm truncate max-w-[300px] sm:max-w-none">
                                  "รายงานมูลค่าการจัดซื้อยาสมุนไพร"
                              </h1>
                              <p class="text-sm sm:text-sm text-slate-300 uppercase tracking-wider font-medium truncate max-w-[300px] sm:max-w-none">
                                  "กลุ่มงานเภสัชกรรม โรงพยาบาลสระโบสถ์"
                              </p>
                          </div>
                      </div>

                      <div class="hidden sm:flex items-center gap-6">
                          <div class="flex items-center gap-3 text-right">
                              <span class="text-base text-amber-400 font-medium">"วันนี้"</span>
                              <span class="text-lg font-semibold text-white">{ today_thai_long() }</span>
                          </div>
                      </div>
                  </div>
              </div>
          </header>

          <main class="flex-1 max-w-7xl w-full mx-auto px-4 sm:px-6 lg:px-8 py-8 animate-fade-in">
              <div class="flex flex-col md:flex-row justify-between items-start md:items-end gap-4 mb-8 border-b border-border pb-6">
                  <div>
                      <h2 class="text-3xl font-bold text-text-primary tracking-tight">
                          "ภาพรวมการจัดซื้อสมุนไพร"
                      </h2>
                      <p class="text-text-secondary mt-1 text-lg">
                          "รายงานสรุปมูลค่าการจัดซื้อยาสมุนไพรประจำปีงบประมาณ"
                      </p>
                  </div>

                  <div class="w-full md:w-auto">{controls()}</div>
              </div>

              {children()}
          </main>

          <footer class="bg-white border-t border-border mt-auto">
              <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
                  <div class="flex justify-center items-center text-sm text-text-muted">
                      <p>{ format!("© {year} ฝ่ายเภสัชกรรม โรงพยาบาลสระโบสถ์.") }</p>
                  </div>
              </div>
          </footer>
      </div>
  }
}
