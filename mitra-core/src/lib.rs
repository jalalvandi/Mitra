//  ~/mitra-core/src/lib.rs
//
//  This is the main library crate for Mitra.
//  It exposes all the core logic, types, and utilities for use by other crates
//  like `mitra-cli` and `mitra-gui`.

// Make modules accessible within the crate
mod logic;

// Make these modules public so other crates can use them
pub mod events;
pub mod utils;

// Publicly export key functions and types for easy access
pub use logic::{
    add_to_datetime, convert_from_gregorian, convert_to_gregorian, get_current_parsi_datetime,
    get_date_info, get_days_diff, get_weekday, is_leap_year, sub_from_datetime, DateInfo,
};

// Also re-export important types from dependencies
pub use anyhow::{Error, Result};
pub use parsidate::{self, ParsiDate, ParsiDateTime};
