pub mod calendar;
pub mod demo;

pub use calendar::CalendarTemplate;

#[cfg(feature = "assets")]
pub fn get_base_css() -> &'static [u8] {
    include_bytes!("../assets/css/base.css")
}
