//! Civil (proleptic Gregorian) date/time value types plus the minimal calendar math the date
//! widgets need. Pure and dependency-free (see ADR-0011): no time zones, no parsing, no instants —
//! just enough to lay out a month grid and step hours/minutes.

/// Number of months in a year, used to wrap month navigation.
const MONTHS_PER_YEAR: u32 = 12;
/// Full English month names, indexed by `month - 1`.
const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// A civil date in the proleptic Gregorian calendar. `month` is `1..=12`, `day` is
/// `1..=days_in_month`. Construct via [`Date::new`], which clamps both into range.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl Default for Date {
    fn default() -> Self {
        Self {
            year: 2000,
            month: 1,
            day: 1,
        }
    }
}

impl Date {
    /// Create a date, clamping `month` to `1..=12` and `day` to that month's valid range.
    #[must_use]
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        let month = month.clamp(1, MONTHS_PER_YEAR);
        let day = day.clamp(1, days_in_month(year, month));
        Self { year, month, day }
    }

    /// Same date with `day` replaced (clamped into the current month's range).
    #[must_use]
    pub fn with_day(self, day: u32) -> Self {
        Self::new(self.year, self.month, day)
    }

    /// The previous month, keeping the day clamped into the new month's length (so e.g.
    /// 31 March → 28/29 February).
    #[must_use]
    pub fn previous_month(self) -> Self {
        let (year, month) = if self.month == 1 {
            (self.year - 1, MONTHS_PER_YEAR)
        } else {
            (self.year, self.month - 1)
        };
        Self::new(year, month, self.day)
    }

    /// The next month, keeping the day clamped into the new month's length.
    #[must_use]
    pub fn next_month(self) -> Self {
        let (year, month) = if self.month == MONTHS_PER_YEAR {
            (self.year + 1, 1)
        } else {
            (self.year, self.month + 1)
        };
        Self::new(year, month, self.day)
    }
}

/// A wall-clock time of day. `hour` is `0..=23`, `minute` is `0..=59`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Time {
    pub hour: u32,
    pub minute: u32,
}

impl Time {
    /// Create a time, clamping `hour` to `0..=23` and `minute` to `0..=59`.
    #[must_use]
    pub fn new(hour: u32, minute: u32) -> Self {
        Self {
            hour: hour.min(23),
            minute: minute.min(59),
        }
    }
}

/// Whether `year` is a leap year in the Gregorian calendar.
#[must_use]
pub const fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// Number of days in `month` (`1..=12`) of `year`; `0` for an out-of-range month.
#[must_use]
pub const fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

/// Day of the week of `date`, `0 = Monday … 6 = Sunday` (Sakamoto's algorithm).
#[must_use]
pub fn day_of_week(date: Date) -> u32 {
    // Sakamoto: month offsets, treating Jan/Feb as months of the previous year.
    const T: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = if date.month < 3 {
        date.year - 1
    } else {
        date.year
    };
    let m = (date.month - 1) as usize;
    // 0 = Sunday in Sakamoto's result.
    let sunday0 = (y + y / 4 - y / 100 + y / 400 + T[m] + date.day as i32).rem_euclid(7);
    // Shift so Monday = 0 .. Sunday = 6.
    ((sunday0 + 6) % 7) as u32
}

/// Full English name of `month` (`1..=12`); empty for an out-of-range month.
#[must_use]
pub fn month_name(month: u32) -> &'static str {
    MONTH_NAMES
        .get((month.max(1) - 1) as usize)
        .copied()
        .unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leap_years_follow_gregorian_rule() {
        assert!(is_leap_year(2000)); // divisible by 400
        assert!(!is_leap_year(1900)); // divisible by 100, not 400
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2023));
    }

    #[test]
    fn february_length_tracks_leap_years() {
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2023, 2), 28);
        assert_eq!(days_in_month(2023, 4), 30);
        assert_eq!(days_in_month(2023, 12), 31);
    }

    #[test]
    fn day_of_week_matches_known_dates() {
        // 2000-01-01 was a Saturday; 2026-06-17 is a Wednesday.
        assert_eq!(day_of_week(Date::new(2000, 1, 1)), 5); // Saturday (Mon=0)
        assert_eq!(day_of_week(Date::new(2026, 6, 17)), 2); // Wednesday
    }

    #[test]
    fn new_clamps_into_range() {
        assert_eq!(Date::new(2023, 2, 31), Date::new(2023, 2, 28));
        assert_eq!(Date::new(2023, 13, 1).month, 12);
        assert_eq!(Time::new(99, 99), Time::new(23, 59));
    }

    #[test]
    fn month_navigation_wraps_year_and_clamps_day() {
        assert_eq!(
            Date::new(2023, 1, 15).previous_month(),
            Date::new(2022, 12, 15)
        );
        assert_eq!(Date::new(2023, 12, 15).next_month(), Date::new(2024, 1, 15));
        // 31 March → February clamps the day.
        assert_eq!(
            Date::new(2024, 3, 31).previous_month(),
            Date::new(2024, 2, 29)
        );
    }
}
