use askama::Template;
use chrono::NaiveDate;

use crate::{
    calendar::{DayConfig, DayMarker},
    CalendarTemplate,
};

#[derive(Template)]
#[template(path = "demo.html")]
pub struct DemoTemplate {
    pub calendar: CalendarTemplate,
}

#[allow(deprecated)]
pub fn demo() -> DemoTemplate {
    let calendar = CalendarTemplate::try_new(
        10,
        2024,
        vec![
            DayConfig {
                date: NaiveDate::from_ymd(2024, 10, 1),
                marker: Some(DayMarker::Green),
            },
            DayConfig {
                date: NaiveDate::from_ymd(2024, 10, 7),
                marker: Some(DayMarker::Green),
            },
            DayConfig {
                date: NaiveDate::from_ymd(2024, 10, 8),
                marker: None,
            },
            DayConfig {
                date: NaiveDate::from_ymd(2024, 10, 9),
                marker: Some(DayMarker::Orange),
            },
            DayConfig {
                date: NaiveDate::from_ymd(2024, 10, 10),
                marker: Some(DayMarker::Red),
            },
            DayConfig {
                date: NaiveDate::from_ymd(2024, 10, 13),
                marker: Some(DayMarker::Green),
            },
            DayConfig {
                date: NaiveDate::from_ymd(2024, 10, 14),
                marker: Some(DayMarker::Orange),
            },
            DayConfig {
                date: NaiveDate::from_ymd(2024, 10, 15),
                marker: Some(DayMarker::Green),
            },
            DayConfig {
                date: NaiveDate::from_ymd(2024, 10, 16),
                marker: Some(DayMarker::Red),
            },
            DayConfig {
                date: NaiveDate::from_ymd(2024, 10, 21),
                marker: Some(DayMarker::Green),
            },
            DayConfig {
                date: NaiveDate::from_ymd(2024, 10, 22),
                marker: Some(DayMarker::Orange),
            },
            DayConfig {
                date: NaiveDate::from_ymd(2024, 10, 24),
                marker: Some(DayMarker::Green),
            },
            DayConfig {
                date: NaiveDate::from_ymd(2024, 10, 25),
                marker: Some(DayMarker::Green),
            },
            DayConfig {
                date: NaiveDate::from_ymd(2024, 10, 28),
                marker: Some(DayMarker::Red),
            },
            DayConfig {
                date: NaiveDate::from_ymd(2024, 10, 29),
                marker: Some(DayMarker::Red),
            },
            DayConfig {
                date: NaiveDate::from_ymd(2024, 10, 30),
                marker: Some(DayMarker::Green),
            },
            DayConfig {
                date: NaiveDate::from_ymd(2024, 10, 31),
                marker: Some(DayMarker::Green),
            },
        ],
    )
    .unwrap();
    DemoTemplate { calendar }
}
