//  ~/src/handlers.rs
//
//  * Copyright (C) Mohammad (Sina) Jalalvandi 2024-2025 <jalalvandi.sina@gmail.com>
//  * Package : mitra
//  * License : Apache-2.0
//  * Version : 2.2.0
//  * URL     : https://github.com/jalalvandi/Mitra
//  * Sign: mitra-20250412-bb54068aea85-656d1cd95639e412f55fa197c609b120
//
//! Contains the core logic functions (handlers) for each CLI subcommand.

use crate::cli::FormatStyle; // Import needed items from sibling modules
use crate::events;
use crate::utils::{map_parsidate_error, parse_input_datetime_or_date, print_result};
use anyhow::{Context, Result, bail};
use chrono::Duration; // Use chrono::Duration for time arithmetic
use parsidate::{ParsiDate, ParsiDateTime};

// --- Command Handler Functions ---

/// Handles the `now` command: Fetches and prints the current Parsi date and time.
pub fn handle_now() -> Result<()> {
    let now = ParsiDateTime::now().context("Failed to get current Parsi datetime")?;
    println!("{}", now); // Uses ParsiDateTime's Display trait
    Ok(())
}

/// Handles the `cal` command: Displays a monthly Parsi calendar.
pub fn handle_cal(month_opt: Option<u32>, year_opt: Option<i32>) -> Result<()> {
    let today = ParsiDate::today().context("Failed to get today's date")?;
    let year = year_opt.unwrap_or_else(|| today.year());

    // Determine the month
    let month = match month_opt {
        Some(m) => {
            if !(1..=12).contains(&m) {
                bail!("Error: Month must be between 1 and 12.");
            }
            m
        }
        None => {
            if year_opt.is_some() {
                bail!("Error: Year cannot be specified without a month.");
            }
            today.month()
        }
    };

    // Validate year/month by creating the first day
    let first_day_of_month = ParsiDate::new(year, month, 1)
        .map_err(|e| map_parsidate_error(e, "creating first day of target month"))?;

    // --- Get Calendar Data ---
    let month_name = first_day_of_month.format("%B");
    let days_in_month = ParsiDate::days_in_month(year, month);
    if days_in_month == 0 {
        bail!(
            "Error: Could not determine days in month {}-{}",
            year,
            month
        );
    }

    let first_weekday_name = first_day_of_month
        .weekday()
        .map_err(|e| map_parsidate_error(e, "getting weekday name of first day"))?;

    let first_weekday: u32 = match first_weekday_name.as_str() {
        "شنبه" => 0,
        "یکشنبه" => 1,
        "دوشنبه" => 2,
        "سه‌شنبه" => 3,
        "چهارشنبه" => 4,
        "پنج‌شنبه" => 5,
        "جمعه" => 6,
        _ => bail!("Error: Unexpected weekday name: {}", first_weekday_name),
    };

    // --- Print Calendar ---
    let total_width = 36;
    let header = format!("{} {}", month_name, year);
    println!("----------------------------");
    println!("{:^width$}", header, width = total_width);
    println!("");
    // Print English weekday names header
    // Each name takes 2 chars + 2 spaces = 4 chars
    println!(" Sa  Su  Mo  Tu  We  Th  Fr");

    // Print leading spaces for the first week
    // Each day slot takes 4 characters
    let padding = (first_weekday * 4) as usize;
    print!("{:width$}", "", width = padding);

    // Check if the displayed month is the current month for highlighting today
    let current_day_num = if year == today.year() && month == today.month() {
        Some(today.day())
    } else {
        None
    };

    // Print the days of the month
    for day in 1..=days_in_month {
        let is_today = current_day_num == Some(day);
        // Get event indicator ('*', '+', or None) for the specific year
        let event_indicator = events::get_event_indicator(year, month, day);

        // Formatting for highlighting today
        let start_highlight = if is_today { "\x1b[7m" } else { "" }; // Start reverse video
        let end_highlight = if is_today { "\x1b[0m" } else { "" }; // Reset formatting

        // Get the character to print after the day number
        let indicator_char = event_indicator.unwrap_or(' ');

        // Print the day cell: HighlightStart DayNumber Indicator HighlightEnd
        // Pad the number to 2 spaces (right aligned), add indicator, total 3 chars used
        // Add one trailing space for a total width of 4 characters per cell.
        print!(
            "{}{:>2}{} {}",
            start_highlight, day, indicator_char, end_highlight
        );
        // Note the space ^^^ added after end_highlight

        // Calculate current weekday (0-6) to handle line breaks
        let current_weekday = (first_weekday + day - 1) % 7;

        // If it's Friday (weekday 6) or the last day of the month, print a newline
        if current_weekday == 6 || day == days_in_month {
            println!();
        } else {
            // No extra space needed between cells due to the space added in the print! above
        }
    }

    // Print the legend for event indicators
    // Add some spacing before the legend
    println!("----------------------------");
    println!("\n*: Holiday  +: Other Event");
    println!("");

    Ok(())
}
/// Handles the `add` command: Adds a specified duration to a base date/datetime.
pub fn handle_add(
    base_dt_str: String,
    days: Option<i64>,
    months: Option<i32>,
    years: Option<i32>,
    hours: Option<i64>,
    minutes: Option<i64>,
    seconds: Option<i64>,
) -> Result<()> {
    // Validate that exactly one duration unit is provided (clap also helps here).
    let unit_count = [
        days,
        months.map(|i| i as i64),
        years.map(|i| i as i64),
        hours,
        minutes,
        seconds,
    ]
    .iter()
    .filter(|opt| opt.is_some())
    .count();

    if unit_count == 0 {
        bail!(
            "Error: Please specify exactly one duration unit (--days, --months, --years, --hours, --minutes, or --seconds) to add."
        );
    }
    if unit_count > 1 {
        bail!("Error: Please specify only one duration unit at a time.");
    }

    // Parse the base date/datetime input.
    let (base_pdt, was_datetime) = parse_input_datetime_or_date(&base_dt_str)?;

    // Perform the addition based on the provided unit.
    let result_pdt = if let Some(d) = days {
        base_pdt
            .add_days(d)
            .map_err(|e| map_parsidate_error(e, "adding days"))?
    } else if let Some(m) = months {
        base_pdt
            .add_months(m)
            .map_err(|e| map_parsidate_error(e, "adding months"))?
    } else if let Some(y) = years {
        base_pdt
            .add_years(y)
            .map_err(|e| map_parsidate_error(e, "adding years"))?
    } else if let Some(h) = hours {
        base_pdt
            .add_duration(Duration::hours(h))
            .map_err(|e| map_parsidate_error(e, "adding hours"))?
    } else if let Some(m) = minutes {
        base_pdt
            .add_duration(Duration::minutes(m))
            .map_err(|e| map_parsidate_error(e, "adding minutes"))?
    } else if let Some(s) = seconds {
        base_pdt
            .add_duration(Duration::seconds(s))
            .map_err(|e| map_parsidate_error(e, "adding seconds"))?
    } else {
        unreachable!("Logic error: No duration unit found.");
    };

    // Print the result appropriately.
    print_result(result_pdt, was_datetime);
    Ok(())
}

