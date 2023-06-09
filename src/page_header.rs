use std::borrow::Cow;

use askama::Template;

use crate::IconLink;

#[derive(Template)]
#[template(path = "page_header.html")]
pub struct PageHeader {
    label: Cow<'static, str>,
    links: Vec<IconLink>,
}

impl PageHeader {
    pub fn new<S: Into<Cow<'static, str>>>(label: S) -> Self {
        PageHeader {
            label: label.into(),
            links: Vec::new(),
        }
    }

    pub fn with_link(mut self, link: IconLink) -> Self {
        self.links.push(link);
        self
    }
}

impl<T: Into<Cow<'static, str>>> From<T> for PageHeader {
    fn from(value: T) -> Self {
        PageHeader::new(value)
    }
}
