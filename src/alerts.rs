use std::borrow::Cow;

use askama::Template;

use crate::{htmx::Dynamic, icons, Icon};

use super::color::Color;

/// Single Alert entry.
#[derive(Debug, Clone)]
pub struct Alert {
    /// Coloring used for the background of the alert icon.
    pub color: Color,
    /// Font-Awesome Icon used for the alert.
    pub icon: Icon,
    /// Typically the date and time of the alert.
    pub headline: String,
    /// Contents of the alert.
    pub message: String,
    /// If set, will bold the `message` text.
    pub unread: bool,
}

impl Alert {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(headline: S1, message: S2) -> Self {
        Alert {
            headline: headline.as_ref().to_string(),
            message: message.as_ref().to_string(),
            color: Color::Warning,
            icon: icons::fa::EXCLAMATION_CIRCLE,
            unread: false,
        }
    }

    pub fn with_icon(mut self, icon: Icon) -> Self {
        self.icon = icon;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn unread(mut self) -> Self {
        self.unread = true;
        self
    }
}

#[derive(Template)]
#[template(path = "alertlist.html")]
pub struct AlertList(pub Vec<Alert>);

impl AlertList {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// List of Alerts
#[derive(Template)]
#[template(path = "alerts.html")]
pub struct Alerts {
    /// List of [`Alert`]s
    pub alerts: Dynamic<AlertList>,
    /// Optional link to page where alerts can be viewed in full.
    pub show_all_url: Option<Cow<'static, str>>,
}
