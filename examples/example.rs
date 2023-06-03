use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use dashboard::{
    alerts::{Alert, Alerts},
    color::Color,
    sidebar::{Group, NavItem, Sidebar, SubGroup},
    userinfo::UserInfo,
    Dashboard, IconLink, LinkAction, PlainLink, Template,
};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(dashboard))
        .merge(dashboard::files::serve_at("/static-path/*path"));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn dashboard() -> impl IntoResponse {
    Html(
        Dashboard {
            brand_name: "Dashboard",
            title: "My First Dashboard".into(),
            static_path: "/static-path",
            sidebar: Sidebar {
                name: "Dashboard".into(),
                logo: "fa-laugh-wink".into(),
                groups: vec![Group {
                    label: None,
                    items: vec![NavItem::Link(IconLink {
                        label: "Dashboard".into(),
                        icon: "fa-tachometer-alt".into(),
                        action: LinkAction::Href("/".into()),
                        active: true,
                    }),
                    NavItem::Link(IconLink {
                        label: "Configuration".into(),
                        icon: "fa-cogs".into(),
                        action: LinkAction::Href("/".into()),
                        active: false,
                    })],
                },
                Group {
                    label: Some("Another Group".into()), items: vec![
                        NavItem::Collapsible { label: "Collapsible".into(), icon: "fa-list".into(), subgroups: vec![
                            SubGroup { label: None, links: vec![
                                PlainLink { label: "Placeholders!".into(), active: false, action: LinkAction::Href("/".into()) },
                            ] },
                            SubGroup { label: Some("First Subgroup".into()), links: vec![
                                PlainLink { label: "Lorem".into(), active: false, action: LinkAction::Href("/".into()) },
                                PlainLink { label: "Ipsum".into(), active: false, action: LinkAction::Href("/".into()) },
                            ] },
                            SubGroup { label: Some("Second Subgroup".into()), links: vec![
                                PlainLink { label: "Dolor".into(), active: false, action: LinkAction::Href("/".into()) },
                                PlainLink { label: "Sit Amet".into(), active: false, action: LinkAction::Href("/".into()) },
                            ] }
                        ] },
                        NavItem::Link(IconLink {
                            label: "Plain Link".into(),
                            icon: "fa-bell".into(),
                            action: LinkAction::Href("/".into()),
                            active: false,
                        })
                    ]
                }],
            },
            alerts: Some(Alerts {
                alerts: vec![
                    Alert {
                        color: Color::Primary,
                        icon: "fa-donate",
                        headline: "December 7, 1991".to_string(),
                        message: "A new monthly report is ready to download!".to_string(),
                        unread: true,
                    },
                    Alert {
                        color: Color::Secondary,
                        icon: "fa-donate",
                        headline: "December 7, 1991".to_string(),
                        message: "$290.29 has been deposited into your account!".to_string(),
                        unread: false,
                    },
                ],
                show_all_url: Some("/notifications"),
            }),
            userinfo: Some(UserInfo {
                username: "John Smith".into(),
                image: "https://startbootstrap.github.io/startbootstrap-sb-admin-2/img/undraw_profile.svg".into(),
                groups: vec![
                    vec![
                        IconLink { label: "Profile".into(), icon: "fa-user".into(), action: LinkAction::Href("/".into()), active: false },
                        IconLink { label: "Settings".into(), icon: "fa-cogs".into(), action: LinkAction::Href("/".into()), active: false },
                        IconLink { label: "Activity Log".into(), icon: "fa-list".into(), action: LinkAction::Href("/".into()), active: false },
                    ],
                    vec![
                        IconLink { label: "Logout".into(), icon: "fa-sign-out-alt".into(), action: LinkAction::ToggleModal("logoutModal".into()), active: false }
                    ]
                ],
            }),
            content: "Hello world!"
        }
        .render()
        .unwrap(),
    )
}
