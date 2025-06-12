//  ~/mitra-cli/src/main.rs
//
//  This is the CLI front-end for Mitra. It parses arguments and calls
//  the corresponding handler, which in turn uses the `mitra-core` library.

mod cli;
mod handlers;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Default to 'now' command if no subcommand is provided.
    match cli.command.unwrap_or(Commands::Now) {
        Commands::Now => handlers::handle_now(),
        Commands::Add {
            base_datetime,
            days,
            months,
            years,
            hours,
            minutes,
            seconds,
        } => handlers::handle_add(base_datetime, days, months, years, hours, minutes, seconds),
        Commands::Sub {
            base_datetime,
            days,
            months,
            years,
            hours,
            minutes,
            seconds,
        } => handlers::handle_sub(base_datetime, days, months, years, hours, minutes, seconds),
        Commands::Format {
            datetime_string,
            style,
            pattern,
        } => handlers::handle_format(datetime_string, style, pattern),
        Commands::Diff {
            datetime1,
            datetime2,
        } => handlers::handle_diff(datetime1, datetime2),
        Commands::Weekday { date_string } => handlers::handle_weekday(date_string),
        Commands::ToGregorian { parsi_datetime } => handlers::handle_to_gregorian(parsi_datetime),
        Commands::FromGregorian { gregorian_datetime } => {
            handlers::handle_from_gregorian(gregorian_datetime)
        }
        Commands::IsLeap { year } => handlers::handle_is_leap(year),
        Commands::Info { datetime_string } => handlers::handle_info(datetime_string),
        Commands::Parse {
            input_string,
            pattern,
        } => handlers::handle_parse(input_string, pattern),
        Commands::Cal {
            month,
            year,
            three,
            show_year,
        } => handlers::handle_cal(month, year, three, show_year),
        Commands::Events { date_string } => handlers::handle_events(date_string),
    }
}
