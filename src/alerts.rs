use std::borrow::Cow;

use askama::Template;

use super::color::Color;

/// Single Alert entry.
pub struct Alert {
    /// Coloring used for the background of the alert icon.
    pub color: Color,
    /// Font-Awesome Icon used for the alert.
    pub icon: Cow<'static, str>,
    /// Typically the date and time of the alert.
    pub headline: String,
    /// Contents of the alert.
    pub message: String,
    /// If set, will bold the `message` text.
    pub unread: bool,
}

/// List of Alerts
#[derive(Template)]
#[template(path = "alerts.html")]
pub struct Alerts {
    /// List of [`Alert`]s
    pub alerts: Vec<Alert>,
    /// Optional link to page where alerts can be viewed in full.
    pub show_all_url: Option<Cow<'static, str>>,
}
