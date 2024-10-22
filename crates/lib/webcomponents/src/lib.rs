pub mod calendar;

pub use calendar::CalendarTemplate;

#[cfg(feature = "assets")]
pub fn get_base_css() -> &'static [u8] {
    include_bytes!("../assets/css/base.css")
}
