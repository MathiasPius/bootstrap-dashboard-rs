use std::borrow::Cow;

use askama::Template;

use super::IconLink;

/// UserInfo object shown in the top-right of the dashboard
/// indicating the currently logged-in user, and providing a
/// menu for interacting with this persona.
#[derive(Template)]
#[template(path = "userinfo.html")]
pub struct UserInfo {
    /// Displayed username.
    pub username: String,
    /// Image URL for the user's profile picture.
    pub image: Cow<'static, str>,
    /// List-of-list-of [`IconLink`]s.
    ///
    /// The list-of-list structure is used for dividing menu items by
    /// type or category. Using a single `vec![vec![(items..)]]` will
    /// render all links as a single un-divided list.
    pub groups: Vec<Vec<IconLink>>,
}
