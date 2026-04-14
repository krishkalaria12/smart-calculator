#[path = "support/mod.rs"]
mod support;

use support::{Case, ExpectedOutcome, block_on, case, run_cases};

pub fn cases() -> Vec<Case> {
    vec![
        // DATES - Relative
        case("today", Some("date"), "today", ExpectedOutcome::Ok("date")),
        case(
            "tomorrow",
            Some("date"),
            "tomorrow",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "yesterday",
            Some("date"),
            "yesterday",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "day after tomorrow",
            Some("date"),
            "day after tomorrow",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "day before yesterday",
            Some("date"),
            "day before yesterday",
            ExpectedOutcome::Ok("date"),
        ),
        // DATES - Named Days
        case(
            "next monday",
            Some("date"),
            "next monday",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "next tuesday",
            Some("date"),
            "next tuesday",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "next wednesday",
            Some("date"),
            "next wednesday",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "next thursday",
            Some("date"),
            "next thursday",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "next friday",
            Some("date"),
            "next friday",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "next saturday",
            Some("date"),
            "next saturday",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "next sunday",
            Some("date"),
            "next sunday",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "last monday",
            Some("date"),
            "last monday",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "last friday",
            Some("date"),
            "last friday",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "last sunday",
            Some("date"),
            "last sunday",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "this monday",
            Some("date"),
            "this monday",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "this wednesday",
            Some("date"),
            "this wednesday",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "this friday",
            Some("date"),
            "this friday",
            ExpectedOutcome::Ok("date"),
        ),
        // DATES - Offsets
        case(
            "3 days from now",
            Some("date"),
            "3 days from now",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "1 day from now",
            Some("date"),
            "1 day from now",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "7 days from now",
            Some("date"),
            "7 days from now",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "30 days from now",
            Some("date"),
            "30 days from now",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "2 weeks from now",
            Some("date"),
            "2 weeks from now",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "1 week from now",
            Some("date"),
            "1 week from now",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "2 weeks ago",
            Some("date"),
            "2 weeks ago",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "1 week ago",
            Some("date"),
            "1 week ago",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "3 days ago",
            Some("date"),
            "3 days ago",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "1 day ago",
            Some("date"),
            "1 day ago",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "in 5 months",
            Some("date"),
            "in 5 months",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "in 1 month",
            Some("date"),
            "in 1 month",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "in 2 weeks",
            Some("date"),
            "in 2 weeks",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "in 3 days",
            Some("date"),
            "in 3 days",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "in 1 year",
            Some("date"),
            "in 1 year",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "in 6 months",
            Some("date"),
            "in 6 months",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "1 month ago",
            Some("date"),
            "1 month ago",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "6 months ago",
            Some("date"),
            "6 months ago",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "1 year ago",
            Some("date"),
            "1 year ago",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "2 years ago",
            Some("date"),
            "2 years ago",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "5 hours from now",
            Some("date"),
            "5 hours from now",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "10 minutes from now",
            Some("date"),
            "10 minutes from now",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "1 hour ago",
            Some("date"),
            "1 hour ago",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "30 minutes ago",
            Some("date"),
            "30 minutes ago",
            ExpectedOutcome::Ok("date"),
        ),
        // DATES - Unix Timestamps
        case(
            "1741000000",
            Some("date"),
            "unix timestamp 2025",
            ExpectedOutcome::Ok("date"),
        ),
        case("0", Some("date"), "unix epoch", ExpectedOutcome::Ok("date")),
        case(
            "1000000000",
            Some("date"),
            "unix 1 billion",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "1700000000",
            Some("date"),
            "unix 2023 approx",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "1609459200",
            Some("date"),
            "unix 2021-01-01",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "2000000000",
            Some("date"),
            "unix 2033 approx",
            ExpectedOutcome::Ok("date"),
        ),
        // DATES - ISO Parsing
        case(
            "2025-03-03",
            Some("date"),
            "ISO date",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "2025-01-01",
            Some("date"),
            "ISO date new year",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "2025-12-31",
            Some("date"),
            "ISO date new years eve",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "2024-02-29",
            Some("date"),
            "ISO date leap day",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "2000-01-01",
            Some("date"),
            "ISO date Y2K",
            ExpectedOutcome::Ok("date"),
        ),
        // DATES - To Unix / Timestamp
        case(
            "now to unix",
            Some("date"),
            "now to unix timestamp",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "today to timestamp",
            Some("date"),
            "today to timestamp",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "today to unix",
            Some("date"),
            "today to unix",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "tomorrow to unix",
            Some("date"),
            "tomorrow to unix",
            ExpectedOutcome::Ok("date"),
        ),
        // DATES - Date Diff
        case(
            "from 2025-01-01 to 2025-12-31",
            Some("date"),
            "date diff full year",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "from 2025-01-01 to 2025-03-01",
            Some("date"),
            "date diff 2 months",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "from 2020-01-01 to 2025-01-01",
            Some("date"),
            "date diff 5 years",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "from 2025-03-01 to 2025-03-15",
            Some("date"),
            "date diff 2 weeks",
            ExpectedOutcome::Ok("date"),
        ),
        // DATES - Natural Language Variations
        case(
            "whats today",
            Some("date"),
            "natural: whats today",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "what's today's date",
            Some("date"),
            "natural: what's today's date",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "what day is it",
            Some("date"),
            "natural: what day is it",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "what date is tomorrow",
            Some("date"),
            "natural: what date is tomorrow",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "what is the date today",
            Some("date"),
            "natural: what is the date today",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "todays date",
            Some("date"),
            "natural: todays date",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "date today",
            Some("date"),
            "natural: date today",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "current date",
            Some("date"),
            "natural: current date",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "what day is next monday",
            Some("date"),
            "natural: what day is next monday",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "when is next friday",
            Some("date"),
            "natural: when is next friday",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "when is coming friday",
            Some("date"),
            "natural: when is coming friday",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "what day is monday?",
            Some("date"),
            "natural: weekday with punctuation",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "what date is tmr",
            Some("date"),
            "natural: short tomorrow",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "3 days from yesterday",
            Some("date"),
            "relative from yesterday",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "2 weeks later",
            Some("date"),
            "relative with later",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "next weekend",
            Some("date"),
            "relative weekend",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "start of month",
            Some("date"),
            "start of month anchor",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "end of year",
            Some("date"),
            "end of year anchor",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "month after next",
            Some("date"),
            "compound relative month",
            ExpectedOutcome::Ok("date"),
        ),
        case(
            "year before last",
            Some("date"),
            "compound relative year",
            ExpectedOutcome::Ok("date"),
        ),
    ]
}

#[test]
fn imported_super_calculator_date_cases() {
    block_on(run_cases("date", cases()));
}
