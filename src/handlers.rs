//  ~/mitra-cli/src/handlers.rs
//
//  Contains the handler functions for each CLI subcommand. These functions
//  act as a bridge between the CLI arguments and the `mitra-core` logic.
//  They parse inputs, call core functions, and print results to the console.

use crate::cli::FormatStyle;
use anyhow::{Context, Result, bail};
use mitra_core::{
    self, ParsiDate, ParsiDateTime, events,
    utils::{map_mitra_error, parse_input_datetime_or_date, print_result},
};
use std::collections::VecDeque;

// =====================================================================================
// Calendar Display Logic (CLI-Specific)
// This section remains within the CLI crate because its output is text-based.
// =====================================================================================

/// Generates the lines of text representing a single month's calendar grid.
/// Returns a Vec<String>, where each string is a line (header, weekdays, days).
/// Includes event indicators and today highlighting.
fn generate_month_lines(year: i32, month: u32, today: &ParsiDate) -> Result<Vec<String>> {
    // --- Width Configuration ---
    let day_width = 2;
    let indicator_width = 1;
    let cell_padding = 1;
    let cell_width = day_width + indicator_width + cell_padding;
    let total_width = (7 * cell_width) - cell_padding;

    let mut lines: Vec<String> = Vec::with_capacity(8);

    if !(1..=12).contains(&month) {
        return Ok(vec![format!("Invalid Month: {}", month)]);
    }
    let first_day_of_month = ParsiDate::new(year, month, 1)
        .map_err(|e| map_mitra_error(e, &format!("creating date {}-{}-1", year, month)))?;

    let month_name = first_day_of_month.format("%B");

    let first_weekday_name = first_day_of_month
        .weekday()
        .map_err(|e| map_mitra_error(e, &format!("getting weekday for {}-{}-1", year, month)))?;
    let first_weekday: u32 = match first_weekday_name.as_str() {
        "شنبه" => 0,
        "یکشنبه" => 1,
        "دوشنبه" => 2,
        "سه‌شنبه" => 3,
        "چهارشنبه" => 4,
        "پنجشنبه" => 5,
        "جمعه" => 6,
        _ => bail!("Unexpected weekday name: {}", first_weekday_name),
    };

    let days_in_month = ParsiDate::days_in_month(year, month);
    if days_in_month == 0 {
        return Ok(vec![format!("Invalid days for {}-{}", year, month)]);
    }

    // --- Build Lines ---
    let header = format!("{} {}", month_name, year);
    lines.push(format!("{:^width$}", header, width = total_width));

    lines.push(" Sat Sun Mon Tue Wed Thu Fri".to_string());

    let mut current_line = String::with_capacity(total_width);
    current_line.push_str(&" ".repeat((first_weekday * cell_width as u32) as usize));

    let is_current_month_year = year == today.year() && month == today.month();
    let today_day_num = if is_current_month_year {
        Some(today.day())
    } else {
        None
    };

    for day in 1..=days_in_month {
        let is_today = today_day_num == Some(day);
        let event_indicator = events::get_event_indicator(year, month, day).unwrap_or(' ');

        let start_highlight = if is_today { "\x1b[7m" } else { "" };
        let end_highlight = if is_today { "\x1b[0m" } else { "" };

        current_line.push_str(&format!(
            "{}{:width$}{}{}{}",
            start_highlight,
            day,
            event_indicator,
            end_highlight,
            " ".repeat(cell_padding),
            width = day_width
        ));

        let current_weekday = (first_weekday + day - 1) % 7;

        if current_weekday == 6 || day == days_in_month {
            lines.push(format!(
                "{:<width$}",
                current_line.trim_end(),
                width = total_width
            ));
            current_line.clear();
        }
    }

    let empty_line = " ".repeat(total_width);
    while lines.len() < 8 {
        lines.push(empty_line.clone());
    }

    Ok(lines)
}

