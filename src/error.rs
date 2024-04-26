use std::fmt::Display;

#[derive(Template)]
#[template(path = "error.html")]
pub struct Error<T: Display> {
    pub title: Cow<'static, str>,
    pub message: T,
}

impl<T: Display> Error<T> {
    pub fn new(title: &str, message: T) -> Self {
        Self {
            title: title.into(),
            message,
        }
    }
}
