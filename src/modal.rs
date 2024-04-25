use std::borrow::Cow;

use askama::Template;

#[derive(Template)]
#[template(path = "modal.html")]
pub struct Modal {
    pub id: Cow<'static, str>,
    pub header: Cow<'static, str>,
    pub content: Cow<'static, str>,
    pub action: ModalAction,
    pub confirm: Cow<'static, str>,
}

#[derive(Debug, Clone)]
pub enum ModalAction {
    Post(Cow<'static, str>),
    Get(Cow<'static, str>),
}
