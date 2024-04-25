use std::borrow::Cow;

use askama::Template;

use crate::{modal::Modal, Icon};

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

    pub fn modal_name<S: Into<Cow<'static, str>>>(name: S) -> Self {
        LinkAction::ToggleModal(name.into())
    }

    pub fn modal(modal: &Modal) -> Self {
        LinkAction::ToggleModal(modal.id.clone().into())
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

/// A plain link without an icon.
#[derive(Template)]
#[template(
    ext = "html",
    source = r#"
<a class="nav-link" href="{{ action.href() }}" {{ action.props()|safe }}>
<span>{{ label }}</span></a>
"#
)]
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

#[derive(Template)]
#[template(
    ext = "html",
    source = r#"
<a class="nav-link" href="{{ action.href() }}" {{ action.props()|safe }}>
<i class="fas fa-fw {{ icon }}"></i>
<span>{{ label }}</span></a>
"#
)]
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

#[derive(Template)]
#[template(
    ext = "html",
    source = r#"
{% match self %}
{% when NavLink::Plain with (plain) %}
{{ plain|safe }}
{% when NavLink::Icon with (icon) %}
{{ icon|safe }}
{% endmatch %}
"#
)]
/// Either a plain or icon link.
pub enum NavLink {
    Plain(PlainLink),
    Icon(IconLink),
}

impl NavLink {
    pub fn label(&self) -> &Cow<'static, str> {
        match self {
            NavLink::Plain(plain) => &plain.label,
            NavLink::Icon(icon) => &icon.label,
        }
    }

    pub fn icon(&self) -> Option<&Icon> {
        if let NavLink::Icon(IconLink { icon, .. }) = self {
            Some(icon)
        } else {
            None
        }
    }

    pub fn action(&self) -> &LinkAction {
        match self {
            NavLink::Plain(plain) => &plain.action,
            NavLink::Icon(icon) => &icon.action,
        }
    }

    pub fn active(&self) -> bool {
        match self {
            NavLink::Plain(plain) => plain.active,
            NavLink::Icon(icon) => icon.active,
        }
    }
}

impl From<IconLink> for NavLink {
    fn from(value: IconLink) -> Self {
        NavLink::Icon(value)
    }
}

impl From<PlainLink> for NavLink {
    fn from(value: PlainLink) -> Self {
        NavLink::Plain(value)
    }
}
