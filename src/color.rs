
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
