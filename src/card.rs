use std::{borrow::Cow, fmt::Display};

use askama::Template;

#[derive(Template)]
#[template(path = "card.html")]
pub struct Card<Content: Display = &'static str> {
    header: Option<Cow<'static, str>>,
    content: Content,
}

impl<Content: Display> Card<Content> {
    pub fn new(content: Content) -> Self {
        Card {
            header: None,
            content,
        }
    }

    pub fn with_header<S: Into<Cow<'static, str>>>(mut self, header: S) -> Self {
        self.header = Some(header.into());
        self
    }
}
