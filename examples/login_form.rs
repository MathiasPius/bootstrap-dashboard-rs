use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use bootstrap_dashboard::{
    icons,
    login::{LoginForm, SignupForm, UnauthenticatedNav},
    IconLink, LinkAction, Page, PlainLink,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/login", get(login))
        .route("/signup", get(signup))
        .merge(bootstrap_dashboard::files::serve_at(
            "/static-path/nested/*path",
        ));

    println!("Example running at http://localhost:3000");

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn login() -> impl IntoResponse {
    Html(
        Page::new("Log In", "/static-path/nested")
            .with_content(LoginForm {
                submit_target: "/login".into(),
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
            })
            .to_string(),
    )
}

async fn signup() -> impl IntoResponse {
    Html(
        Page::new("Sign Up", "/static-path/nested")
            .with_content(SignupForm {
                submit_target: "/signup".into(),
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
            })
            .to_string(),
    )
}
