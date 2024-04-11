use std::borrow::Cow;

use askama::Template;

use crate::{IconLink, PlainLink};

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginForm {
    pub unauthenticated_nav: Option<UnauthenticatedNav>,
    pub submit_target: Cow<'static, str>,
}

#[derive(Template)]
#[template(path = "unauth_nav.html")]
pub struct UnauthenticatedNav {
    pub header_link: PlainLink,
    pub login_link: Option<IconLink>,
    pub signup_link: Option<IconLink>,
}

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignupForm {
    pub unauthenticated_nav: Option<UnauthenticatedNav>,
    pub submit_target: Cow<'static, str>,
}
