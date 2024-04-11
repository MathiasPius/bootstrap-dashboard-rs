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
pub mod htmx;
pub mod icons;
mod page_header;
mod sidebar;
mod userinfo;

pub use alerts::*;
pub use color::*;
use htmx::Dynamic;
pub use icons::Icon;
pub use page_header::PageHeader;
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

/// Base template upon which all other types must be embedded.
///
/// Structures the page and includes necessary css and javascript files.
#[derive(Template)]
#[template(path = "page.html")]
pub struct Page<Content: Display = &'static str> {
    pub title: Cow<'static, str>,
    /// Path where static resources for the dashboard are served.
    ///
    /// See [`files::serve_at`] for more information as well as an example
    /// of how this could be done using [axum](https://github.com/tokio-rs/axum)
    pub static_path: Cow<'static, str>,
    pub content: Content,
}

impl Page<&'static str> {
    pub fn new<S1: Into<Cow<'static, str>>, S2: Into<Cow<'static, str>>>(
        title: S1,
        static_path: S2,
    ) -> Self {
        Page {
            title: title.into(),
            static_path: static_path.into(),
            content: "",
        }
    }
}

impl<Content: Display> Page<Content> {
    pub fn with_content<NewContent: Display>(self, content: NewContent) -> Page<NewContent> {
        Page {
            title: self.title,
            static_path: self.static_path,
            content,
        }
    }
}

/// Dashboard definition.
///
/// Embed within a [`Page`] before rendering.
#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct Dashboard<Content: Display = &'static str> {
    /// Used for copyright notice.
    pub copyright: Option<Cow<'static, str>>,
    /// [`Sidebar`] structure defining the layout of the left-hand menu.
    pub sidebar: Sidebar,
    pub alerts: Option<Dynamic<Alerts>>,
    pub userinfo: Option<UserInfo>,
    pub page_header: Option<PageHeader>,
    pub content: Content,
}

impl Dashboard<&'static str> {
    pub fn new(sidebar: Sidebar) -> Self {
        Dashboard {
            copyright: None,
            sidebar,
            alerts: None,
            userinfo: None,
            page_header: None,
            content: "",
        }
    }
}

impl<Content: Display> Dashboard<Content> {
    pub fn with_copyright<S: Into<Cow<'static, str>>>(mut self, copyright: S) -> Self {
        self.copyright = Some(copyright.into());
        self
    }

    pub fn with_alerts<T: Into<Dynamic<Alerts>>>(mut self, alerts: T) -> Self {
        self.alerts = Some(alerts.into());
        self
    }

    pub fn with_userinfo(mut self, userinfo: UserInfo) -> Self {
        self.userinfo = Some(userinfo);
        self
    }

    pub fn with_page_header<P: Into<PageHeader>>(mut self, page_header: P) -> Self {
        self.page_header = Some(page_header.into());
        self
    }

    pub fn replace_content<NewContent: Display>(
        self,
        content: NewContent,
    ) -> Dashboard<NewContent> {
        Dashboard {
            copyright: self.copyright,
            sidebar: self.sidebar,
            alerts: self.alerts,
            userinfo: self.userinfo,
            page_header: self.page_header,
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
    /// #   copyright: None,
    /// #   page_header: None,
    /// #   alerts: None,
    /// #   userinfo: None,
    /// #   content: "",
    /// }.with_active_label("Configuration");
    ///
    /// ```
    pub fn with_active_label(self, active_label: &str) -> Self {
        self.with_active(|_, label| label == active_label)
    }

    /// Given the currently active URL, attempt to deduce the active link
    /// by inspecting the target URLs and comparing them.
    pub fn with_active_from_path(self, current_path: &str) -> Self {
        self.with_active(|action, _| {
            if let LinkAction::Href(url) = action {
                if current_path.ends_with(url.as_ref()) {
                    return true;
                }
            }

            false
        })
    }

    fn with_active(mut self, selector: impl for<'r> Fn(&'r LinkAction, &'r str) -> bool) -> Self {
        'outer: for group in &mut self.sidebar.groups {
            for item in &mut group.items {
                match item {
                    NavItem::Link(link) => {
                        if selector(&link.action, &link.label) {
                            link.active = true;
                            break 'outer;
                        }
                    }
                    NavItem::Collapsible { subgroups, .. } => {
                        for subgroup in subgroups {
                            for link in &mut subgroup.links {
                                if selector(&link.action, &link.label) {
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
