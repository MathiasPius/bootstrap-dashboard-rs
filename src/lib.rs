use std::{borrow::Cow, fmt::Display};

pub use askama;
use askama::Template;

mod alerts;
mod color;
pub mod files;
mod fontawesome;
mod sidebar;
mod userinfo;

pub use alerts::*;
pub use color::*;
pub use fontawesome::*;
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

/// A plain link without an icon.
pub struct PlainLink {
    pub label: Cow<'static, str>,
    pub active: bool,
    pub action: LinkAction,
}

/// A link with an associated Font-Awesome icon.
pub struct IconLink {
    pub label: Cow<'static, str>,
    pub icon: Icon,
    pub active: bool,
    pub action: LinkAction,
}

/// Complete Dashboard definition.
///
/// Renders to HTML.
#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct Dashboard<Content: Display> {
    /// Used in the top-left corner as well as the copyright notice.
    pub brand_name: &'static str,
    /// Path where static resources for the dashboard are served.
    ///
    /// See [`files::serve_at`] for more informationa as well as an example
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
