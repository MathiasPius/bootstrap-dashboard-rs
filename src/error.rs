use std::{borrow::Cow, fmt::Display};

use askama::Template;

#[derive(Template)]
#[template(path = "error.html")]
pub struct Error<T: Display> {
    pub title: Cow<'static, str>,
    pub message: T,
}

impl<T: Display> Error<T> {
    pub fn new<H: Into<Cow<'static, str>>>(title: H, message: T) -> Self {
        Self {
            title: title.into(),
            message,
        }
    }
}
