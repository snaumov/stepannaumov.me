mod blog;

use std::path::Path;

use axum::{
    extract::State,
    http::StatusCode,
    response::Html,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use notify::{recommended_watcher, RecursiveMode, Watcher};
use std::process::Command;
use std::sync::{Arc, Mutex, MutexGuard};
use tera::{Context, Tera};
use tower_http::services::ServeDir;

static ASSETS_DIR: &str = "./assets";

// Define your application shared state
#[derive(Clone)]
struct AppState {
    engine: Arc<Mutex<Tera>>,
}

impl AppState {
    pub fn get_engine(&self) -> MutexGuard<'_, Tera> {
        // TODO make it conditional on env var
        self.engine.lock().unwrap()
    }
}

fn setup_tera() -> Tera {
    let mut tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    tera.autoescape_on(vec![]);
    tera
}

#[tokio::main]
async fn main() {
    let tera = Arc::new(Mutex::new(setup_tera()));
    let tera_clone = tera.clone();
    // TODO enable this only in dev mode
    let mut watcher = recommended_watcher(move |res| match res {
        Ok(_) => {
            Command::new("bash")
                .arg("-c")
                .arg("just build-tailwind")
                .output()
                .expect("failed to rebuild tailwind");
            tera_clone.lock().unwrap().full_reload().unwrap();
        }
        Err(e) => println!("watch error: {:?}", e),
    })
    .expect("Cannot create watcher");
    watcher
        .watch(Path::new("./templates/"), RecursiveMode::Recursive)
        .unwrap();

    let app_state = AppState { engine: tera };

    let app = Router::new()
        .route("/", get(home))
        .nest_service("/assets", ServeDir::new(ASSETS_DIR))
        // .merge(blog::routes(app_state.clone()))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let tera = state.get_engine();
    Ok(Html(tera.render("index.html", &Context::new())?))
}

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
