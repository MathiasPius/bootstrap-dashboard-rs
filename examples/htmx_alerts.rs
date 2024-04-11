use std::{sync::Arc, time::Duration};

use axum::{
    body::Body,
    extract::State,
    http::{header, HeaderValue, Response, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use bootstrap_dashboard::{
    card::Card,
    grid::{Breakpoint, Column, Row},
    htmx::{Dynamic, Hx, IntoDynamic, TriggerEvent},
    icons, Alert, AlertList, Alerts, Dashboard, Group, IconLink, LinkAction, NavItem, Page,
    PlainLink, Sidebar, SubGroup, UserInfo,
};
use tokio::{net::TcpListener, sync::RwLock};

#[tokio::main]
async fn main() {
    let alert_vec = Arc::new(RwLock::new(vec![Alert::new(
        "December 7, 1991",
        "A new monthly report is ready to download!",
    )
    .unread()]));

    let alerts_clone = alert_vec.clone();
    tokio::spawn(async move {
        let mut index = 0;
        loop {
            {
                let mut lock = alerts_clone.write().await;
                lock.push(Alert::new(
                    format!("Alert number {index}!"),
                    "Should probably really act on this!",
                ));
            }
            index += 1;

            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });

    // build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/configuration", get(configuration))
        .route("/alerts", get(alerts))
        .route("/img/undraw_profile.svg", get(serve_profile_image))
        .merge(bootstrap_dashboard::files::serve_at(
            "/static-path/nested/*path",
        ))
        .with_state(alert_vec);

    println!("Example running at http://localhost:3000");

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service())
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
        .body(Body::from(
            include_bytes!("img/undraw_profile.svg").as_slice(),
        ))
        .unwrap()
}

async fn index(state: State<Arc<RwLock<Vec<Alert>>>>) -> impl IntoResponse {
    Html(
        Page::new("Dashboard", "/static-path/nested")
            .with_content(
                dashboard_builder(state)
                    .await
                    .with_active_label("Dashboard")
                    .with_page_header("Dashboard")
                    .replace_content("This is the front page!"),
            )
            .to_string(),
    )
}

async fn configuration(state: State<Arc<RwLock<Vec<Alert>>>>) -> impl IntoResponse {
    let row1 = Row::new()
        .with_column(
            Card::new("Hello world")
                .with_header("First card")
                .to_string(),
        )
        .with_column(
            Column::new(Card::new("Hello world").to_string()).with_size(Breakpoint::ExtraLarge, 2),
        )
        .with_column(
            Column::new(
                Card::new("Hello world")
                    .with_header("Large Card")
                    .to_string(),
            )
            .with_size(Breakpoint::ExtraLarge, 7),
        );

    Html(
        Page::new("My First Dashbaord", "/static-path/nested")
            .with_content(
                dashboard_builder(state)
                    .await
                    .with_active_label("Configuration")
                    .with_page_header("Configuration")
                    .replace_content(row1),
            )
            .to_string(),
    )
}

async fn alerts(State(alerts): State<Arc<RwLock<Vec<Alert>>>>) -> Dynamic<AlertList> {
    let alerts = alerts.read().await.iter().cloned().collect();

    AlertList(alerts).with_hx(
        Hx::get(format!("/alerts")).with_trigger(TriggerEvent::Every(Duration::from_secs(1))),
    )
}

async fn dashboard_builder(state: State<Arc<RwLock<Vec<Alert>>>>) -> Dashboard {
    let sidebar = Sidebar::new("Dashboard", icons::fa::LAUGH_SQUINT)
        .with_group(
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
        )
        .with_group(
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
        );

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

    let alerts = Alerts {
        alerts: alerts(state).await.into(),
        show_all_url: Some("/notifications".into()),
    };

    Dashboard::new(sidebar)
        .with_copyright("Bootstrap Dashboard")
        .with_alerts(alerts)
        .with_userinfo(userinfo)
}