//  ~/src/events.rs
//
//  * Copyright (C) Mohammad (Sina) Jalalvandi 2024-2025 <jalalvandi.sina@gmail.com>
//  * Package : mitra
//  * License : Apache-2.0
//  * Version : 2.3.0
//  * URL     : https://github.com/parsilab/Mitra
//  * Sign: mitra-20250419-bd5fbe728fa2-5836b45f25d83501625cc5529193d5f0
//
//! Handles loading, storing, and querying calendar event data.
//! Reads event information from an embedded JSON file (`src/data/events.json`).
//! Supports fixed Persian calendar events and Hijri events mapped to a specific
//! reference Persian year.

use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap; // Used to potentially get current year if needed, though not currently

// Represents a single calendar event.
#[derive(Deserialize, Debug, Clone)]
pub struct Event {
    /// Indicates if the event is an official public holiday.
    pub holiday: bool,
    /// The Shamsi month (1-12) this event occurs on.
    #[serde(default)] // Default allows flexibility if some event types might omit month/day
    pub month: u32,
    /// The Shamsi day (1-31) this event occurs on.
    #[serde(default)]
    pub day: u32,
    /// The category or type of the event (e.g., "Iran", "Religious", "AncientIran").
    //#[serde(rename = "type", default)] // Rename to avoid Rust keyword conflict
    //pub event_type: String,
    /// The title or description of the event.
    pub title: String,
    /// The original Hijri month (1-12), if this event is a mapped Hijri event.
    #[serde(default)] // Make optional as it only exists for mapped events
    pub hijri_month: Option<u32>,
    /// The original Hijri day (1-30), if this event is a mapped Hijri event.
    #[serde(default)] // Make optional
    pub hijri_day: Option<u32>,
}

// Structure mirroring the top-level JSON data file (`events.json`).
#[derive(Deserialize, Debug)]
struct CalendarData {
    /// The reference Persian (Shamsi) year for which the `hijri_events_mapping` is valid.
    persian_reference_year: i32,

    /// List of fixed Persian events that occur on the same Shamsi month/day every year.
    /// Expected JSON key: "Persian Calendar"
    #[serde(default, rename = "Persian Calendar")]
    persian_events: Vec<Event>,

    /// List of Hijri events mapped to their corresponding Shamsi month/day
    /// specifically for the `persian_reference_year`.
    /// Expected JSON key: "hijri_events_mapping"
    #[serde(default, rename = "hijri_events_mapping")]
    hijri_events_mapping: Vec<Event>,
}

// Type alias for storing events, mapping (Month, Day) tuples to a list of events.
type EventMap = HashMap<(u32, u32), Vec<Event>>;

// Holds the loaded and processed event data.
struct LoadedEvents {
    /// The reference Shamsi year for the mapped Hijri events.
    reference_year: i32,
    /// Map storing fixed Persian events [(Month, Day) -> Vec<Event>].
    fixed_persian_events: EventMap,
    /// Map storing Hijri events mapped to Shamsi dates for the reference year [(Month, Day) -> Vec<Event>].
    mapped_hijri_events: EventMap,
}

