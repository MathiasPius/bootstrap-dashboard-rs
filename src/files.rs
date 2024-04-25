//! Embeds the JavaScript and CSS files required to render the
//! dashboard correctly within the [`STATIC_FILES`] constant.
//!
//! If you're using the [axum](https://github.com/tokio-rs/axum) crate,
//! enable the `axum` feature and use [`serve_at`] directly for serving
//! the contained files.
//!
//! Otherwise, please see the [include_dir](https://crates.io/crates/include_dir)
//! crate for information about how to traverse and access this embedded
//! structure directly.

use include_dir::Dir;

/// Embedded directory of static files which must be served from your
/// application in order for the dashboard to appear correctly.
///
/// See [`serve_at`] for an example of how to do this with [axum](https://github.com/tokio-rs/axum)
pub const STATIC_FILES: Dir<'_> = include_dir::include_dir!("$CARGO_MANIFEST_DIR/static");

#[cfg(feature = "axum")]
mod axum_files {
    use axum::{
        body::Body,
        extract::Path,
        http::{header, HeaderValue, Response, StatusCode},
        response::IntoResponse,
        routing::get,
        Router,
    };

    async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
        let path = path.trim_start_matches('/');
        let mime_type = mime_guess::from_path(path).first_or_text_plain();

        match super::STATIC_FILES.get_file(path) {
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap(),
            Some(file) => Response::builder()
                .status(StatusCode::OK)
                .header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str(mime_type.as_ref()).unwrap(),
                )
                .body(Body::from(file.contents()))
                .unwrap(),
        }
    }

    /// Configure a router to serve the embedded dashboard files
    /// from.
    ///
    /// # Example
    /// ```rust
    /// # use axum::{Router, routing::get, response::IntoResponse};
    ///
    /// let app = Router::<()>::new()
    ///     .route("/", get(my_front_page))
    ///     .merge(bootstrap_dashboard::files::serve_at("/static/*path"));
    ///
    /// async fn my_front_page() -> impl IntoResponse {
    ///     "Hello world!"
    /// }
    /// ```
    ///
    /// Note that the path prefix (`/static` in this case) must
    /// match the one provided in the [`Dashboard`](crate::Dashboard) `static_path`
    /// variable since it is used when rendering the relative paths of the
    /// css and js files used in the dashboard.
    pub fn serve_at<S: Clone + Send + Sync + 'static>(path: &str) -> Router<S> {
        Router::new().route(path, get(static_path))
    }
}

#[cfg(feature = "axum")]
pub use axum_files::serve_at;
