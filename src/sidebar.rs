use std::borrow::Cow;

use askama::Template;

use crate::Icon;

use super::{IconLink, PlainLink};

/// Sidebar menu [`Group`]s have optional labels, and are always
/// separated by a divider.
pub struct Group {
    /// Optional Group label.
    pub label: Option<Cow<'static, str>>,
    /// Group's navigation items.
    pub items: Vec<NavItem>,
}

impl Group {
    pub fn unlabeled() -> Self {
        Group {
            label: None,
            items: Vec::new(),
        }
    }

    pub fn new<S: Into<Cow<'static, str>>>(label: S) -> Self {
        Group {
            label: Some(label.into()),
            items: Vec::new(),
        }
    }

    pub fn with_label<S: Into<Cow<'static, str>>>(self, label: S) -> Self {
        Group {
            label: Some(label.into()),
            items: self.items,
        }
    }

    pub fn with_item<I: Into<NavItem>>(mut self, item: I) -> Self {
        self.items.push(item.into());

        Group {
            label: self.label,
            items: self.items,
        }
    }
}

impl From<Vec<NavItem>> for Group {
    fn from(value: Vec<NavItem>) -> Self {
        Group {
            label: None,
            items: value,
        }
    }
}

/// Top-level sidebar menu item containing either a direct link/modal toggle,
/// or a collapsible menu item with further sub-groups of links.
pub enum NavItem {
    /// Label & Icon which links to page or toggles a Modal.
    Link(IconLink),
    /// Collapsible sub-menu containing one or more groups of links.
    Collapsible {
        /// Label for the collapsible menu item.
        label: Cow<'static, str>,
        /// Font-Awesome icon.
        icon: Icon,
        /// [`SubGroup`] elements contained within the collapsible menu.
        subgroups: Vec<SubGroup>,
    },
}

impl NavItem {
    pub fn collapsible<S: Into<Cow<'static, str>>>(
        label: S,
        icon: Icon,
        subgroups: Vec<SubGroup>,
    ) -> Self {
        NavItem::Collapsible {
            label: label.into(),
            icon,
            subgroups,
        }
    }
}

impl From<IconLink> for NavItem {
    fn from(value: IconLink) -> Self {
        NavItem::Link(value)
    }
}

impl NavItem {
    /// Get the label for the [`NavItem`]
    pub fn label(&self) -> &Cow<'static, str> {
        match self {
            NavItem::Link(IconLink { label, .. }) | NavItem::Collapsible { label, .. } => label,
        }
    }
}

/// [`SubGroup`]s are an element of [`NavItem::Collapsible`] sidebar objects,
/// which allow you to group links using a de-emphasized label.
pub struct SubGroup {
    pub label: Option<Cow<'static, str>>,
    pub links: Vec<PlainLink>,
}

impl SubGroup {
    pub fn new<S: Into<Cow<'static, str>>>(label: S) -> SubGroup {
        SubGroup {
            label: Some(label.into()),
            links: Vec::new(),
        }
    }

    pub fn unlabeled() -> Self {
        SubGroup {
            label: None,
            links: Vec::new(),
        }
    }

    pub fn with_link(mut self, link: PlainLink) -> Self {
        self.links.push(link);
        SubGroup {
            label: self.label,
            links: self.links,
        }
    }
}

/// Dashboard logo and title as well as left-hand side menu.
///
/// Supports both plain links and collapsible categories of links via
/// [`NavItem::Link`] and [`NavItem::Collapsible`] respectively.
///
/// ## Structure
///
/// [`Sidebar`] contains many [`Group`]s.
///
/// Each [`Group`]:
/// * Optionally has a header label.
/// * Contains many [`NavItem`]s.
///
/// Each [`NavItem`] is *either*:
/// * a [`NavItem::Link`] with [`IconLink`] and action directly associated, or
/// * a [`NavItem::Collapsible`] with one or more [`SubGroup`]s.
///
/// Each [`SubGroup`]:
/// * Optionally contains a header label.
/// * Contains many [`PlainLink`]s.
#[derive(Template)]
#[template(path = "sidebar.html")]
pub struct Sidebar {
    pub name: Cow<'static, str>,
    pub logo: Icon,
    pub groups: Vec<Group>,
}

impl Sidebar {
    pub fn new<S: Into<Cow<'static, str>>>(name: S, logo: Icon) -> Self {
        Sidebar {
            name: name.into(),
            logo,
            groups: Vec::new(),
        }
    }

    pub fn with_group(mut self, group: Group) -> Self {
        self.groups.push(group);
        self
    }
}
