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
pub mod favicons;
pub mod files;
pub mod grid;
pub mod htmx;
pub mod icons;
mod links;
pub mod login;
mod page_header;
mod sidebar;
mod userinfo;

pub use alerts::*;
pub use color::*;
use favicons::FavIcons;
pub use htmx::Dynamic;
pub use icons::Icon;
pub use links::{IconLink, LinkAction, NavLink, PlainLink};
pub use page_header::PageHeader;
pub use sidebar::*;
pub use userinfo::*;

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
    pub favicons: Option<FavIcons>,
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
            favicons: None,
            content: "",
        }
    }
}

impl<Content: Display> Page<Content> {
    pub fn with_content<NewContent: Display>(self, content: NewContent) -> Page<NewContent> {
        Page {
            title: self.title,
            static_path: self.static_path,
            favicons: None,
            content,
        }
    }

    pub fn with_favicons(self, favicons: FavIcons) -> Self {
        Page {
            title: self.title,
            static_path: self.static_path,
            favicons: Some(favicons),
            content: self.content,
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
    pub sidebar: Option<Sidebar>,
    pub alerts: Option<Dynamic<Alerts>>,
    pub userinfo: Option<UserInfo>,
    pub page_header: Option<PageHeader>,
    pub content: Content,
}

impl Dashboard<&'static str> {
    pub fn new() -> Self {
        Dashboard {
            copyright: None,
            sidebar: None,
            alerts: None,
            userinfo: None,
            page_header: None,
            content: "",
        }
    }
}

impl Default for Dashboard<&'static str> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Content: Display> Dashboard<Content> {
    pub fn with_sidebar<S: Into<Sidebar>>(mut self, sidebar: S) -> Self {
        self.sidebar = Some(sidebar.into());
        self
    }

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
}
