use std::borrow::Cow;

use askama::Template;

use crate::links::{NavLink, PlainLink};

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginForm {
    pub unauthenticated_nav: Option<UnauthenticatedNav>,
    pub submit_target: Cow<'static, str>,
    pub email: Option<Cow<'static, str>>,
    pub email_feedback: Option<Cow<'static, str>>,
    pub password_feedback: Option<Cow<'static, str>>,
}

#[derive(Template)]
#[template(path = "unauth_nav.html")]
pub struct UnauthenticatedNav {
    pub header_link: PlainLink,
    pub login_link: Option<NavLink>,
    pub signup_link: Option<NavLink>,
}

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignupForm {
    pub unauthenticated_nav: Option<UnauthenticatedNav>,
    pub submit_target: Cow<'static, str>,
    pub email: Option<Cow<'static, str>>,
    pub email_feedback: Option<Cow<'static, str>>,
    pub password_feedback: Option<Cow<'static, str>>,
}
