/// Format a number with Thai/English thousands separators, no decimals.
///
/// Replaces `Intl.NumberFormat('th-TH')` which is unavailable on WASM.
pub fn format_number(value: f64) -> String {
  let rounded = if value.is_finite() {
    value.round() as i64
  } else {
    0
  };
  let negative = rounded < 0;
  let digits: String = rounded.unsigned_abs().to_string();
  let grouped: String = digits
    .chars()
    .rev()
    .enumerate()
    .map(|(i, c)| {
      if i != 0 && i % 3 == 0 {
        format!(",{c}")
      } else {
        c.to_string()
      }
    })
    .collect::<String>()
    .chars()
    .rev()
    .collect();
  if negative {
    format!("-{grouped}")
  } else {
    grouped
  }
}

/// Currency formatting with a leading ฿, e.g. "฿1,234".
pub fn format_thb(value: f64) -> String {
  format!("฿{}", format_number(value))
}

/// Currency formatting followed by "บาท", e.g. "1,234 บาท".
pub fn format_thb_full(value: f64) -> String {
  format!("{} บาท", format_number(value))
}

/// Compact axis label: 1_500_000 -> "1.5M", 12_000 -> "12K", else "1234".
pub fn format_axis(value: f64) -> String {
  if value >= 1_000_000.0 {
    format!("{:.1}M", value / 1_000_000.0)
  } else if value >= 1_000.0 {
    format!("{}K", (value / 1_000.0).round() as i64)
  } else {
    format!("{}", value.round() as i64)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn formats_thousands_separators() {
    assert_eq!(format_number(0.0), "0");
    assert_eq!(format_number(1234.0), "1,234");
    assert_eq!(format_number(1234567.0), "1,234,567");
    assert_eq!(format_number(-2500.0), "-2,500");
  }

  #[test]
  fn formats_currency() {
    assert_eq!(format_thb(1234.0), "฿1,234");
    assert_eq!(format_thb_full(500.0), "500 บาท");
  }

  #[test]
  fn formats_axis_compact() {
    assert_eq!(format_axis(950.0), "950");
    assert_eq!(format_axis(12_000.0), "12K");
    assert_eq!(format_axis(1_500_000.0), "1.5M");
  }
}
