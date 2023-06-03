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
        dashboard_builder()
            .with_active_label("Dashboard")
            .replace_content("This is the front page!")
            .to_string(),
    )
}

async fn configuration() -> impl IntoResponse {
    Html(
        dashboard_builder()
            .with_active_label("Configuration")
            .replace_content("This is the configuration page!")
            .to_string(),
    )
}

fn dashboard_builder() -> Dashboard {
    let sidebar = Sidebar {
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
    };

    let alerts = Alerts {
        alerts: vec![
            Alert::new(
                "December 7, 1991",
                "A new monthly report is ready to download!",
            )
            .unread(),
            Alert::new(
                "$290.29 has been deposited into your account!",
                "A new monthly report is ready to download!",
            )
            .with_icon(icons::fa::DONATE)
            .with_color(Color::Secondary),
        ],
        show_all_url: Some("/notifications".into()),
    };

    let userinfo = UserInfo {
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
    };

    Dashboard::new("My First Dashboard", "/static-path", sidebar)
        .with_copyright("Bootstrap Dashboard")
        .with_alerts(alerts)
        .with_userinfo(userinfo)
}
