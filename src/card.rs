use std::{borrow::Cow, fmt::Display};

use askama::Template;

use crate::{LinkAction, PlainLink};

#[derive(Template)]
#[template(path = "card.html")]
pub struct Card<Content: Display = &'static str> {
    header: Option<Cow<'static, str>>,
    context_links: Vec<ContextGroup>,
    content: Content,
}

/// Context menu [`ContextGroup`]s have optional labels, and are always
/// separated by a divider.
pub struct ContextGroup {
    /// Optional Group label.
    pub label: Option<Cow<'static, str>>,
    /// Group's navigation items.
    pub items: Vec<PlainLink>,
}

impl ContextGroup {
    pub fn new() -> Self {
        ContextGroup {
            label: None,
            items: Vec::new(),
        }
    }

    pub fn with_label<S: Into<Cow<'static, str>>>(mut self, label: S) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_link<S: Into<Cow<'static, str>>>(mut self, label: S, action: LinkAction) -> Self {
        self.items.push(PlainLink::new(label, action));
        self
    }
}

impl Default for ContextGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl<Content: Display> Card<Content> {
    pub fn new(content: Content) -> Self {
        Card {
            header: None,
            context_links: Vec::new(),
            content,
        }
    }

    pub fn with_header<S: Into<Cow<'static, str>>>(mut self, header: S) -> Self {
        self.header = Some(header.into());
        self
    }

    pub fn with_context_group(mut self, group: ContextGroup) -> Self {
        self.context_links.push(group);
        self
    }
}
