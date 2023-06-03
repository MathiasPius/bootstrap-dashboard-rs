use std::borrow::Cow;

use askama::Template;

use super::{IconLink, PlainLink};

pub struct Group {
    pub label: Option<Cow<'static, str>>,
    pub items: Vec<NavItem>,
}

#[allow(unused)]
pub enum NavItem {
    Link(IconLink),
    Collapsible {
        label: Cow<'static, str>,
        icon: Cow<'static, str>,
        subgroups: Vec<SubGroup>,
    },
}

impl NavItem {
    pub fn label(&self) -> &Cow<'static, str> {
        match self {
            NavItem::Link(IconLink { label, .. }) | NavItem::Collapsible { label, .. } => label,
        }
    }
}

pub struct SubGroup {
    pub label: Option<Cow<'static, str>>,
    pub links: Vec<PlainLink>,
}

#[derive(Template)]
#[template(path = "sidebar.html")]
pub struct Sidebar {
    pub name: Cow<'static, str>,
    pub logo: Cow<'static, str>,
    pub groups: Vec<Group>,
}
