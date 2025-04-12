# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Note on Versioning

This changelog only documents major and minor version updates. Patch versions and minor fixes are not included here to maintain focus on significant changes and new features. For detailed patch-level changes, please refer to the commit history.

## [2.2.0] - 2025-04-12

### Added

* support for Hijri events mapping in events.json, including a new key for "hijri_events_mapping".

### Changed

* Enhanced event structure to include optional Hijri month and day fields.
* Refactored event loading logic to separate fixed Persian events and mapped Hijri events.
* Improved error handling for JSON parsing in event loading.
* Updated event indicator and retrieval functions to accommodate new event structure.
* Adjusted command handlers to utilize updated event retrieval functions.

## [2.1.0] - 2025-04-07

### Added

*   **New Command `cal`:** Display a monthly Parsi calendar similar to `ncal`, with options for month/year and indicators for days with events (`*` for holidays, `+` for others).
*   **Event Data Handling:** Integrated JSON data (`src/data/events.json`) containing Persian calendar occasions. Data is loaded statically using `once_cell`. Added `events.rs` module to manage loading and querying.
*   **New Command `events`:** Added `mitra events <DATE>` command to list holidays and occasions for the specified Parsi date. It retrieves data loaded from `events.json` and indicates official holidays.

## [1.1.0] - 2025-04-07

### Added

*   **New Commands:**
    *   `from-gregorian`: Convert Gregorian dates/times to Parsi.
    *   `is-leap`: Check if a given Parsi year is a leap year.
    *   `info`: Display detailed information about a Parsi date/time (weekday, ordinal day, Gregorian equivalent, boundaries, etc.).
    *   `diff`: Calculate the difference in days between two Parsi dates.
    *   `weekday`: Get the Persian weekday name for a given date.
    *   `parse`: Parse a date/time string using an explicit format pattern.
*   **Enhanced `add` / `sub` Commands:**
    *   Added support for adding/subtracting `--hours`, `--minutes`, and `--seconds` using precise duration arithmetic via `chrono::Duration`.
*   **Enhanced `format` Command:**
    *   Added `--style` option with predefined formats: `short`, `long`, `iso`. Conflicts with `--pattern`.
*   **Project Structure:**
    *   Refactored codebase into multiple modules (`cli.rs`, `handlers.rs`, `utils.rs`) for better organization and maintainability.
*   **CI/CD:**
    *   Added GitHub Actions workflow (`ci.yml`) for code checking, linting, building, and testing across Linux, macOS, and Windows.
    *   Added GitHub Actions workflow (`release.yml`) to automatically build binaries for Linux (musl), macOS (x86_64, aarch64), and Windows (x86_64) upon pushing a version tag (e.g., `v0.3.0`), and attach them to a GitHub Release. Includes basic release note generation placeholder (can be improved).
*   **Documentation:**
    *   Created a comprehensive `README.md` with features, installation instructions, and detailed usage examples for all commands.
    *   Added English code comments and documentation strings throughout the codebase.

### Changed

*   Improved error messages using `anyhow` and context mapping in `utils::map_parsidate_error`.
*   Enhanced input parsing (`utils::parse_input_datetime_or_date`) to handle more common date/time formats automatically (including ISO 8601-like dates and T-separated datetimes).
*   Standardized output printing using `utils::print_result` to show only date for date-only inputs.
*   Updated `Cargo.toml` with description, license, and bumped version.

### Fixed

*   Ensured mutually exclusive duration arguments in `add` and `sub` commands using `clap` attributes and added explicit checks.