/// Handles the `sub` command: Subtracts a specified duration from a base date/datetime.
pub fn handle_sub(
    base_dt_str: String,
    days: Option<u64>,
    months: Option<u32>,
    years: Option<u32>,
    hours: Option<u64>,
    minutes: Option<u64>,
    seconds: Option<u64>,
) -> Result<()> {
    // Validate input unit count.
    let unit_count = [
        days.map(|u| u as i64),
        months.map(|u| u as i64),
        years.map(|u| u as i64),
        hours.map(|u| u as i64),
        minutes.map(|u| u as i64),
        seconds.map(|u| u as i64),
    ]
    .iter()
    .filter(|opt| opt.is_some())
    .count();

    if unit_count == 0 {
        bail!(
            "Error: Please specify exactly one duration unit (--days, --months, --years, --hours, --minutes, or --seconds) to subtract."
        );
    }
    if unit_count > 1 {
        bail!("Error: Please specify only one duration unit at a time.");
    }

    // Parse base input.
    let (base_pdt, was_datetime) = parse_input_datetime_or_date(&base_dt_str)?;

    // Perform subtraction.
    let result_pdt = if let Some(d) = days {
        base_pdt
            .sub_days(d)
            .map_err(|e| map_parsidate_error(e, "subtracting days"))?
    } else if let Some(m) = months {
        base_pdt
            .sub_months(m)
            .map_err(|e| map_parsidate_error(e, "subtracting months"))?
    } else if let Some(y) = years {
        base_pdt
            .sub_years(y)
            .map_err(|e| map_parsidate_error(e, "subtracting years"))?
    } else if let Some(h) = hours {
        // Convert u64 to i64 for Duration constructor
        let h_i64 = h
            .try_into()
            .context("Hour value too large for subtraction")?;
        base_pdt
            .sub_duration(Duration::hours(h_i64))
            .map_err(|e| map_parsidate_error(e, "subtracting hours"))?
    } else if let Some(m) = minutes {
        let m_i64 = m
            .try_into()
            .context("Minute value too large for subtraction")?;
        base_pdt
            .sub_duration(Duration::minutes(m_i64))
            .map_err(|e| map_parsidate_error(e, "subtracting minutes"))?
    } else if let Some(s) = seconds {
        let s_i64 = s
            .try_into()
            .context("Second value too large for subtraction")?;
        base_pdt
            .sub_duration(Duration::seconds(s_i64))
            .map_err(|e| map_parsidate_error(e, "subtracting seconds"))?
    } else {
        unreachable!();
    };

    // Print result.
    print_result(result_pdt, was_datetime);
    Ok(())
}

