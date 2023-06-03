use std::borrow::Cow;

use askama::Template;

use super::{IconLink, PlainLink};

/// Sidebar menu [`Group`]s have optional labels, and are always
/// separated by a divider.
pub struct Group {
    /// Optional Group label.
    pub label: Option<Cow<'static, str>>,
    /// Group's navigation items.
    pub items: Vec<NavItem>,
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
        icon: Cow<'static, str>,
        /// [`SubGroup`] elements contained within the collapsible menu.
        subgroups: Vec<SubGroup>,
    },
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
    pub logo: Cow<'static, str>,
    pub groups: Vec<Group>,
}
