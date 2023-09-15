use eyre::Result;

use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

use std::net::SocketAddr;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // dotenv().ok();
    // let rust_log = std::env::var("RUST_LOG").expect("RUST_LOG missing");
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or("debug".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    let app = Router::new()
        .route("/*path", get(frontend::static_path))
        .route("/", get(frontend::index))
        .route("/index_template.html", get(template::index_template))
        // .layer(Extension(state))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

mod frontend {

    use axum::{
        body::{self, Empty, Full},
        extract::Path,
        http::StatusCode,
        http::{header, HeaderValue},
        response::{IntoResponse, Response},
    };

    use include_dir::{include_dir, Dir};

    static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");

    pub(crate) async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
        let path = path.trim_start_matches('/');
        let mime_type = mime_guess::from_path(path).first_or_text_plain();
        tracing::info!("path {:?}", &path);
        tracing::info!("mime type {:?}", &mime_type);
        let x = STATIC_DIR.get_file(path);

        match x {
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

    pub(crate) async fn index() -> impl IntoResponse {
        "Hello from a function"
    }
}

mod template {
    use askama::Template;
    use axum::{
        http::StatusCode,
        response::{Html, IntoResponse, Response},
    };

    #[derive(Template)]
    #[template(path = "index_template.html")]
    pub(crate) struct IndexTemplate {
        hello: String,
    }

    pub(crate) async fn index_template(// Extension(state): Extension<Arc<AppState>>,
    ) -> impl IntoResponse {
        tracing::info!("index_template");
        let template = IndexTemplate {
            hello: "Hello".to_string(),
        };
        HtmlTemplate(template)
    }

    pub(crate) struct HtmlTemplate<T>(pub(crate) T);

    impl<T> IntoResponse for HtmlTemplate<T>
    where
        T: Template,
    {
        fn into_response(self) -> Response {
            match self.0.render() {
                Ok(html) => Html(html).into_response(),
                Err(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to render template. Error: {}", err),
                )
                    .into_response(),
            }
        }
    }
}
