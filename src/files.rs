use include_dir::Dir;

pub static STATIC_FILES: Dir<'_> = include_dir::include_dir!("$CARGO_MANIFEST_DIR/static");

#[cfg(feature = "axum")]
mod axum_files {
    use axum::{
        body::{self, Empty, Full},
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
                .body(body::boxed(Empty::new()))
                .unwrap(),
            Some(file) => Response::builder()
                .status(StatusCode::OK)
                .header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str(mime_type.as_ref()).unwrap(),
                )
                .body(body::boxed(Full::from(file.contents())))
                .unwrap(),
        }
    }

    /// Configure a router to serve the embedded dashboard files
    /// from.
    ///
    /// # Example
    /// ```rust,norun
    /// let app = Router::new()
    ///     .route("/", get(my_front_page))
    ///     .merge(dashboard::files::serve_at("/static/*path"));
    /// ```
    pub fn serve_at(path: &'static str) -> Router {
        Router::new().route(path, get(static_path))
    }
}

#[cfg(feature = "axum")]
pub use axum_files::serve_at;
