use std::{sync::Arc, time::Duration};

use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use bootstrap_dashboard::{
    htmx::{Dynamic, HxSwap, IntoDynamic, TriggerEvent},
    icons, Alert, AlertList, Alerts, Dashboard, Group, IconLink, LinkAction, Page, Sidebar,
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
        .route("/alerts", get(alerts))
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

async fn index(state: State<Arc<RwLock<Vec<Alert>>>>) -> impl IntoResponse {
    let sidebar = Sidebar::new("Dashboard", icons::fa::LAUGH_SQUINT)
        .with_group(Group::unlabeled().with_item(IconLink::new(
            "Dashboard",
            icons::fa::TACHOMETER_ALT,
            LinkAction::to("/"),
        )))
        .with_active_label("Dashboard");

    let alerts = Alerts {
        alerts: alerts(state).await.into(),
        show_all_url: Some("/notifications".into()),
    };

    let dashboard = Dashboard::default()
        .with_sidebar(sidebar)
        .with_alerts(alerts)
        .with_page_header("Dashboard")
        .replace_content("This is the front page!");

    Html(
        Page::new("Dashboard", "/static-path/nested")
            .with_content(dashboard)
            .to_string(),
    )
}

async fn alerts(State(alerts): State<Arc<RwLock<Vec<Alert>>>>) -> Dynamic<AlertList> {
    let alerts = alerts.read().await.iter().cloned().collect();

    AlertList(alerts).with_hx(
        HxSwap::get(format!("/alerts")).with_trigger(TriggerEvent::Every(Duration::from_secs(1))),
    )
}
