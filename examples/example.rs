use axum::{
    body::Full,
    http::{header, HeaderValue, Response, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use bootstrap_dashboard::{
    Alert, Alerts, Color, Dashboard, Group, IconLink, LinkAction, NavItem, PlainLink, Sidebar,
    SubGroup, UserInfo, icons,
};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(dashboard))
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

async fn dashboard() -> impl IntoResponse {
    Html(
        Dashboard {
            copyright: "Bootstrap Dashboard",
            title: "My First Dashboard".into(),
            static_path: "/static-path",
            sidebar: Sidebar {
                name: "Dashboard".into(),
                logo: icons::fa::LAUGH_SQUINT,
                groups: vec![
                    Group {
                        label: None,
                        items: vec![
                            NavItem::Link(IconLink {
                                label: "Dashboard".into(),
                                icon: icons::fa::TACHOMETER_ALT,
                                action: LinkAction::Href("/".into()),
                                active: true,
                            }),
                            NavItem::Link(IconLink {
                                label: "Configuration".into(),
                                icon: icons::fa::COGS,
                                action: LinkAction::Href("/".into()),
                                active: false,
                            }),
                        ],
                    },
                    Group {
                        label: Some("Another Group".into()),
                        items: vec![
                            NavItem::Collapsible {
                                label: "Collapsible".into(),
                                icon: icons::fa::LIST,
                                subgroups: vec![
                                    SubGroup {
                                        label: None,
                                        links: vec![PlainLink {
                                            label: "Placeholders!".into(),
                                            active: false,
                                            action: LinkAction::Href("/".into()),
                                        }],
                                    },
                                    SubGroup {
                                        label: Some("First Subgroup".into()),
                                        links: vec![
                                            PlainLink {
                                                label: "Lorem".into(),
                                                active: false,
                                                action: LinkAction::Href("/".into()),
                                            },
                                            PlainLink {
                                                label: "Ipsum".into(),
                                                active: false,
                                                action: LinkAction::Href("/".into()),
                                            },
                                        ],
                                    },
                                    SubGroup {
                                        label: Some("Second Subgroup".into()),
                                        links: vec![
                                            PlainLink {
                                                label: "Dolor".into(),
                                                active: false,
                                                action: LinkAction::Href("/".into()),
                                            },
                                            PlainLink {
                                                label: "Sit Amet".into(),
                                                active: false,
                                                action: LinkAction::Href("/".into()),
                                            },
                                        ],
                                    },
                                ],
                            },
                            NavItem::Link(IconLink {
                                label: "Plain Link".into(),
                                icon: icons::fa::BELL,
                                action: LinkAction::Href("/".into()),
                                active: false,
                            }),
                        ],
                    },
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
                        IconLink {
                            label: "Profile".into(),
                            icon: icons::fa::USER,
                            action: LinkAction::Href("/".into()),
                            active: false,
                        },
                        IconLink {
                            label: "Settings".into(),
                            icon: icons::fa::COGS,
                            action: LinkAction::Href("/".into()),
                            active: false,
                        },
                        IconLink {
                            label: "Activity Log".into(),
                            icon: icons::fa::LIST,
                            action: LinkAction::Href("/".into()),
                            active: false,
                        },
                    ],
                    vec![IconLink {
                        label: "Logout".into(),
                        icon: icons::fa::SIGN_OUT_ALT,
                        action: LinkAction::ToggleModal("logoutModal".into()),
                        active: false,
                    }],
                ],
            }),
            content: "Hello world!",
        }
        .to_string(),
    )
}
