use std::{borrow::Cow, collections::HashSet, fmt::Display};

use askama::Template;

use crate::grid::{Column, ColumnSize, Breakpoint, ColumnWidth};

#[derive(Template)]
#[template(path = "card.html")]
pub struct Card<Content: Display = &'static str> {
    widths: HashSet<ColumnSize>,
    header: Option<Cow<'static, str>>,
    content: Content,
}

impl<Content: Display> Card<Content> {
    pub fn new(content: Content) -> Self {
        Card {
            widths: HashSet::new(),
            header: None,
            content,
        }
    }

    pub fn with_header<S: Into<Cow<'static, str>>>(mut self, header: S) -> Self {
        self.header = Some(header.into());
        self
    }

    pub fn with_size<Width: Into<ColumnWidth>>(mut self, breakpoint: Breakpoint, width: Width) -> Self {
        self.widths.insert(ColumnSize::new(breakpoint, width.into()));
        self
    }
}

impl<Content: Display> Column for Card<Content> {
    fn columns(&self) -> &HashSet<ColumnSize> {
        &self.widths
    }
}