/// Handles the `format` command: Formats a date/datetime using a style or pattern.
pub fn handle_format(
    datetime_string: String,
    style: Option<FormatStyle>,
    pattern: Option<String>,
) -> Result<()> {
    // Ensure either style or pattern is provided (clap also checks conflicts).
    if style.is_none() && pattern.is_none() {
        bail!("Error: Please provide either --style or --pattern for formatting.");
    }

    // Parse input.
    let (pdt, was_datetime) = parse_input_datetime_or_date(&datetime_string)?;

    // Determine the format string to use.
    let formatted_string = match style {
        Some(FormatStyle::Short) => {
            // ParsiDate's "short" is YYYY/MM/DD. We add time if input had it.
            if was_datetime {
                pdt.format("%Y/%m/%d %H:%M:%S")
            } else {
                pdt.date().format("short")
            }
        }
        Some(FormatStyle::Long) => {
            // ParsiDate's "long" is "D Month YYYY". Time is usually omitted.
            pdt.date().format("long")
        }
        Some(FormatStyle::Iso) => {
            // ParsiDate's "iso" is YYYY-MM-DD. Add ISO time if input had it.
            if was_datetime {
                pdt.format("%Y-%m-%dT%T")
            } else {
                pdt.date().format("iso")
            }
        }
        None => {
            // Use the custom pattern provided.
            pdt.format(pattern.as_ref().unwrap())
        }
    };

    println!("{}", formatted_string);
    Ok(())
}

/// Handles the `diff` command: Calculates the difference in days between two dates.
pub fn handle_diff(dt_str1: String, dt_str2: String) -> Result<()> {
    let (pdt1, _) = parse_input_datetime_or_date(&dt_str1)
        .with_context(|| format!("Failed to parse first date/datetime: {}", dt_str1))?;
    let (pdt2, _) = parse_input_datetime_or_date(&dt_str2)
        .with_context(|| format!("Failed to parse second date/datetime: {}", dt_str2))?;

    // Calculate difference in days using ParsiDate::days_between (absolute value).
    let days_diff = pdt1
        .date()
        .days_between(&pdt2.date())
        .map_err(|e| map_parsidate_error(e, "calculating date difference"))?;

    println!("Difference: {} days", days_diff);
    Ok(())
}