pub fn handle_cal(
    month_opt: Option<u32>,
    year_opt: Option<i32>,
    three_months: bool,
    year_to_show_opt: Option<i32>,
) -> Result<()> {
    let today = ParsiDate::today().context("Failed to get today's date")?;

    if let Some(year_to_show) = year_to_show_opt {
        // Full Year Mode
        println!("{:^83}", year_to_show); // Adjusted width for 3 months side-by-side

        let mut all_months_lines: Vec<Vec<String>> = Vec::with_capacity(12);
        for m in 1..=12 {
            let lines = generate_month_lines(year_to_show, m, &today)?;
            all_months_lines.push(lines);
        }

        // Print months in 4 rows of 3
        for row in 0..4 {
            let month_indices = (row * 3)..(row * 3 + 3);
            let months_in_row: Vec<_> = month_indices
                .clone()
                .filter_map(|i| all_months_lines.get(i))
                .collect();
            if months_in_row.is_empty() {
                continue;
            }

            // Assuming all months generate 8 lines
            for line_idx in 0..8 {
                let mut row_line = String::new();
                for (i, month_lines) in months_in_row.iter().enumerate() {
                    row_line.push_str(month_lines.get(line_idx).map_or("", |s| s.as_str()));
                    if i < months_in_row.len() - 1 {
                        row_line.push_str("  "); // Spacer between calendars
                    }
                }
                println!("{}", row_line);
            }
            println!(); // Blank line between rows of months
        }
    } else if three_months {
        // Three Month Mode
        let target_year = today.year();
        let target_month = today.month();

        let (prev_year, prev_month) = if target_month == 1 {
            (target_year - 1, 12)
        } else {
            (target_year, target_month - 1)
        };
        let (next_year, next_month) = if target_month == 12 {
            (target_year + 1, 1)
        } else {
            (target_year, target_month + 1)
        };

        let prev_lines = generate_month_lines(prev_year, prev_month, &today)?;
        let current_lines = generate_month_lines(target_year, target_month, &today)?;
        let next_lines = generate_month_lines(next_year, next_month, &today)?;

        for i in 0..prev_lines.len() {
            println!(
                "{}  {}  {}",
                prev_lines.get(i).map_or("", |s| s.as_str()),
                current_lines.get(i).map_or("", |s| s.as_str()),
                next_lines.get(i).map_or("", |s| s.as_str())
            );
        }
    } else {
        // Single Month Mode
        let target_year: i32;
        let target_month: u32;

        if let Some(month_num) = month_opt {
            target_month = month_num;
            target_year = year_opt.unwrap_or_else(|| today.year());
            if !(1..=12).contains(&target_month) {
                bail!("Error: Month must be between 1 and 12.");
            }
        } else {
            if year_opt.is_some() {
                bail!("Error: Year cannot be specified without a month in single month mode.");
            }
            target_month = today.month();
            target_year = today.year();
        }

        let lines = generate_month_lines(target_year, target_month, &today)?;
        for line in lines {
            println!("{}", line);
        }
    }

    println!("\n*: Holiday  +: Other Event");
    Ok(())
}

// =====================================================================================
// Handlers using mitra-core
// =====================================================================================

/// Handles the `now` command by calling the core logic.
pub fn handle_now() -> Result<()> {
    let now = mitra_core::get_current_parsi_datetime()?;
    println!("{}", now);
    Ok(())
}

/// Handles the `add` command.
pub fn handle_add(
    base_dt_str: String,
    days: Option<i64>,
    months: Option<i32>,
    years: Option<i32>,
    hours: Option<i64>,
    minutes: Option<i64>,
    seconds: Option<i64>,
) -> Result<()> {
    let (base_pdt, was_datetime) = parse_input_datetime_or_date(&base_dt_str)?;
    let result_pdt =
        mitra_core::add_to_datetime(base_pdt, days, months, years, hours, minutes, seconds)?;
    print_result(result_pdt, was_datetime);
    Ok(())
}

/// Handles the `sub` command.
pub fn handle_sub(
    base_dt_str: String,
    days: Option<u64>,
    months: Option<u32>,
    years: Option<u32>,
    hours: Option<u64>,
    minutes: Option<u64>,
    seconds: Option<u64>,
) -> Result<()> {
    let (base_pdt, was_datetime) = parse_input_datetime_or_date(&base_dt_str)?;
    let result_pdt =
        mitra_core::sub_from_datetime(base_pdt, days, months, years, hours, minutes, seconds)?;
    print_result(result_pdt, was_datetime);
    Ok(())
}

/// Handles the `format` command.
pub fn handle_format(
    datetime_string: String,
    style: Option<FormatStyle>,
    pattern: Option<String>,
) -> Result<()> {
    if style.is_none() && pattern.is_none() {
        bail!("Error: Please provide either --style or --pattern for formatting.");
    }

    let (pdt, was_datetime) = parse_input_datetime_or_date(&datetime_string)?;

    let formatted_string = match style {
        Some(FormatStyle::Short) => {
            if was_datetime {
                pdt.format("%Y/%m/%d %H:%M:%S")
            } else {
                pdt.date().format("short")
            }
        }
        Some(FormatStyle::Long) => pdt.date().format("long"),
        Some(FormatStyle::Iso) => {
            if was_datetime {
                pdt.format("%Y-%m-%dT%T")
            } else {
                pdt.date().format("iso")
            }
        }
        None => pdt.format(pattern.as_ref().unwrap()),
    };

    println!("{}", formatted_string);
    Ok(())
}

/// Handles the `diff` command.
pub fn handle_diff(dt_str1: String, dt_str2: String) -> Result<()> {
    let (pdt1, _) = parse_input_datetime_or_date(&dt_str1)
        .with_context(|| format!("Failed to parse first date: {}", dt_str1))?;
    let (pdt2, _) = parse_input_datetime_or_date(&dt_str2)
        .with_context(|| format!("Failed to parse second date: {}", dt_str2))?;
    let days_diff = mitra_core::get_days_diff(pdt1, pdt2)?;
    println!("Difference: {} days", days_diff);
    Ok(())
}

/// Handles the `weekday` command.
pub fn handle_weekday(date_str: String) -> Result<()> {
    let (pdt, _) = parse_input_datetime_or_date(&date_str)
        .with_context(|| format!("Failed to parse date: {}", date_str))?;
    let weekday_name = mitra_core::get_weekday(pdt)?;
    println!("{}", weekday_name);
    Ok(())
}

