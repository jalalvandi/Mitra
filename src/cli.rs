//  ~/src/cli.rs
//
//  * Copyright (C) 2024–2025 Parsicore <parsicore.dev@gmail.com>
//  * Package : mitra
//  * License : Apache-2.0
//  * Version : 2.3.0
//  * URL     : https://github.com/parsicore/Mitra
//  * Sign: mitra-20250419-bd5fbe728fa2-5836b45f25d83501625cc5529193d5f0
//
//! Defines the command-line interface structure using clap.

use clap::{Parser, Subcommand, ValueEnum};

// Top-level CLI arguments structure
#[derive(Parser, Debug)]
#[command(
    author = "parsicore <parsicore.dev@gmail.com>",
    version = "2.3.0",
    about = "Mitra: A CLI tool for Persian (Jalali/Shamsi) date operations.",
    long_about = "Provides various functionalities for working with ParsiDate dates and datetimes, including conversion, arithmetic, formatting, and information retrieval."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>, // Optional command, defaults to 'now'
}

// Enum defining the available subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Display the current Parsi date and time (default action).
    Now,

    /// Add a duration (days, months, years, hours, minutes, seconds) to a given date/datetime.
    /// Only one duration unit can be specified at a time.
    Add {
        /// Base date (YYYY/MM/DD or YYYY-MM-DD) or datetime (YYYY/MM/DD HH:MM:SS or YYYY-MM-DDTHH:MM:SS).
        base_datetime: String,

        // Duration units - mutually exclusive using clap's `conflicts_with_all`
        #[arg(long, conflicts_with_all = ["months", "years", "hours", "minutes", "seconds"])]
        /// Number of days to add (e.g., 5 or -3).
        days: Option<i64>,

        #[arg(long, conflicts_with_all = ["days", "years", "hours", "minutes", "seconds"])]
        /// Number of months to add (e.g., 2 or -1). Handles day clamping.
        months: Option<i32>,

        #[arg(long, conflicts_with_all = ["days", "months", "hours", "minutes", "seconds"])]
        /// Number of years to add (e.g., 1 or -10). Handles leap day adjustment.
        years: Option<i32>,

        #[arg(long, conflicts_with_all = ["days", "months", "years", "minutes", "seconds"])]
        /// Number of hours to add (e.g., 3 or -1). Uses precise duration arithmetic.
        hours: Option<i64>,

        #[arg(long, conflicts_with_all = ["days", "months", "years", "hours", "seconds"])]
        /// Number of minutes to add (e.g., 30 or -15). Uses precise duration arithmetic.
        minutes: Option<i64>,

        #[arg(long, conflicts_with_all = ["days", "months", "years", "hours", "minutes"])]
        /// Number of seconds to add (e.g., 90 or -45). Uses precise duration arithmetic.
        seconds: Option<i64>,
    },

    /// Subtract a duration (days, months, years, hours, minutes, seconds) from a given date/datetime.
    /// Only one duration unit can be specified at a time.
    Sub {
        /// Base date (YYYY/MM/DD or YYYY-MM-DD) or datetime (YYYY/MM/DD HH:MM:SS or YYYY-MM-DDTHH:MM:SS).
        base_datetime: String,

        // Duration units - mutually exclusive
        #[arg(long, conflicts_with_all = ["months", "years", "hours", "minutes", "seconds"])]
        /// Number of days to subtract (must be non-negative, e.g., 5).
        days: Option<u64>,

        #[arg(long, conflicts_with_all = ["days", "years", "hours", "minutes", "seconds"])]
        /// Number of months to subtract (must be non-negative, e.g., 2). Handles day clamping.
        months: Option<u32>,

        #[arg(long, conflicts_with_all = ["days", "months", "hours", "minutes", "seconds"])]
        /// Number of years to subtract (must be non-negative, e.g., 1). Handles leap day adjustment.
        years: Option<u32>,

        #[arg(long, conflicts_with_all = ["days", "months", "years", "minutes", "seconds"])]
        /// Number of hours to subtract (must be non-negative, e.g., 3). Uses precise duration arithmetic.
        hours: Option<u64>,

        #[arg(long, conflicts_with_all = ["days", "months", "years", "hours", "seconds"])]
        /// Number of minutes to subtract (must be non-negative, e.g., 30). Uses precise duration arithmetic.
        minutes: Option<u64>,

        #[arg(long, conflicts_with_all = ["days", "months", "years", "hours", "minutes"])]
        /// Number of seconds to subtract (must be non-negative, e.g., 90). Uses precise duration arithmetic.
        seconds: Option<u64>,
    },

    /// Format a given date/datetime string using a predefined style or a custom pattern.
    Format {
        /// Date (YYYY/MM/DD or YYYY-MM-DD) or datetime (YYYY/MM/DD HH:MM:SS or YYYY-MM-DDTHH:MM:SS).
        datetime_string: String,

        /// Use a predefined format style. Conflicts with --pattern.
        #[arg(long, value_enum, conflicts_with = "pattern")]
        style: Option<FormatStyle>,

        /// Use a custom format pattern (e.g., "%Y-%m-%d", "%A %d %B ساعت %T").
        /// See mitra docs for specifiers. Conflicts with --style.
        #[arg(short, long)]
        pattern: Option<String>,
    },

    /// Calculate the absolute difference in days between two dates/datetimes.
    Diff {
        /// First date/datetime string.
        datetime1: String,
        /// Second date/datetime string.
        datetime2: String,
    },

    /// Get the Persian weekday name for a given date.
    Weekday {
        /// Date string (YYYY/MM/DD or YYYY-MM-DD). Time part is ignored if present.
        date_string: String,
    },

    /// Convert a Parsi date/datetime to Gregorian.
    ToGregorian {
        /// Parsi date (YYYY/MM/DD or YYYY-MM-DD) or datetime (YYYY/MM/DD HH:MM:SS or YYYY-MM-DDTHH:MM:SS).
        parsi_datetime: String,
    },

    /// Convert a Gregorian date/datetime to Parsi.
    FromGregorian {
        /// Gregorian date (YYYY-MM-DD) or datetime (YYYY-MM-DD HH:MM:SS or YYYY-MM-DDTHH:MM:SS).
        gregorian_datetime: String,
    },

    /// Check if a given Parsi year is a leap year.
    IsLeap {
        /// The Parsi year (e.g., 1403).
        year: i32,
    },

    /// Display detailed information about a Parsi date/datetime.
    Info {
        /// Parsi date (YYYY/MM/DD or YYYY-MM-DD) or datetime (YYYY/MM/DD HH:MM:SS or YYYY-MM-DDTHH:MM:SS).
        datetime_string: String,
    },

    /// Parse a date/datetime string using an explicit format pattern.
    /// The tool attempts to infer Date vs DateTime based on time specifiers in the pattern.
    Parse {
        /// The input string to parse.
        input_string: String,
        /// The explicit format pattern to use for parsing (e.g., "%Y/%m/%d %H:%M").
        #[arg(short, long)]
        pattern: String,
    },

    /// Display a monthly Parsi calendar.
    /// If month and year are not provided, the current month is shown.
    Cal {
        /// Optional month (1-12) to display.
        #[arg(conflicts_with_all = ["three", "show_year"])]
        // Cannot specify month with -3 or -y
        month: Option<u32>,

        /// Optional year (e.g., 1403). Requires month if specified,
        /// unless --show-year is also used.
        #[arg(conflicts_with = "three")]
        // Cannot specify year with -3 (year for -y is separate)
        year: Option<i32>,

        /// Display three months: previous, current, and next.
        /// Defaults to current month/year if month/year are not applicable.
        #[arg(short = '3', long, conflicts_with_all = ["month", "show_year"])]
        three: bool,

        /// Display the calendar for the entire specified year.
        #[arg(short = 'y', long = "year", value_name = "YEAR", conflicts_with_all = ["month", "three"])]
        show_year: Option<i32>, // Takes the year as an argument
    },
    /// List events for a specific Parsi date.
    Events {
        /// The date to check for events (e.g., YYYY/MM/DD, YYYY-MM-DD).
        date_string: String,
    },
}

// Enum for predefined format styles used in the `format` command
#[derive(ValueEnum, Clone, Debug)]
pub enum FormatStyle {
    Short, // YYYY/MM/DD
    Long,  // D Month YYYY (e.g., 2 مرداد 1403)
    Iso,   // YYYY-MM-DD or YYYY-MM-DDTHH:MM:SS
}
