use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use bootstrap_dashboard::{
    icons, Dashboard, Group, IconLink, LinkAction, NavItem, Page, PlainLink, Sidebar, SubGroup,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/configuration", get(configuration))
        .merge(bootstrap_dashboard::files::serve_at("/static-path/*path"));

    println!("Example running at http://localhost:3000");

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> impl IntoResponse {
    Html(
        Page::new("Front Page", "/static-path")
            .with_content(dashboard_builder("Dashboard").replace_content("This is the front page!"))
            .to_string(),
    )
}

async fn configuration() -> impl IntoResponse {
    Html(
        Page::new("Configuration Page", "/static-path")
            .with_content(
                dashboard_builder("Configuration")
                    .replace_content("This is the configuration page!"),
            )
            .to_string(),
    )
}

fn dashboard_builder(active_label: &str) -> Dashboard {
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
    }
    .with_active_label(active_label);

    Dashboard::default().with_sidebar(sidebar)
}
