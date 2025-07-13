use chrono::{Datelike, NaiveDateTime};
use clap::ValueEnum;
use serde::Deserialize;

pub struct DateFilter {
    quarter: Option<Quarter>,
    year: Option<i32>,
}

impl DateFilter {
    pub fn new(quarter: Option<Quarter>, year: Option<i32>) -> Self {
        Self { quarter, year }
    }

    pub fn matches(&self, date: NaiveDateTime) -> bool {
        if let Some(y) = self.year {
            if date.year() != y {
                return false;
            }
        }
        if let Some(q) = self.quarter {
            if !q.matches(date) {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, ValueEnum)]
pub enum Quarter {
    Q1,
    Q2,
    Q3,
    Q4,
}

impl Quarter {
    pub fn matches(&self, date: NaiveDateTime) -> bool {
        match self {
            Quarter::Q1 => (1..=3).contains(&date.month()),
            Quarter::Q2 => (4..=6).contains(&date.month()),
            Quarter::Q3 => (7..=9).contains(&date.month()),
            Quarter::Q4 => (10..=12).contains(&date.month()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    #[test]
    fn accept_all_dates_filter() {
        let q1_2024 =
            NaiveDateTime::parse_from_str("2024-01-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q2_2024 =
            NaiveDateTime::parse_from_str("2024-04-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q3_2024 =
            NaiveDateTime::parse_from_str("2024-07-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q4_2024 =
            NaiveDateTime::parse_from_str("2024-10-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q1_2025 =
            NaiveDateTime::parse_from_str("2025-01-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q2_2025 =
            NaiveDateTime::parse_from_str("2025-04-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q3_2025 =
            NaiveDateTime::parse_from_str("2025-07-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q4_2025 =
            NaiveDateTime::parse_from_str("2025-10-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();

        let accept_all_filter = DateFilter {
            quarter: None,
            year: None,
        };

        assert!(accept_all_filter.matches(q1_2024));
        assert!(accept_all_filter.matches(q2_2024));
        assert!(accept_all_filter.matches(q3_2024));
        assert!(accept_all_filter.matches(q4_2024));
        assert!(accept_all_filter.matches(q1_2025));
        assert!(accept_all_filter.matches(q2_2025));
        assert!(accept_all_filter.matches(q3_2025));
        assert!(accept_all_filter.matches(q4_2025));
    }

    #[test]
    fn accept_year_filter() {
        let q1_2024 =
            NaiveDateTime::parse_from_str("2024-01-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q2_2024 =
            NaiveDateTime::parse_from_str("2024-04-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q3_2024 =
            NaiveDateTime::parse_from_str("2024-07-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q4_2024 =
            NaiveDateTime::parse_from_str("2024-10-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q1_2025 =
            NaiveDateTime::parse_from_str("2025-01-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q2_2025 =
            NaiveDateTime::parse_from_str("2025-04-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q3_2025 =
            NaiveDateTime::parse_from_str("2025-07-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q4_2025 =
            NaiveDateTime::parse_from_str("2025-10-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();

        let accept_all_filter = DateFilter {
            quarter: None,
            year: Some(2025),
        };

        assert!(!accept_all_filter.matches(q1_2024));
        assert!(!accept_all_filter.matches(q2_2024));
        assert!(!accept_all_filter.matches(q3_2024));
        assert!(!accept_all_filter.matches(q4_2024));
        assert!(accept_all_filter.matches(q1_2025));
        assert!(accept_all_filter.matches(q2_2025));
        assert!(accept_all_filter.matches(q3_2025));
        assert!(accept_all_filter.matches(q4_2025));
    }

    #[test]
    fn accept_quarter_filter() {
        let q1_2024 =
            NaiveDateTime::parse_from_str("2024-01-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q2_2024 =
            NaiveDateTime::parse_from_str("2024-04-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q3_2024 =
            NaiveDateTime::parse_from_str("2024-07-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q4_2024 =
            NaiveDateTime::parse_from_str("2024-10-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q1_2025 =
            NaiveDateTime::parse_from_str("2025-01-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q2_2025 =
            NaiveDateTime::parse_from_str("2025-04-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q3_2025 =
            NaiveDateTime::parse_from_str("2025-07-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q4_2025 =
            NaiveDateTime::parse_from_str("2025-10-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();

        let accept_all_filter = DateFilter {
            quarter: Some(Quarter::Q3),
            year: None,
        };

        assert!(!accept_all_filter.matches(q1_2024));
        assert!(!accept_all_filter.matches(q2_2024));
        assert!(accept_all_filter.matches(q3_2024));
        assert!(!accept_all_filter.matches(q4_2024));
        assert!(!accept_all_filter.matches(q1_2025));
        assert!(!accept_all_filter.matches(q2_2025));
        assert!(accept_all_filter.matches(q3_2025));
        assert!(!accept_all_filter.matches(q4_2025));
    }

    #[test]
    fn accept_quarter_and_year() {
        let q1_2024 =
            NaiveDateTime::parse_from_str("2024-01-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q2_2024 =
            NaiveDateTime::parse_from_str("2024-04-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q3_2024 =
            NaiveDateTime::parse_from_str("2024-07-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q4_2024 =
            NaiveDateTime::parse_from_str("2024-10-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q1_2025 =
            NaiveDateTime::parse_from_str("2025-01-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q2_2025 =
            NaiveDateTime::parse_from_str("2025-04-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q3_2025 =
            NaiveDateTime::parse_from_str("2025-07-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();
        let q4_2025 =
            NaiveDateTime::parse_from_str("2025-10-01 00:00:00", SIMPLE_DATE_FORMAT).unwrap();

        let accept_all_filter = DateFilter {
            quarter: Some(Quarter::Q3),
            year: Some(2024),
        };

        assert!(!accept_all_filter.matches(q1_2024));
        assert!(!accept_all_filter.matches(q2_2024));
        assert!(accept_all_filter.matches(q3_2024));
        assert!(!accept_all_filter.matches(q4_2024));
        assert!(!accept_all_filter.matches(q1_2025));
        assert!(!accept_all_filter.matches(q2_2025));
        assert!(!accept_all_filter.matches(q3_2025));
        assert!(!accept_all_filter.matches(q4_2025));
    }
    mod quarter {
        use chrono::Days;

        use super::*;

        #[test]
        fn check_all_dates() {
            let mut date =
                NaiveDateTime::parse_from_str("2024-12-31 00:00:00", SIMPLE_DATE_FORMAT).unwrap();

            for _ in 0..(31 + 28 + 31) {
                date = date.checked_add_days(Days::new(1)).unwrap();
                assert!(Quarter::Q1.matches(date));
                assert!(!Quarter::Q2.matches(date));
                assert!(!Quarter::Q3.matches(date));
                assert!(!Quarter::Q4.matches(date));
            }
            for _ in 0..(30 + 31 + 30) {
                date = date.checked_add_days(Days::new(1)).unwrap();
                assert!(!Quarter::Q1.matches(date));
                assert!(Quarter::Q2.matches(date));
                assert!(!Quarter::Q3.matches(date));
                assert!(!Quarter::Q4.matches(date));
            }
            for _ in 0..(31 + 31 + 30) {
                date = date.checked_add_days(Days::new(1)).unwrap();
                assert!(!Quarter::Q1.matches(date));
                assert!(!Quarter::Q2.matches(date));
                assert!(Quarter::Q3.matches(date));
                assert!(!Quarter::Q4.matches(date));
            }
            for _ in 0..(31 + 30 + 31) {
                date = date.checked_add_days(Days::new(1)).unwrap();
                assert!(!Quarter::Q1.matches(date));
                assert!(!Quarter::Q2.matches(date));
                assert!(!Quarter::Q3.matches(date));
                assert!(Quarter::Q4.matches(date));
            }
        }
    }
}
