use askama::Template;

#[derive(Template)]
#[template(path = "calendar.html")]
pub struct CalendarTemplate {}
