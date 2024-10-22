use askama::Template;

#[derive(Template)]
#[template(path = "layout.html")]
pub struct LayoutTemplate<'a, Component: Template> {
    pub build_id: &'a str,
    pub component: Component,
}
