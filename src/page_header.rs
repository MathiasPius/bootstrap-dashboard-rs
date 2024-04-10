use std::borrow::Cow;

use askama::Template;

#[derive(Template)]
#[template(path = "page_header.html")]
pub struct PageHeader {
    label: Cow<'static, str>,
}

impl PageHeader {
    pub fn new<S: Into<Cow<'static, str>>>(label: S) -> Self {
        PageHeader {
            label: label.into(),
        }
    }
}

impl<T: Into<Cow<'static, str>>> From<T> for PageHeader {
    fn from(value: T) -> Self {
        PageHeader::new(value)
    }
}
