use axum::{
    body::Full,
    http::{header, HeaderValue, Response, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use bootstrap_dashboard::{
    icons, Alert, Alerts, Color, Dashboard, Group, IconLink, LinkAction, NavItem, PlainLink,
    Sidebar, SubGroup, UserInfo,
};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/configuration", get(configuration))
        .route("/img/undraw_profile.svg", get(serve_profile_image))
        .merge(bootstrap_dashboard::files::serve_at("/static-path/*path"));

    println!("Example running at http://localhost:3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn serve_profile_image() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            HeaderValue::from_str("image/svg+xml").unwrap(),
        )
        .body(Full::from(
            include_bytes!("img/undraw_profile.svg").as_slice(),
        ))
        .unwrap()
}
async fn index() -> impl IntoResponse {
    Html(
        dashboard_builder("This is the front page!")
            .with_active_label("Dashboard")
            .to_string(),
    )
}

async fn configuration() -> impl IntoResponse {
    Html(
        dashboard_builder("This is the configuration page!")
            .with_active_label("Configuration")
            .to_string(),
    )
}

fn dashboard_builder(content: &'static str) -> Dashboard<&'static str> {
    Dashboard {
        content,
        copyright: "Bootstrap Dashboard",
        title: "My First Dashboard".into(),
        static_path: "/static-path",
        sidebar: Sidebar {
            name: "Dashboard".into(),
            logo: icons::fa::LAUGH_SQUINT,
            groups: vec![
                Group::unlabeled()
                    .with_item(IconLink::new(
                        "Dashboard",
                        icons::fa::TACHOMETER_ALT,
                        LinkAction::to("/"),
                    ))
                    .with_item(IconLink::new(
                        "Configuration",
                        icons::fa::COGS,
                        LinkAction::to("/configuration"),
                    )),
                Group::new("Another Group")
                    .with_item(NavItem::collapsible(
                        "Collapsible",
                        icons::fa::LIST,
                        vec![
                            SubGroup::unlabeled()
                                .with_link(PlainLink::new("Placeholders!", LinkAction::to("/"))),
                            SubGroup::new("First Subgroup")
                                .with_link(PlainLink::new("Lorem", LinkAction::to("/")))
                                .with_link(PlainLink::new("Ipsum", LinkAction::to("/"))),
                            SubGroup::new("Second Subgroup")
                                .with_link(PlainLink::new("Dolor", LinkAction::to("/")))
                                .with_link(PlainLink::new("Sit Amet", LinkAction::to("/"))),
                        ],
                    ))
                    .with_item(IconLink::new(
                        "Plain Link",
                        icons::fa::BELL,
                        LinkAction::to("/"),
                    )),
            ],
        },
        alerts: Some(Alerts {
            alerts: vec![
                Alert {
                    color: Color::Primary,
                    icon: icons::fa::MONEY_BILL,
                    headline: "December 7, 1991".to_string(),
                    message: "A new monthly report is ready to download!".to_string(),
                    unread: true,
                },
                Alert {
                    color: Color::Secondary,
                    icon: icons::fa::DONATE,
                    headline: "December 7, 1991".to_string(),
                    message: "$290.29 has been deposited into your account!".to_string(),
                    unread: false,
                },
            ],
            show_all_url: Some("/notifications".into()),
        }),
        userinfo: Some(UserInfo {
            username: "John Smith".into(),
            image: "/img/undraw_profile.svg".into(),
            groups: vec![
                vec![
                    IconLink::new("Profile", icons::fa::USER, LinkAction::to("/")).into(),
                    IconLink::new("Settings", icons::fa::COGS, LinkAction::to("/")).into(),
                    IconLink::new("Activity Log", icons::fa::LIST, LinkAction::to("/")).into(),
                ],
                vec![IconLink::new(
                    "Logout",
                    icons::fa::SIGN_OUT_ALT,
                    LinkAction::modal("logoutModal"),
                )
                .into()],
            ],
        }),
    }
}
