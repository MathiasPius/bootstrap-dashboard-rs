use std::{borrow::Cow, sync::Arc};

use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use bootstrap_dashboard::{
    icons,
    login::{LoginForm, SignupForm, UnauthenticatedNav},
    IconLink, LinkAction, Page, PlainLink,
};
use serde::Deserialize;
use tokio::{net::TcpListener, sync::RwLock};

#[derive(Debug, Clone, Default)]
struct LoginState {
    pub email: Option<String>,
    pub email_feedback: Option<String>,
    pub password_feedback: Option<String>,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(RwLock::new(LoginState::default()));

    // build our application with a route
    let app = Router::new()
        .route("/login", get(login))
        .route("/login", post(submit_login))
        .route("/signup", get(signup))
        .route("/signup", post(submit_signup))
        .merge(bootstrap_dashboard::files::serve_at(
            "/static-path/nested/*path",
        ))
        .with_state(state);

    println!("Example running at http://localhost:3000");

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Deserialize)]
struct SubmittedForm {
    pub email: String,
    pub password: String,
}

async fn submit_login(
    State(state): State<Arc<RwLock<LoginState>>>,
    Form(form): Form<SubmittedForm>,
) -> impl IntoResponse {
    {
        let mut lock = state.write().await;

        if form.email.len() > 5 {
            lock.email_feedback = Some(format!(
                "Emails should be shorter than 5 characters, which '{}' is not!",
                form.email
            ))
        }

        if form.password.chars().any(|c| c.is_alphabetic()) {
            lock.password_feedback = Some("Password can only contain numbers".to_string())
        }

        lock.email = Some(form.email);
    }

    Redirect::to("/login")
}

async fn submit_signup(
    State(state): State<Arc<RwLock<LoginState>>>,
    Form(form): Form<SubmittedForm>,
) -> impl IntoResponse {
    {
        let mut lock = state.write().await;
        lock.email = Some(form.email);
    }

    Redirect::to("/login")
}

async fn login(State(state): State<Arc<RwLock<LoginState>>>) -> impl IntoResponse {
    let login = state.read().await;

    Html(
        Page::new("Log In", "/static-path/nested")
            .with_content(LoginForm {
                submit_target: "/login".into(),
                email: login.email.clone().map(Cow::from),
                unauthenticated_nav: Some(UnauthenticatedNav {
                    header_link: PlainLink::new("Back to front page", LinkAction::Href("/".into())),
                    login_link: Some(IconLink::new(
                        "Log In",
                        icons::fa::DOOR_CLOSED,
                        LinkAction::to("/login"),
                    )),
                    signup_link: Some(IconLink::new(
                        "Sign Up",
                        icons::fa::TICKET_ALT,
                        LinkAction::to("/signup"),
                    )),
                }),
                email_feedback: login.email_feedback.clone().map(Cow::from),
                password_feedback: login.password_feedback.clone().map(Cow::from),
            })
            .to_string(),
    )
}

async fn signup(State(state): State<Arc<RwLock<LoginState>>>) -> impl IntoResponse {
    let login = state.read().await;

    Html(
        Page::new("Sign Up", "/static-path/nested")
            .with_content(SignupForm {
                submit_target: "/signup".into(),
                email: login.email.clone().map(Cow::from),

                unauthenticated_nav: Some(UnauthenticatedNav {
                    header_link: PlainLink::new("Back to front page", LinkAction::Href("/".into())),
                    login_link: Some(IconLink::new(
                        "Log In",
                        icons::fa::DOOR_CLOSED,
                        LinkAction::to("/login"),
                    )),
                    signup_link: Some(IconLink::new(
                        "Sign Up",
                        icons::fa::TICKET_ALT,
                        LinkAction::to("/signup"),
                    )),
                }),
                email_feedback: login.email_feedback.clone().map(Cow::from),
                password_feedback: login.password_feedback.clone().map(Cow::from),
            })
            .to_string(),
    )
}