// Lazily load and process the event data from the embedded JSON file.
// Ensures the JSON is parsed only once during the application's lifetime.
static LOADED_DATA: Lazy<LoadedEvents> = Lazy::new(|| {
    // Embed the JSON file content directly into the binary at compile time.
    let json_data = include_str!("data/events.json");

    // Attempt to parse the JSON data into our CalendarData struct.
    match serde_json::from_str::<CalendarData>(json_data) {
        Ok(data) => {
            // Process fixed Persian events into their own map.
            let mut fixed_persian_events: EventMap = HashMap::new();
            for event in data.persian_events {
                // Defensively ensure no Hijri info is accidentally associated
                let mut clean_event = event;
                clean_event.hijri_month = None;
                clean_event.hijri_day = None;
                // Add the event to the map, keyed by (month, day).
                fixed_persian_events
                    .entry((clean_event.month, clean_event.day))
                    .or_default() // Get vec or create if absent
                    .push(clean_event); // Add event to the vec
            }

            // Process mapped Hijri events into their own map.
            let mut mapped_hijri_events: EventMap = HashMap::new();
            for event in data.hijri_events_mapping {
                // These events should inherently have Shamsi month/day from the mapping.
                // Keep hijri_month/day info if present in JSON.
                mapped_hijri_events
                    .entry((event.month, event.day))
                    .or_default()
                    .push(event);
            }

            // Return the processed data wrapped in LoadedEvents.
            LoadedEvents {
                reference_year: data.persian_reference_year,
                fixed_persian_events,
                mapped_hijri_events,
            }
        }
        Err(e) => {
            // If JSON parsing fails, log a critical error and return an empty structure.
            // Event functionality will be effectively disabled.
            eprintln!(
                "CRITICAL: Error parsing event data from 'events.json': {}",
                e
            );
            eprintln!("Event listing and calendar indicators will be unavailable.");
            LoadedEvents {
                reference_year: 0, // Using 0 to indicate an error state
                fixed_persian_events: HashMap::new(),
                mapped_hijri_events: HashMap::new(),
            }
        }
    }
});

/// Returns a combined list of relevant `Event`s for the given Shamsi year, month, and day.
///
/// This function always includes fixed Persian events (those occurring on the same
/// Shamsi date each year). It *only* includes mapped Hijri events if the `query_year`
/// matches the `reference_year` defined in the loaded event data.
///
/// Returns `None` if no relevant events are found for the given date and year context,
/// or if the event data failed to load initially.
pub fn get_events_for_date(
    query_year: i32,
    query_month: u32,
    query_day: u32,
) -> Option<Vec<Event>> {
    // Access the globally loaded (and potentially processed) event data.
    let loaded_data = &*LOADED_DATA;

    // If the reference year is 0, it indicates the data failed to load.
    if loaded_data.reference_year == 0 {
        return None;
    }

    // The key used to look up events in our maps.
    let key = (query_month, query_day);
    // Initialize an empty vector to store the combined results.
    let mut results: Vec<Event> = Vec::new();

    // 1. Add fixed Persian events: These apply regardless of the year.
    if let Some(fixed_events) = loaded_data.fixed_persian_events.get(&key) {
        // Extend the results with clones of the fixed events.
        results.extend(fixed_events.iter().cloned());
    }

    // 2. Conditionally add mapped Hijri events: Only if the queried year
    //    matches the year for which the mapping is valid.
    if query_year == loaded_data.reference_year {
        if let Some(mapped_events) = loaded_data.mapped_hijri_events.get(&key) {
            // Extend the results with clones of the mapped events.
            results.extend(mapped_events.iter().cloned());
        }
    }

    // Return the combined list if it's not empty, otherwise return None.
    if results.is_empty() {
        None
    } else {
        // Optional: Sort the results, e.g., holidays first.
        // results.sort_by_key(|e| !e.holiday); // Sorts so holidays (true) come first
        Some(results)
    }
}

/// Determines an indicator character for calendar display based on events for a specific date.
///
/// Considers both fixed Persian events and mapped Hijri events (only if the `query_year`
/// matches the reference year).
///
/// Returns:
/// - `Some('*')`: If there is at least one holiday event for the date in the relevant year context.
/// - `Some('+')`: If there are events but none are holidays for the date in the relevant year context.
/// - `None`: If there are no relevant events for the date and year context, or if data loading failed.
pub fn get_event_indicator(query_year: i32, query_month: u32, query_day: u32) -> Option<char> {
    // Get the relevant events for the specific year context first.
    get_events_for_date(query_year, query_month, query_day).map(|events_for_day| {
        // If Some(events_for_day) is returned...
        // Check if any event in the list is marked as a holiday.
        if events_for_day.iter().any(|event| event.holiday) {
            '*' // It's a holiday.
        } else {
            '+' // There are events, but none are holidays.
        }
        // If get_events_for_date returned None, this .map() is skipped, returning None.
    })
}
