//! src/events.rs
//! Handles loading and querying calendar event data.

use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Event {
    pub holiday: bool,
    pub month: u32,
    pub day: u32,
    #[serde(rename = "type")] // Rename 'type' field as 'type' is a Rust keyword
    pub event_type: String,
    pub title: String,
}

#[derive(Deserialize, Debug)]
struct CalendarData {
    #[serde(rename = "Persian Calendar")]
    persian_calendar: Vec<Event>,
    // We can ignore "Gregorian Calendar" for now, or add it if needed later
    // #[serde(rename = "Gregorian Calendar")]
    // gregorian_calendar: Vec<Event>,
}

// Type alias for our events map: Key is (month, day), Value is list of events for that day
type EventMap = HashMap<(u32, u32), Vec<Event>>;

// Load and process the event data lazily and store it statically.
// This ensures the JSON is parsed only once during the program's lifetime.
static EVENTS: Lazy<EventMap> = Lazy::new(|| {
    //println!("Loading event data..."); // Debug message, remove later if desired
    let json_data = include_str!("data/events.json"); // Include JSON content at compile time
    match serde_json::from_str::<CalendarData>(json_data) {
        Ok(data) => {
            let mut event_map: EventMap = HashMap::new();
            // Process only Persian Calendar events for now
            for event in data.persian_calendar {
                let key = (event.month, event.day);
                // Get the vector for this day, or create a new one if it doesn't exist
                event_map.entry(key).or_default().push(event);
            }
            /*println!(
                "Event data loaded successfully. {} unique dates with events.",
                event_map.len()
            ); */// Debug
            event_map
        }
        Err(e) => {
            // If JSON parsing fails, print an error and return an empty map.
            // Consider making this a hard error depending on requirements.
            eprintln!(
                "Error parsing event data: {}. Returning empty event map.",
                e
            );
            HashMap::new() // Return empty map on error
        }
    }
});

/// Checks if a given Parsi date has any events associated with it.
/// Returns an Option containing:
/// - Some('*') if there is at least one holiday event.
/// - Some('+') if there are events but none are holidays.
/// - None if there are no events for this date.
pub fn get_event_indicator(month: u32, day: u32) -> Option<char> {
    EVENTS.get(&(month, day)).map(|events_for_day| {
        // Check if any event on this day is a holiday
        if events_for_day.iter().any(|event| event.holiday) {
            '*' // Holiday indicator
        } else {
            '+' // Non-holiday event indicator
        }
    })
}

// Optional: Function to get all event titles for a specific day
// pub fn get_event_titles(month: u32, day: u32) -> Option<Vec<String>> {
//     EVENTS.get(&(month, day)).map(|events_for_day| {
//         events_for_day.iter().map(|event| event.title.clone()).collect()
//     })
// }
