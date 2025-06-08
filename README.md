# Mitra - Terminal Based Persian (Jalali) Date Tool

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg?style=flat-square)](./LICENSE)
[![CI](https://github.com/jalalvandi/mitra/actions/workflows/ci.yml/badge.svg)](https://github.com/jalalvandi/mitra/actions/workflows/ci.yml)
![Maintenance](https://img.shields.io/badge/maintained-actively-green)

A versatile command-line interface (CLI) tool for working with Persian (Jalali/Shamsi) dates and times, built upon the robust [`parsidate`](https://crates.io/crates/parsidate) Rust library.

Mitra allows you to perform common date/time operations directly from your terminal, including conversions, arithmetic, formatting, and retrieving detailed information.

## Features

*   **Current Time:** Display the current Parsi date and time.
*   **Date/Time Arithmetic:** Add or subtract years, months, days, hours, minutes, or seconds from a given Parsi date/time. Handles day clamping and leap year adjustments correctly.
*   **Month View:** Displays a monthly Parsi calendar, similar to `ncal`.
*   **Event Listing:** List holidays and occasions for a specific Parsi date (`events`).
*   **Formatting:** Format Parsi dates and times using predefined styles (`short`, `long`, `iso`) or custom `strftime`-like patterns.
*   **Conversion:**
    *   Convert Parsi dates/times to Gregorian.
    *   Convert Gregorian dates/times to Parsi.
*   **Information:** Get detailed information about a Parsi date (weekday, day of year, leap year status, Gregorian equivalent, etc.).
*   **Validation:** Check if a Parsi year is a leap year.
*   **Difference:** Calculate the difference in days between two Parsi dates.
*   **Parsing:** Parse date/time strings using explicit format patterns.
*   **Powered by `parsidate`:** Leverages the core logic and accuracy of the `parsidate` crate for all Persian calendar calculations.

## Installation

### Prerequisites

*   Ensure you have Rust and Cargo installed. You can get them from [rustup.rs](https://rustup.rs/).

### Build from Source

1.  Clone the repository:
    ```bash
    git clone https://github.com/jalalvandi/mitra.git
    cd mitra
    ```
2.  Build the release executable:
    ```bash
    cargo build --release
    ```
3.  The executable will be located at `./target/release/mitra`. You can copy this file to a directory in your system's `PATH` (e.g., `~/.cargo/bin`, `/usr/local/bin`) for easier access.

## General Usage

The basic command structure is:

```bash
mitra <COMMAND> [ARGUMENTS/OPTIONS]
```

*   If no `COMMAND` is provided, it defaults to `mitra now`.
*   Use `mitra --help` to see the list of all commands.
*   Use `mitra <COMMAND> --help` for help on a specific command.

### Accepted Date/Time Formats

Commands that accept date or datetime strings (`add`, `sub`, `format`, `diff`, `info`, `to-gregorian`) attempt to parse the following common Parsi formats automatically:

*   **Date:** `YYYY/MM/DD`, `YYYY-MM-DD`
*   **DateTime:** `YYYY/MM/DD HH:MM:SS`, `YYYY-MM-DDTHH:MM:SS`, `YYYY-MM-DD HH:MM:SS`

The `from-gregorian` command accepts similar Gregorian formats.
The `parse` command requires an explicit format pattern.

## Commands and Examples

Here's a breakdown of the available commands:

---

### `now`

Displays the current Parsi date and time based on your system's local clock.

**Usage:**

```bash
mitra
# or
mitra now
```

**Example Output:**

```
1403/05/06 10:35:15
```

---

### `add`

Adds a specified duration unit to a base Parsi date or datetime. Only one duration unit (`--days`, `--months`, etc.) can be used per command.

**Usage:**

```bash
mitra add <BASE_DATETIME> [DURATION_OPTION]
```

**Examples:**

```bash
# Add 15 days to a date
mitra add "1403/06/20" --days 15
# Output: 1403/07/05

# Add 2 months to a date (handles day clamping)
mitra add "1403/06/31" --months 2
# Output: 1403/08/30 (Shahrivar 31 -> Aban 30)

# Add 1 year to a leap day (handles leap adjustment)
mitra add "1403/12/30" --years 1
# Output: 1404/12/29 (1404 is not leap)

# Add 3 hours to a datetime
mitra add "1403/01/01 22:00:00" --hours 3
# Output: 1403/01/02 01:00:00 (Crosses midnight)

# Add 90 seconds to a datetime
mitra add "1403/05/06 10:30:00" --seconds 90
# Output: 1403/05/06 10:31:30

# Add -5 days (subtract 5 days)
mitra add "1403/02/03" --days -5
# Output: 1403/01/29
```

---

### `sub`

Subtracts a specified duration unit from a base Parsi date or datetime. Input values for duration must be non-negative. Only one duration unit can be used per command.

**Usage:**

```bash
mitra sub <BASE_DATETIME> [DURATION_OPTION]
```

**Examples:**

```bash
# Subtract 10 days from a date
mitra sub "1403/03/05" --days 10
# Output: 1403/02/26

# Subtract 1 month from a date (handles day clamping)
mitra sub "1403/01/31" --months 1
# Output: 1402/12/29 (Esfand 29 in common year 1402)

# Subtract 4 years from a leap day
mitra sub "1403/12/30" --years 4
# Output: 1399/12/30 (1399 is leap)

# Subtract 2 hours from a datetime
mitra sub "1403/07/10 01:30:00" --hours 2
# Output: 1403/07/09 23:30:00 (Crosses midnight backwards)

# Subtract 75 minutes from a datetime
mitra sub "1403/05/06 11:00:00" --minutes 75
# Output: 1403/05/06 09:45:00
```

---

### `format`

Formats a given Parsi date/datetime string according to a specified style or custom pattern.

**Usage:**

```bash
mitra format <DATETIME_STRING> (--style <STYLE> | --pattern <PATTERN>)
```

**Styles (`--style`):**

*   `short`: `YYYY/MM/DD` (or `YYYY/MM/DD HH:MM:SS` if input has time)
*   `long`: `D Month YYYY` (e.g., `6 مرداد 1403`, time is ignored)
*   `iso`: `YYYY-MM-DD` (or `YYYY-MM-DDTHH:MM:SS` if input has time)

**Pattern (`--pattern`):** Uses `strftime`-like specifiers (see `parsidate` docs or examples below).

**Examples:**

```bash
# Format using short style (default for date)
mitra format "1403/05/06" --style short
# Output: 1403/05/06

# Format using long style
mitra format "1403-05-06" --style long
# Output: 6 مرداد 1403

# Format datetime using iso style
mitra format "1403/05/06 11:00:30" --style iso
# Output: 1403-05-06T11:00:30

# Format using a custom pattern
mitra format "1403/05/06 11:00:30" --pattern "%A %d %B, %Y - %H:%M"
# Output: یکشنبه 06 مرداد, 1403 - 11:00

# Another custom pattern
mitra format "1403/01/01" --pattern "Day %j of %Y (%A)"
# Output: Day 001 of 1403 (چهارشنبه)
```

---

### `diff`

Calculates the absolute difference in days between two Parsi dates/datetimes (ignores time part for calculation).

**Usage:**

```bash
mitra diff <DATETIME1> <DATETIME2>
```

**Examples:**

```bash
mitra diff "1403/01/01" "1404/01/01"
# Output: Difference: 366 days (1403 is leap)

mitra diff "1404/01/01" "1405/01/01"
# Output: Difference: 365 days

mitra diff "1403/05/10" "1403/05/01"
# Output: Difference: 9 days
```

---

### `weekday`

Gets the full Persian weekday name for a given Parsi date.

**Usage:**

```bash
mitra weekday <DATE_STRING>
```

**Examples:**

```bash
mitra weekday "1403/05/06"
# Output: یکشنبه

mitra weekday "1403-01-01"
# Output: چهارشنبه

mitra weekday "1357/11/22"
# Output: یکشنبه
```

---

### `to-gregorian`

Converts a Parsi date or datetime to its Gregorian equivalent.

**Usage:**

```bash
mitra to-gregorian <PARSI_DATETIME>
```

**Examples:**

```bash
mitra to-gregorian "1403/05/06"
# Output: 2024-07-27

mitra to-gregorian "1403/01/01 00:00:00"
# Output: 2024-03-20 00:00:00

mitra to-gregorian "1399-12-30T12:00:00" # End of leap year
# Output: 2021-03-20 12:00:00
```

---

### `from-gregorian`

Converts a Gregorian date or datetime to its Parsi equivalent.

**Usage:**

```bash
mitra from-gregorian <GREGORIAN_DATETIME>
```

**Examples:**

```bash
mitra from-gregorian "2024-07-27"
# Output: 1403/05/06

mitra from-gregorian "2024-03-20 10:30:00" # Nowruz 1403
# Output: 1403/01/01 10:30:00

mitra from-gregorian "1979-02-11"
# Output: 1357/11/22
```

---

### `is-leap`

Checks if a given Parsi year is a leap year according to the common 33-year cycle approximation used by `parsidate`.

**Usage:**

```bash
mitra is-leap <YEAR>
```

**Examples:**

```bash
mitra is-leap 1403
# Output: Yes

mitra is-leap 1404
# Output: No

mitra is-leap 1399
# Output: Yes
```

---

### `info`

Displays detailed information about a given Parsi date or datetime.

**Usage:**

```bash
mitra info <DATETIME_STRING>
```

**Example:**

```bash
mitra info "1403/12/30 23:59:55"
```

**Example Output:**

```
Input Parsi Date/Time: 1403/12/30 23:59:55
-------------------------
 Parsed Date: 1403/12/30
 Parsed Time: 23:59:55
 Weekday: پنجشنبه
 Day of Year: 366
 Days in Current Month: 30
 Is Leap Year: Yes
 Gregorian Equivalent: 2025-03-20 23:59:55
 First Day of Month: 1403/12/01
 Last Day of Month: 1403/12/30
 First Day of Year: 1403/01/01
 Last Day of Year: 1403/12/30
```

---

### `parse`

Parses an input string using an explicit `strftime`-like pattern. It attempts to infer whether a Date or DateTime is expected based on the pattern.

**Usage:**

```bash
mitra parse <INPUT_STRING> --pattern <PATTERN>
```

**Examples:**

```bash
# Parse a date
mitra parse "1403 خرداد 05" --pattern "%Y %B %d"
# Output: Parsed Date: 1403/03/05

# Parse a datetime
mitra parse "Time:22:15:00 Date:1403/10/11" --pattern "Time:%T Date:%Y/%m/%d"
# Output: Parsed DateTime: 1403/10/11 22:15:00

# Parse failure example
mitra parse "1403-5-6" --pattern "%Y-%m-%d"
# Output: Error: Parse error while parsing date with explicit format: could not parse number, required digits mismatch, or value out of range
```

---
### `cal`

Displays a monthly Parsi calendar, similar to `ncal`, with options for specifying the month and year. If omitted, the current month is shown. Includes indicators for days with events: `*` for holidays, `+` for other occasions.

**Usage:**

```bash
mitra cal [MONTH] [YEAR]
```

**Examples:**

```bash
# Display calendar for the current month
mitra cal

# Display calendar for Farvardin 1403
mitra cal 1 1403

# Display calendar for Esfand 1399 (leap year)
mitra cal 12 1399
```

```bash
mitra cal [OPTIONS]
```

**Examples:**

```bash
#Display calendar for 3 month
mitra cal --three
#or
mitra cal -3

#Display calendar for 1404 year
mitra cal -y 1404
```

### `events`

Lists the holidays and other occasions recorded for a specific Parsi date. Data is based on the included events.json file.

**Usage:**

```bash
mitra events <DATE_STRING>
```

**Examples:**

```bash
# List events for Nowruz (Farvardin 1st)
mitra events 1403/01/01
# Example Output:
# Events for 01 Farvardin:
#   [تعطیل] آغاز نوروز
#   - جشن نوروز، نوروز جمشیدی (جمشید پیشدادی) - ابتدای بهار


# List events for a day with a non-holiday occasion
mitra events 1403/02/25
# Example Output:
# Events for 25 Ordibehesht:
#   - روز پاسداشت زبان فارسی و بزرگداشت حکیم ابوالقاسم فردوسی
#   - بزرگداشت استاد توس فردوسی بزرگ

# Query a day with no specific events
mitra events 1403/07/10
# Example Output:
# Events for 10 Mehr:
#   - No events found.
```

## Dependencies

*   **[`parsidate`](https://crates.io/crates/parsidate):** The core Rust library providing Persian date logic.
*   **[`clap`](https://crates.io/crates/clap):** For command-line argument parsing.
*   **[`anyhow`](https://crates.io/crates/anyhow):** For flexible error handling.
*   **[`chrono`](https://crates.io/crates/chrono):** Used internally by `parsidate` and for `Duration` handling.
*   **[`serde`](https://crates.io/crates/serde) & [`serde_json`](https://crates.io/crates/serde_json):** For deserializing the event data from the embedded JSON file.
*   **[`once_cell`](https://crates.io/crates/once_cell):** For lazy, static initialization of the event data, ensuring it's loaded only once.

## Contributing

Contributions (bug reports, feature requests, pull requests) are welcome! Please check the issue tracker on the repository.

## License

This project is licensed under either of

*   Apache License, Version 2.0, ([LICENSE](./LICENSE)).
```
Version: 2.3.0
Sign: mitra-20250419-bd5fbe728fa2-5836b45f25d83501625cc5529193d5f0
```
