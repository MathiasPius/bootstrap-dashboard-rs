use std::{borrow::Cow, fmt::Display};

pub use askama::Template;

use self::{alerts::Alerts, sidebar::Sidebar, userinfo::UserInfo};

pub mod alerts;
pub mod color;
pub mod files;
pub mod sidebar;
pub mod userinfo;

pub enum LinkAction {
    Href(Cow<'static, str>),
    Modal(Cow<'static, str>),
}

impl LinkAction {
    pub fn href(&self) -> Cow<'static, str> {
        match self {
            LinkAction::Href(url) => url.clone(),
            LinkAction::Modal(_) => "#".into(),
        }
    }

    pub fn props(&self) -> Cow<'static, str> {
        match self {
            LinkAction::Href(_) => "".into(),
            LinkAction::Modal(modal) => {
                format!(" data-toggle=\"modal\" data-target=\"#{modal}\"").into()
            }
        }
    }
}

pub struct BasicLink {
    pub label: Cow<'static, str>,
    pub active: bool,
    pub action: LinkAction,
}

pub struct IconLink {
    pub label: Cow<'static, str>,
    pub icon: Cow<'static, str>,
    pub active: bool,
    pub action: LinkAction,
}

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct Dashboard<Content: Display> {
    pub brand_name: &'static str,
    pub static_path: &'static str,
    pub title: Cow<'static, str>,
    pub sidebar: Sidebar,
    pub alerts: Alerts,
    pub userinfo: UserInfo,
    pub content: Content,
}