/// Handles the `weekday` command: Prints the Persian weekday name for a given date.
pub fn handle_weekday(date_str: String) -> Result<()> {
    // Parse input, ignore time part.
    let (pdt, _) = parse_input_datetime_or_date(&date_str)
        .with_context(|| format!("Failed to parse date: {}", date_str))?;

    // Get weekday name.
    let weekday_name = pdt
        .date()
        .weekday()
        .map_err(|e| map_parsidate_error(e, "getting weekday"))?;

    println!("{}", weekday_name);
    Ok(())
}

/// Handles the `to-gregorian` command: Converts a Parsi date/datetime to Gregorian.
pub fn handle_to_gregorian(parsi_dt_str: String) -> Result<()> {
    let (pdt, was_datetime) = parse_input_datetime_or_date(&parsi_dt_str)
        .with_context(|| format!("Failed to parse Parsi date/datetime: {}", parsi_dt_str))?;

    // Convert.
    let gregorian_ndt = pdt
        .to_gregorian()
        .map_err(|e| map_parsidate_error(e, "converting to Gregorian"))?;

    // Print using standard Gregorian formats.
    if was_datetime {
        println!("{}", gregorian_ndt.format("%Y-%m-%d %H:%M:%S"));
    } else {
        println!("{}", gregorian_ndt.format("%Y-%m-%d"));
    }
    Ok(())
}

/// Handles the `from-gregorian` command: Converts a Gregorian date/datetime to Parsi.
pub fn handle_from_gregorian(gregorian_dt_str: String) -> Result<()> {
    let trimmed_input = gregorian_dt_str.trim();
    let mut was_datetime = false; // Track if the input included time

    // Try parsing common Gregorian formats (ISO and slash, DateTime first).
    let gregorian_ndt = chrono::NaiveDateTime::parse_from_str(trimmed_input, "%Y-%m-%d %H:%M:%S")
        .or_else(|_| chrono::NaiveDateTime::parse_from_str(trimmed_input, "%Y-%m-%dT%H:%M:%S"))
        .or_else(|_| chrono::NaiveDateTime::parse_from_str(trimmed_input, "%Y/%m/%d %H:%M:%S"))
        .inspect(|_ndt| {
            was_datetime = true; // Successfully parsed as DateTime
        })
        .or_else(|_| {
            // If DateTime parsing fails, try parsing as NaiveDate.
            chrono::NaiveDate::parse_from_str(trimmed_input, "%Y-%m-%d")
                .or_else(|_| chrono::NaiveDate::parse_from_str(trimmed_input, "%Y/%m/%d"))
                .map(|nd| {
                    was_datetime = false; // Successfully parsed as Date
                    // Convert NaiveDate to NaiveDateTime at midnight.
                    nd.and_hms_opt(0, 0, 0).unwrap() // 00:00:00 is always valid
                })
        })
        // If both fail, return an error.
        .with_context(|| format!("Could not parse Gregorian date/datetime '{}'. Use formats like YYYY-MM-DD, YYYY-MM-DD HH:MM:SS, or YYYY-MM-DDTHH:MM:SS", trimmed_input))?;

    // Convert the parsed Gregorian NaiveDateTime to ParsiDateTime.
    let parsi_pdt = ParsiDateTime::from_gregorian(gregorian_ndt)
        .map_err(|e| map_parsidate_error(e, "converting from Gregorian"))?;

    // Print the result based on whether the input seemed like a datetime or just a date.
    print_result(parsi_pdt, was_datetime);
    Ok(())
}

/// Handles the `is-leap` command: Checks if a Parsi year is a leap year.
pub fn handle_is_leap(year: i32) -> Result<()> {
    if year <= 0 {
        bail!("Error: Year must be a positive number.");
    }
    let is_leap = ParsiDate::is_persian_leap_year(year);
    println!("{}", if is_leap { "Yes" } else { "No" });
    Ok(())
}

