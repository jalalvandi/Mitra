//  ~/src/utils.rs
//
//  * Copyright (C) Mohammad (Sina) Jalalvandi 2024-2025 <jalalvandi.sina@gmail.com>
//  * Package : mitra
//  * License : Apache-2.0
//  * Version : 2.1.0
//  * URL     : https://github.com/jalalvandi/Mitra
//  * 714b5631-87ad-4fde-905f-89dc149387f2
//
//! Contains utility functions used by command handlers, such as parsing input strings,
//! printing results consistently, and mapping errors.

use anyhow::{Result, anyhow, bail};
use parsidate::{DateError, ParseErrorKind, ParsiDate, ParsiDateTime};

/// Attempts to parse the input string first as a ParsiDateTime, then as a ParsiDate,
/// trying common formats (slash-separated and ISO-like).
/// Returns the parsed ParsiDateTime and a boolean indicating if the input included time.
/// This is used by commands that accept flexible date/datetime input.
pub fn parse_input_datetime_or_date(input: &str) -> Result<(ParsiDateTime, bool)> {
    // Trim whitespace from input for robustness.
    let trimmed_input = input.trim();

    // Define common formats to try, prioritizing more specific ones (DateTime) first.
    let dt_formats = [
        "%Y/%m/%d %H:%M:%S", // Slash date, space time
        "%Y-%m-%dT%T",       // ISO date, T separator, T time macro (%H:%M:%S)
        "%Y-%m-%d %H:%M:%S", // ISO date, space time
                             // Add more formats if needed, e.g., with different separators or orders
                             // "%Y.%m.%d %H:%M:%S",
    ];
    let d_formats = [
        "%Y/%m/%d", // Slash date
        "%Y-%m-%d", // ISO date
                    // "%Y.%m.%d",
    ];

    // 1. Try parsing as DateTime using various common formats.
    for fmt in dt_formats {
        if let Ok(pdt) = ParsiDateTime::parse(trimmed_input, fmt) {
            return Ok((pdt, true)); // Success as DateTime
        }
    }

    // 2. Try parsing as Date using various common formats.
    for fmt in d_formats {
        if let Ok(pd) = ParsiDate::parse(trimmed_input, fmt) {
            // Convert ParsiDate to ParsiDateTime at 00:00:00.
            // Use new_unchecked as ParsiDate is valid and time 00:00:00 is always valid.
            let pdt =
                unsafe { ParsiDateTime::new_unchecked(pd.year(), pd.month(), pd.day(), 0, 0, 0) };
            return Ok((pdt, false)); // Success as Date
        }
    }

    // 3. If none of the common formats worked, return an error.
    bail!(
        "Could not parse input '{}'. Expected common formats like YYYY/MM/DD, YYYY-MM-DD, YYYY/MM/DD HH:MM:SS, or YYYY-MM-DDTHH:MM:SS.",
        trimmed_input
    )
}

/// Prints the resulting ParsiDateTime, showing only the date part if the original input was just a date.
/// Uses the default `Display` implementation for each type.
pub fn print_result(pdt: ParsiDateTime, was_datetime: bool) {
    if was_datetime {
        println!("{}", pdt); // Print full DateTime (e.g., "1403/05/02 10:30:00")
    } else {
        println!("{}", pdt.date()); // Print only the Date part (e.g., "1403/05/02")
    }
}

/// Maps internal `parsidate::DateError` types to more user-friendly `anyhow::Error`
/// messages suitable for CLI output, providing context about the operation being performed.
pub fn map_parsidate_error(err: DateError, context_msg: &str) -> anyhow::Error {
    let base_message = match err {
        DateError::ParseError(kind) => {
            // Provide specific messages for different parsing failures.
            let kind_msg = match kind {
                ParseErrorKind::FormatMismatch => "input string does not match expected format or has extra characters",
                ParseErrorKind::InvalidNumber => "could not parse number, required digits mismatch, or value out of range",
                ParseErrorKind::InvalidDateValue => "parsed values form a logically invalid date (e.g., day 31 in Mehr, Esfand 30 in non-leap year)",
                ParseErrorKind::InvalidTimeValue => "parsed values form a logically invalid time (e.g., hour 24, minute 60)",
                ParseErrorKind::UnsupportedSpecifier => "format pattern contains specifier unsupported for parsing (e.g., %A, %j)",
                ParseErrorKind::InvalidMonthName => "could not recognize Persian month name in input",
                ParseErrorKind::InvalidWeekdayName => "could not recognize Persian weekday name in input", // Currently unused for parsing
            };
            format!("Parse error: {}", kind_msg)
        }
        DateError::InvalidDate => "Operation resulted in an invalid date".to_string(),
        DateError::InvalidTime => "Operation resulted in an invalid time".to_string(),
        DateError::GregorianConversionError => "Gregorian conversion failed. Input might be outside supported range (e.g., before 622 AD)".to_string(),
        DateError::ArithmeticOverflow => "Date arithmetic resulted in overflow/underflow or went outside supported year range [1, 9999]".to_string(),
        DateError::InvalidOrdinal => "Invalid ordinal day number used".to_string(),
    };
    // Combine the specific error message with the context of the operation.
    anyhow!("Error while {}: {}", context_msg, base_message)
}
