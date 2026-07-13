use chrono::{Datelike, Utc};

/// Current Thai fiscal year as a พ.ศ. (Buddhist Era) integer.
///
/// The Thai fiscal year starts in October: if we are in or after October we are
/// already in the *next* AD year's fiscal period.
pub fn current_thai_fiscal_year() -> i32 {
  let now = Utc::now();
  let month = now.month(); // 1-12
  let year_ad = now.year();
  let fiscal_year_ad = if month >= 10 { year_ad + 1 } else { year_ad };
  fiscal_year_ad + 543
}

/// The current date formatted in Thai locale, e.g. "13 กรกฎาคม 2569".
///
/// `Intl.DateTimeFormat` is unavailable on WASM, so we build the string with a
/// small lookup table.
pub fn today_thai_long() -> String {
  let now = Utc::now();
  let day = now.day();
  let month = now.month() as usize;
  let year_be = now.year() + 543;

  const MONTHS: [&str; 12] = [
    "มกราคม",
    "กุมภาพันธ์",
    "มีนาคม",
    "เมษายน",
    "พฤษภาคม",
    "มิถุนายน",
    "กรกฎาคม",
    "สิงหาคม",
    "กันยายน",
    "ตุลาคม",
    "พฤศจิกายน",
    "ธันวาคม",
  ];

  format!("{day} {} {year_be}", MONTHS[month - 1])
}