/// Handles the `info` command: Displays detailed information about a date/datetime.
pub fn handle_info(datetime_string: String) -> Result<()> {
    let (pdt, was_datetime) = parse_input_datetime_or_date(&datetime_string)?;

    println!("Input Parsi Date/Time: {}", datetime_string);
    println!("-------------------------");

    // Basic Components
    println!(" Parsed Date: {}", pdt.date());
    if was_datetime {
        println!(
            " Parsed Time: {:02}:{:02}:{:02}",
            pdt.hour(),
            pdt.minute(),
            pdt.second()
        );
    }

    // Calculated Info (handle errors gracefully)
    match pdt.date().weekday() {
        Ok(wd) => println!(" Weekday: {}", wd),
        Err(e) => println!(" Weekday: Error ({})", e),
    }
    match pdt.date().ordinal() {
        Ok(ord) => println!(" Day of Year: {}", ord),
        Err(e) => println!(" Day of Year: Error ({})", e),
    }
    let days_in_mon = ParsiDate::days_in_month(pdt.year(), pdt.month());
    if days_in_mon > 0 {
        // Check if month was valid
        println!(" Days in Current Month: {}", days_in_mon);
    } else {
        println!(" Days in Current Month: N/A (Invalid Month?)");
    }
    let is_leap = ParsiDate::is_persian_leap_year(pdt.year());
    println!(" Is Leap Year: {}", if is_leap { "Yes" } else { "No" });

    // Gregorian Conversion
    match pdt.to_gregorian() {
        Ok(g_ndt) => {
            if was_datetime {
                println!(
                    " Gregorian Equivalent: {}",
                    g_ndt.format("%Y-%m-%d %H:%M:%S")
                );
            } else {
                println!(" Gregorian Equivalent: {}", g_ndt.format("%Y-%m-%d"));
            }
        }
        Err(e) => println!(" Gregorian Equivalent: Error ({})", e),
    }

    // Boundary Dates (safe to call on the parsed date)
    println!(" First Day of Month: {}", pdt.date().first_day_of_month());
    println!(" Last Day of Month: {}", pdt.date().last_day_of_month());
    println!(" First Day of Year: {}", pdt.date().first_day_of_year());
    println!(" Last Day of Year: {}", pdt.date().last_day_of_year());

    Ok(())
}

/// Handles the `parse` command: Parses a string using an explicit format pattern.
pub fn handle_parse(input_string: String, pattern: String) -> Result<()> {
    // Infer if the pattern expects time components
    let expects_time = pattern.contains("%H")
        || pattern.contains("%M")
        || pattern.contains("%S")
        || pattern.contains("%T");

    if expects_time {
        let parsed_dt = ParsiDateTime::parse(&input_string, &pattern)
            .map_err(|e| map_parsidate_error(e, "parsing datetime with explicit format"))?;
        println!("Parsed DateTime: {}", parsed_dt); // Use default Display
    } else {
        let parsed_d = ParsiDate::parse(&input_string, &pattern)
            .map_err(|e| map_parsidate_error(e, "parsing date with explicit format"))?;
        println!("Parsed Date: {}", parsed_d); // Use default Display
    }
    Ok(())
}

/// Handles the `events` command: Lists events for a specific date.
pub fn handle_events(date_string: String) -> Result<()> {
    // Parse the input date string (ignore time part)
    let (pdt, _) = parse_input_datetime_or_date(&date_string)
        .with_context(|| format!("Failed to parse date string: {}", date_string))?;

    let month = pdt.month();
    let day = pdt.day();
    let year = pdt.year();
    // Format the date for display (e.g., "6 مرداد")
    let display_date = pdt.format("%d %B"); // Or "%A %d %B" for weekday

    println!("Events for {}:", display_date);

    // Get events for the parsed date
    if let Some(events_list) = events::get_events_for_date(year, month, day) {
        if events_list.is_empty() {
            // This case shouldn't happen if get_events_for_date returns Some only when non-empty,
            // but good to handle defensively.
            println!("  - No events found.");
        } else {
            // Iterate and print each event title, marking holidays
            for event in events_list {
                let prefix = if event.holiday { "[تعطیل] " } else { "- " };
                // Optional: Include event_type if desired:
                // let prefix = if event.holiday { "[تعطیل] " } else { "" };
                // println!("  - {} ({}) {}", prefix, event.event_type, event.title);
                println!("  {}{}", prefix, event.title);
            }
        }
    } else {
        // If the date key wasn't found in the map
        println!("  - No events found.");
    }

    Ok(())
}
