use askama::Template;

use super::color::Color;

pub struct Alert {
    pub color: Color,
    pub icon: &'static str,
    pub headline: String,
    pub message: String,
    pub unread: bool,
}

#[derive(Template)]
#[template(path = "alerts.html")]
pub struct Alerts {
    pub alerts: Vec<Alert>,
    pub show_all_url: Option<&'static str>,
}
