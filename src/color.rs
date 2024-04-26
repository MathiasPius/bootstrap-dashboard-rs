use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
/// One of the [Bootstrap-defined colors](https://getbootstrap.com/docs/4.6/utilities/colors/).
pub enum Color {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
}

impl Color {
    pub fn as_background(&self) -> &'static str {
        match self {
            Color::Primary => "bg-primary",
            Color::Secondary => "bg-secondary",
            Color::Success => "bg-success",
            Color::Danger => "bg-danger",
            Color::Warning => "bg-warning",
            Color::Info => "bg-info",
            Color::Light => "bg-light",
            Color::Dark => "bg-dark",
        }
    }
}

impl AsRef<str> for Color {
    fn as_ref(&self) -> &str {
        match self {
            Color::Primary => "primary",
            Color::Secondary => "secondary",
            Color::Success => "success",
            Color::Danger => "danger",
            Color::Warning => "warning",
            Color::Info => "info",
            Color::Light => "light",
            Color::Dark => "dark",
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
