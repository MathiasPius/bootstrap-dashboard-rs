//! Batteries-included bootstrap-based dashboard template.
//!
//! This crate provides structures and enums for rendering interactive dashboards based on [SB-Admin-2](https://startbootstrap.com/theme/sb-admin-2)
//!
//! See [examples/example.rs](examples/example.rs) for usage instructions.
use std::{borrow::Cow, fmt::Display};

pub use askama;
use askama::Template;

mod alerts;
pub mod card;
mod color;
pub mod files;
pub mod grid;
pub mod icons;
mod sidebar;
mod userinfo;

pub use alerts::*;
pub use color::*;
pub use icons::Icon;
pub use sidebar::*;
pub use userinfo::*;

/// Action to take on link press.
///
/// Can either act as a regular link, sending the user to a new path,
/// or toggle a modal, such as a logout pop-up window warning or similar.
pub enum LinkAction {
    Href(Cow<'static, str>),
    ToggleModal(Cow<'static, str>),
}

impl LinkAction {
    pub fn to<S: Into<Cow<'static, str>>>(url: S) -> Self {
        LinkAction::Href(url.into())
    }

    pub fn modal<S: Into<Cow<'static, str>>>(name: S) -> Self {
        LinkAction::ToggleModal(name.into())
    }

    /// Contents of the `href` attribute of the link
    ///
    /// For [`LinkAction::Href`] this is the target URL.
    ///
    /// For [`LinkAction::ToggleModal`] this will always be `#`
    pub fn href(&self) -> Cow<'static, str> {
        match self {
            LinkAction::Href(url) => url.clone(),
            LinkAction::ToggleModal(_) => "#".into(),
        }
    }

    /// Additional properties to add to the containing link.
    ///
    /// Regular [`LinkAction::Href`] links won't have any, whereas
    /// [`LinkAction::ToggleModal`] will have extra properties indicating
    /// to [bootstrap](https://getbootstrap.com/docs/4.0/components/modal/)
    /// which modal to toggle.
    pub fn props(&self) -> Cow<'static, str> {
        match self {
            LinkAction::Href(_) => "".into(),
            LinkAction::ToggleModal(modal) => {
                format!(" data-toggle=\"modal\" data-target=\"#{modal}\"").into()
            }
        }
    }
}

/// A simple Label
pub struct Label(Cow<'static, str>);

impl Label {
    pub fn new<S: Into<Cow<'static, str>>>(label: S) -> Self {
        Label(label.into())
    }

    pub fn with_link(self, action: LinkAction) -> PlainLink {
        PlainLink {
            label: self.0,
            active: false,
            action,
        }
    }
}

/// A plain link without an icon.
pub struct PlainLink {
    pub label: Cow<'static, str>,
    pub active: bool,
    pub action: LinkAction,
}

impl PlainLink {
    pub fn new<S: Into<Cow<'static, str>>>(label: S, action: LinkAction) -> Self {
        PlainLink {
            label: label.into(),
            active: false,
            action,
        }
    }

    pub fn with_icon(self, icon: Icon) -> IconLink {
        IconLink {
            label: self.label,
            icon,
            active: self.active,
            action: self.action,
        }
    }
}

/// A link with an associated Font-Awesome icon.
pub struct IconLink {
    pub label: Cow<'static, str>,
    pub icon: Icon,
    pub active: bool,
    pub action: LinkAction,
}

impl IconLink {
    pub fn new<S: Into<Cow<'static, str>>>(label: S, icon: Icon, action: LinkAction) -> Self {
        IconLink {
            label: label.into(),
            active: false,
            icon,
            action,
        }
    }
}

/// Complete Dashboard definition.
///
/// Renders to HTML.
#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct Dashboard<Content: Display = &'static str> {
    /// Used for copyright notice.
    pub copyright: Option<Cow<'static, str>>,
    /// Path where static resources for the dashboard are served.
    ///
    /// See [`files::serve_at`] for more information as well as an example
    /// of how this could be done using [axum](https://github.com/tokio-rs/axum)
    pub static_path: &'static str,
    /// Title of the web page.
    pub title: Cow<'static, str>,
    /// [`Sidebar`] structure defining the layout of the left-hand menu.
    pub sidebar: Sidebar,
    pub alerts: Option<Alerts>,
    pub userinfo: Option<UserInfo>,
    pub content: Content,
}

impl Dashboard<&'static str> {
    pub fn new<S1: Into<Cow<'static, str>>>(
        title: S1,
        static_path: &'static str,
        sidebar: Sidebar,
    ) -> Self {
        Dashboard {
            copyright: None,
            static_path,
            title: title.into(),
            sidebar,
            alerts: None,
            userinfo: None,
            content: "",
        }
    }
}

impl<Content: Display> Dashboard<Content> {
    pub fn with_copyright<S: Into<Cow<'static, str>>>(mut self, copyright: S) -> Self {
        self.copyright = Some(copyright.into());
        self
    }

    pub fn with_alerts(mut self, alerts: Alerts) -> Self {
        self.alerts = Some(alerts);
        self
    }

    pub fn with_userinfo(mut self, userinfo: UserInfo) -> Self {
        self.userinfo = Some(userinfo);
        self
    }

    pub fn replace_content<NewContent: Display>(
        self,
        content: NewContent,
    ) -> Dashboard<NewContent> {
        Dashboard {
            copyright: self.copyright,
            static_path: self.static_path,
            title: self.title,
            sidebar: self.sidebar,
            alerts: self.alerts,
            userinfo: self.userinfo,
            content,
        }
    }

    /// Sets the `active` field of the first [`IconLink`] or [`PlainLink`] whose
    /// label matches the provided `active_label`.
    ///
    /// # Example
    /// ```rust
    /// # use bootstrap_dashboard::{Dashboard, SubGroup, Group, IconLink, icons, LinkAction, Sidebar};
    /// let dashboard = Dashboard {
    ///     sidebar: Sidebar {
    ///         groups: vec![
    ///             Group::unlabeled()
    ///                 .with_item(IconLink::new(
    ///                     "Dashboard",
    ///                     icons::fa::TACHOMETER_ALT,
    ///                     LinkAction::to("/"),
    ///                 ))
    ///                 // This is the item which wil be marked "active".
    ///                 .with_item(IconLink::new(
    ///                     "Configuration",
    ///                     icons::fa::COGS,
    ///                     LinkAction::to("/"),
    ///                 )),
    ///         ],
    ///         // ...
    /// #       name: "".into(),
    /// #       logo: icons::fa::LAUGH_WINK
    ///     },
    ///     // ...
    /// #   copyright: "",
    /// #   static_path: "",
    /// #   title: "".into(),
    /// #   alerts: None,
    /// #   userinfo: None,
    /// #   content: "",
    /// }.with_active_label("Configuration");
    ///
    /// ```
    pub fn with_active_label(self, active_label: &str) -> Self {
        self.with_active(|label| label == active_label)
    }

    fn with_active(mut self, selector: impl for<'r> Fn(&'r str) -> bool) -> Self {
        'outer: for group in &mut self.sidebar.groups {
            for item in &mut group.items {
                match item {
                    NavItem::Link(link) => {
                        if selector(&link.label) {
                            link.active = true;
                            break 'outer;
                        }
                    }
                    NavItem::Collapsible { subgroups, .. } => {
                        for subgroup in subgroups {
                            for link in &mut subgroup.links {
                                if selector(&link.label) {
                                    link.active = true;
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
            }
        }

        self
    }
}
