use leptos::prelude::*;
use leptos_meta::*;

use crate::views::dashboard_view::DashboardView;

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  view! {
      <Title text="รายงานมูลค่าการจัดซื้อยาสมุนไพร | Herb-lytics" />
      <DashboardView />
  }
}
