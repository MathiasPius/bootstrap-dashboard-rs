use axum::{
    extract::OriginalUri,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use bootstrap_dashboard::{icons, Dashboard, Group, IconLink, LinkAction, Page, Sidebar};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/link1", get(others))
        .route("/link2", get(others))
        .route("/link3", get(others))
        .merge(bootstrap_dashboard::files::serve_at(
            "/static-path/nested/*path",
        ));

    println!("Example running at http://localhost:3000");

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> impl IntoResponse {
    Html(
        Page::new("Dashboard", "/static-path/nested")
            .with_content(
                dashboard_builder()
                    .await
                    .with_active_label("Dashboard")
                    .with_page_header("Dashboard")
                    .replace_content("This is the front page!"),
            )
            .to_string(),
    )
}

async fn others(OriginalUri(uri): OriginalUri) -> impl IntoResponse {
    Html(
        Page::new("Dashboard", "/static-path/nested")
            .with_content(
                dashboard_builder()
                    .await
                    .with_active_from_path(uri.path())
                    .with_page_header("A link page")
                    .replace_content("This is a link page"),
            )
            .to_string(),
    )
}

async fn dashboard_builder() -> Dashboard {
    let sidebar = Sidebar::new("Dashboard", icons::fa::LAUGH_SQUINT).with_group(
        Group::unlabeled()
            .with_item(IconLink::new(
                "Dashboard",
                icons::fa::TACHOMETER_ALT,
                LinkAction::to("/"),
            ))
            .with_item(IconLink::new(
                "Link 1",
                icons::fa::COGS,
                LinkAction::to("/link1"),
            ))
            .with_item(IconLink::new(
                "Link 2",
                icons::fa::COGS,
                LinkAction::to("/link2"),
            ))
            .with_item(IconLink::new(
                "Link 3",
                icons::fa::COGS,
                LinkAction::to("/link3"),
            )),
    );

    Dashboard::new(sidebar).with_copyright("Bootstrap Dashboard")
}