/// Handles the `to-gregorian` command.
pub fn handle_to_gregorian(parsi_dt_str: String) -> Result<()> {
    let (pdt, was_datetime) = parse_input_datetime_or_date(&parsi_dt_str)
        .with_context(|| format!("Failed to parse Parsi date: {}", parsi_dt_str))?;
    let gregorian_ndt = mitra_core::convert_to_gregorian(pdt)?;
    if was_datetime {
        println!("{}", gregorian_ndt.format("%Y-%m-%d %H:%M:%S"));
    } else {
        println!("{}", gregorian_ndt.format("%Y-%m-%d"));
    }
    Ok(())
}

/// Handles the `from-gregorian` command.
pub fn handle_from_gregorian(gregorian_dt_str: String) -> Result<()> {
    // This parsing logic is specific to CLI input formats, so it remains here.
    let trimmed_input = gregorian_dt_str.trim();
    let (gregorian_ndt, was_datetime) = chrono::NaiveDateTime::parse_from_str(
        trimmed_input,
        "%Y-%m-%d %H:%M:%S",
    )
    .map(|ndt| (ndt, true))
    .or_else(|_| {
        chrono::NaiveDateTime::parse_from_str(trimmed_input, "%Y-%m-%dT%H:%M:%S")
            .map(|ndt| (ndt, true))
    })
    .or_else(|_| {
        chrono::NaiveDate::parse_from_str(trimmed_input, "%Y-%m-%d")
            .map(|nd| (nd.and_hms_opt(0, 0, 0).unwrap(), false))
    })
    .with_context(|| {
        format!(
            "Could not parse Gregorian date/datetime '{}'. Use YYYY-MM-DD or YYYY-MM-DDTHH:MM:SS",
            trimmed_input
        )
    })?;

    let parsi_pdt = mitra_core::convert_from_gregorian(gregorian_ndt)?;
    print_result(parsi_pdt, was_datetime);
    Ok(())
}

/// Handles the `is-leap` command.
pub fn handle_is_leap(year: i32) -> Result<()> {
    if year <= 0 {
        bail!("Error: Year must be a positive number.");
    }
    let is_leap = mitra_core::is_leap_year(year);
    println!("{}", if is_leap { "Yes" } else { "No" });
    Ok(())
}

/// Handles the `info` command.
pub fn handle_info(datetime_string: String) -> Result<()> {
    let (pdt, was_datetime) = parse_input_datetime_or_date(&datetime_string)?;
    let info = mitra_core::get_date_info(pdt, was_datetime)?;

    println!("Input Parsi Date/Time: {}", datetime_string);
    println!("-------------------------");
    println!(" Parsed Date: {}", info.parsi_date_formatted);
    if let Some(time) = info.parsi_time_formatted {
        println!(" Parsed Time: {}", time);
    }
    println!(" Weekday: {}", info.weekday);
    println!(" Day of Year: {}", info.day_of_year);
    println!(" Days in Current Month: {}", info.days_in_month);
    println!(
        " Is Leap Year: {}",
        if info.is_leap_year { "Yes" } else { "No" }
    );
    println!(" Gregorian Equivalent: {}", info.gregorian_equivalent);
    println!(" First Day of Month: {}", info.first_day_of_month);
    println!(" Last Day of Month: {}", info.last_day_of_month);
    println!(" First Day of Year: {}", info.first_day_of_year);
    println!(" Last Day of Year: {}", info.last_day_of_year);
    Ok(())
}

/// Handles the `parse` command.
pub fn handle_parse(input_string: String, pattern: String) -> Result<()> {
    // This logic is simple enough to be kept in the handler.
    let expects_time = pattern.contains("%H")
        || pattern.contains("%M")
        || pattern.contains("%S")
        || pattern.contains("%T");

    if expects_time {
        let parsed_dt = ParsiDateTime::parse(&input_string, &pattern)
            .map_err(|e| map_mitra_error(e, "parsing datetime with explicit format"))?;
        println!("Parsed DateTime: {}", parsed_dt);
    } else {
        let parsed_d = ParsiDate::parse(&input_string, &pattern)
            .map_err(|e| map_mitra_error(e, "parsing date with explicit format"))?;
        println!("Parsed Date: {}", parsed_d);
    }
    Ok(())
}

/// Handles the `events` command.
pub fn handle_events(date_string: String) -> Result<()> {
    let (pdt, _) = parse_input_datetime_or_date(&date_string)
        .with_context(|| format!("Failed to parse date string: {}", date_string))?;

    let display_date = pdt.format("%A، %d %B %Y");

    println!("Events for {}:", display_date);

    if let Some(events_list) = events::get_events_for_date(pdt.year(), pdt.month(), pdt.day()) {
        for event in events_list {
            let prefix = if event.holiday { "[تعطیل] " } else { "- " };
            println!("  {}{}", prefix, event.title);
        }
    } else {
        println!("  - No events found.");
    }

    Ok(())
}
