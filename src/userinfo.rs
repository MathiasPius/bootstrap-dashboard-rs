use std::borrow::Cow;

use askama::Template;

use super::IconLink;

#[derive(Template)]
#[template(path = "userinfo.html")]
pub struct UserInfo {
    pub username: String,
    pub image: Cow<'static, str>,
    pub groups: Vec<Vec<IconLink>>,
}
