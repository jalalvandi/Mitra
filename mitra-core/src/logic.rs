//  ~/mitra-core/src/logic.rs
//
//  Contains the core application logic, decoupled from any UI (CLI or GUI).

use crate::utils::map_mitra_error;
use anyhow::{Context, Result};
use chrono::Duration;
use parsidate::{ParsiDate, ParsiDateTime};

/// Represents detailed information about a specific date.
#[derive(Debug)]
pub struct DateInfo {
    pub parsi_date_formatted: String,
    pub parsi_time_formatted: Option<String>,
    pub weekday: String,
    pub day_of_year: u32,
    pub days_in_month: u32,
    pub is_leap_year: bool,
    pub gregorian_equivalent: String,
    pub first_day_of_month: String,
    pub last_day_of_month: String,
    pub first_day_of_year: String,
    pub last_day_of_year: String,
}

// --- Logic Functions ---

pub fn get_current_parsi_datetime() -> Result<ParsiDateTime> {
    ParsiDateTime::now().context("Failed to get current Parsi datetime")
}

pub fn add_to_datetime(
    base_pdt: ParsiDateTime,
    days: Option<i64>,
    months: Option<i32>,
    years: Option<i32>,
    hours: Option<i64>,
    minutes: Option<i64>,
    seconds: Option<i64>,
) -> Result<ParsiDateTime> {
    if let Some(d) = days {
        base_pdt
            .add_days(d)
            .map_err(|e| map_mitra_error(e, "adding days"))
    } else if let Some(m) = months {
        base_pdt
            .add_months(m)
            .map_err(|e| map_mitra_error(e, "adding months"))
    } else if let Some(y) = years {
        base_pdt
            .add_years(y)
            .map_err(|e| map_mitra_error(e, "adding years"))
    } else if let Some(h) = hours {
        base_pdt
            .add_duration(Duration::hours(h))
            .map_err(|e| map_mitra_error(e, "adding hours"))
    } else if let Some(m) = minutes {
        base_pdt
            .add_duration(Duration::minutes(m))
            .map_err(|e| map_mitra_error(e, "adding minutes"))
    } else if let Some(s) = seconds {
        base_pdt
            .add_duration(Duration::seconds(s))
            .map_err(|e| map_mitra_error(e, "adding seconds"))
    } else {
        unreachable!("No duration unit provided, should be caught by CLI parser.");
    }
}

pub fn sub_from_datetime(
    base_pdt: ParsiDateTime,
    days: Option<u64>,
    months: Option<u32>,
    years: Option<u32>,
    hours: Option<u64>,
    minutes: Option<u64>,
    seconds: Option<u64>,
) -> Result<ParsiDateTime> {
    if let Some(d) = days {
        base_pdt
            .sub_days(d)
            .map_err(|e| map_mitra_error(e, "subtracting days"))
    } else if let Some(m) = months {
        base_pdt
            .sub_months(m)
            .map_err(|e| map_mitra_error(e, "subtracting months"))
    } else if let Some(y) = years {
        base_pdt
            .sub_years(y)
            .map_err(|e| map_mitra_error(e, "subtracting years"))
    } else if let Some(h) = hours {
        let h_i64 = h.try_into().context("Hour value too large")?;
        base_pdt
            .sub_duration(Duration::hours(h_i64))
            .map_err(|e| map_mitra_error(e, "subtracting hours"))
    } else if let Some(m) = minutes {
        let m_i64 = m.try_into().context("Minute value too large")?;
        base_pdt
            .sub_duration(Duration::minutes(m_i64))
            .map_err(|e| map_mitra_error(e, "subtracting minutes"))
    } else if let Some(s) = seconds {
        let s_i64 = s.try_into().context("Second value too large")?;
        base_pdt
            .sub_duration(Duration::seconds(s_i64))
            .map_err(|e| map_mitra_error(e, "subtracting seconds"))
    } else {
        unreachable!("No duration unit provided, should be caught by CLI parser.");
    }
}

pub fn get_days_diff(pdt1: ParsiDateTime, pdt2: ParsiDateTime) -> Result<u64> {
    let diff_i64 = pdt1
        .date()
        .days_between(&pdt2.date())
        .map_err(|e| map_mitra_error(e, "calculating date difference"))?;

    // Take the absolute value and cast it to u64.
    // This is safe because .abs() always returns a non-negative i64.
    Ok(diff_i64.unsigned_abs())
}

pub fn get_weekday(pdt: ParsiDateTime) -> Result<String> {
    pdt.date()
        .weekday()
        .map_err(|e| map_mitra_error(e, "getting weekday"))
}

pub fn convert_to_gregorian(pdt: ParsiDateTime) -> Result<chrono::NaiveDateTime> {
    pdt.to_gregorian()
        .map_err(|e| map_mitra_error(e, "converting to Gregorian"))
}

pub fn convert_from_gregorian(gregorian_ndt: chrono::NaiveDateTime) -> Result<ParsiDateTime> {
    ParsiDateTime::from_gregorian(gregorian_ndt)
        .map_err(|e| map_mitra_error(e, "converting from Gregorian"))
}

pub fn is_leap_year(year: i32) -> bool {
    ParsiDate::is_persian_leap_year(year)
}

pub fn get_date_info(pdt: ParsiDateTime, was_datetime: bool) -> Result<DateInfo> {
    let date = pdt.date();
    let g_ndt = convert_to_gregorian(pdt)?;

    Ok(DateInfo {
        parsi_date_formatted: date.to_string(),
        parsi_time_formatted: if was_datetime {
            Some(pdt.format("%T").to_string())
        } else {
            None
        },
        weekday: get_weekday(pdt)?,
        day_of_year: date
            .ordinal()
            .map_err(|e| map_mitra_error(e, "getting ordinal"))?,
        days_in_month: ParsiDate::days_in_month(date.year(), date.month()),
        is_leap_year: is_leap_year(date.year()),
        gregorian_equivalent: if was_datetime {
            g_ndt.format("%Y-%m-%d %H:%M:%S").to_string()
        } else {
            g_ndt.format("%Y-%m-%d").to_string()
        },
        first_day_of_month: date.first_day_of_month().to_string(),
        last_day_of_month: date.last_day_of_month().to_string(),
        first_day_of_year: date.first_day_of_year().to_string(),
        last_day_of_year: date.last_day_of_year().to_string(),
    })
}
