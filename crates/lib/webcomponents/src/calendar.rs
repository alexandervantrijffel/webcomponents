use askama::Template;
use chrono::{Datelike, Duration, NaiveDate};

#[derive(Template)]
#[template(path = "calendar.html")]
pub struct CalendarTemplate {
    day_names: [&'static str; 7],
    days: [CalendarDay; 42],
    month_title: String,
}

struct CalendarDay {
    date: NaiveDate,
    corner_classes: &'static str,
    color_classes: &'static str,
}

impl CalendarTemplate {
    pub fn try_new(month: u32, year: i32) -> Option<Self> {
        let first_day = first_starting_sunday(year, month)?;
        Some(Self {
            day_names: ["S", "M", "T", "W", "T", "F", "S"],
            days: std::array::from_fn(|days| {
                let date = first_day + Duration::days(days as i64);
                CalendarDay {
                    color_classes: if date.month() == month {
                        "bg-boxdark text-white"
                    } else {
                        "bg-meta-4 text-gray-400"
                    },
                    date,
                    corner_classes: match days {
                        0 => "rounded-tl-lg rounded-tr-none rounded-b-none focus:z-10",
                        6 => "rounded-tr-lg rounded-tl-none rounded-b-none focus:z-10",
                        35 => "rounded-bl-lg rounded-t-none rounded-bl-none focus:z-10",
                        41 => "rounded-br-lg rounded-t-none rounded-bl-none focus:z-10",
                        _ => "!rounded-none",
                    },
                }
            }),
            month_title: NaiveDate::from_ymd_opt(year, month, 1)?
                .format("%B %Y")
                .to_string(),
        })
    }
}

fn first_starting_sunday(year: i32, month: u32) -> Option<NaiveDate> {
    let first_day_of_month = NaiveDate::from_ymd_opt(year as i32, month as u32, 1)?;
    let weekday = first_day_of_month.weekday().num_days_from_sunday();
    Some(first_day_of_month - Duration::days(weekday as i64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starting_sunday_is_in_previous_month() {
        assert_eq!(
            first_starting_sunday(2024, 10),
            NaiveDate::from_ymd_opt(2024, 9, 29)
        );
    }
    #[test]
    fn test_starting_sunday_is_first_of_the_month() {
        assert_eq!(
            first_starting_sunday(2024, 9),
            NaiveDate::from_ymd_opt(2024, 9, 1)
        );
    }
}
