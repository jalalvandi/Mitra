//  ~/src/main.rs
//
//  * Copyright (C) Mohammad (Sina) Jalalvandi 2024-2025 <jalalvandi.sina@gmail.com>
//  * Package : mitra
//  * License : Apache-2.0
//  * Version : 2.2.1
//  * URL     : https://github.com/jalalvandi/Mitra
//  * Sign: mitra-20250413-807aa3a3c537-3ea6e1ecf4d95369d274d372595c8d3b
//
//! Main entry point for the mitra-cli application.
//! It parses command-line arguments and dispatches to the appropriate handler function.

// Declare the modules within the src directory
mod cli;
mod events;
mod handlers;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands}; // Import specific items needed

fn main() -> Result<()> {
    // Parse the command-line arguments using the definition from the cli module.
    let cli = Cli::parse();

    // Dispatch execution based on the parsed subcommand.
    // Calls the public handler functions defined in the handlers module.
    // If no subcommand is provided, default to the 'now' command.
    match cli.command {
        Some(Commands::Now) => handlers::handle_now(),
        Some(Commands::Add {
            base_datetime,
            days,
            months,
            years,
            hours,
            minutes,
            seconds,
        }) => handlers::handle_add(base_datetime, days, months, years, hours, minutes, seconds),
        Some(Commands::Sub {
            base_datetime,
            days,
            months,
            years,
            hours,
            minutes,
            seconds,
        }) => handlers::handle_sub(base_datetime, days, months, years, hours, minutes, seconds),
        Some(Commands::Format {
            datetime_string,
            style,
            pattern,
        }) => handlers::handle_format(datetime_string, style, pattern),
        Some(Commands::Diff {
            datetime1,
            datetime2,
        }) => handlers::handle_diff(datetime1, datetime2),
        Some(Commands::Weekday { date_string }) => handlers::handle_weekday(date_string),
        Some(Commands::ToGregorian { parsi_datetime }) => {
            handlers::handle_to_gregorian(parsi_datetime)
        }
        Some(Commands::FromGregorian { gregorian_datetime }) => {
            handlers::handle_from_gregorian(gregorian_datetime)
        }
        Some(Commands::IsLeap { year }) => handlers::handle_is_leap(year),
        Some(Commands::Info { datetime_string }) => handlers::handle_info(datetime_string),
        Some(Commands::Parse {
            input_string,
            pattern,
        }) => handlers::handle_parse(input_string, pattern),
        Some(Commands::Cal { month, year }) => handlers::handle_cal(month, year),
        None => handlers::handle_now(), // Default action if no subcommand is specified
        // Add the new command handler call
        Some(Commands::Events { date_string }) => handlers::handle_events(date_string),
    }
}
