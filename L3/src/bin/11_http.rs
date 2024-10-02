use std::{fmt, fs};

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{extract::Form, extract::Query, Json};
use axum::{routing::get, routing::post, Router};
use chrono::NaiveDate;
use clap::{command, Parser};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub id: u64,
    pub user_id: u64,
    pub title: String,
    pub description: String,
    pub date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEvent {
    pub user_id: u64,
    pub title: String,
    pub description: String,
    pub date: NaiveDate,
}

#[derive(Debug)]
pub enum AppError {
    ValidationError(String),
    BusinessLogicError(String),
    InternalServerError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::ValidationError(message) => (
                StatusCode::BAD_REQUEST,
                axum::Json(json!({ "error": message })),
            )
                .into_response(),
            AppError::BusinessLogicError(message) => (
                StatusCode::SERVICE_UNAVAILABLE,
                axum::Json(json!({ "error": message })),
            )
                .into_response(),
            AppError::InternalServerError(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(json!({ "error": message })),
            )
                .into_response(),
        }
    }
}

#[derive(Deserialize)]
pub struct EventQueryParams {
    pub user_id: u64,
    pub date: NaiveDate,
}

pub async fn create_event(
    Form(new_event): Form<NewEvent>,
) -> Result<Json<HashMap<&'static str, String>>, AppError> {
    // Some business logic
    Ok(Json(HashMap::from([(
        "result",
        "Event created successfully".to_string(),
    )])))
}

pub async fn update_event(
    Form(event): Form<Event>,
) -> Result<Json<HashMap<&'static str, String>>, AppError> {
    // Some business logic
    Ok(Json(HashMap::from([(
        "result",
        "Event updated successfully".to_string(),
    )])))
}

pub async fn delete_event(
    Form(params): Form<EventQueryParams>,
) -> Result<Json<HashMap<&'static str, String>>, AppError> {
    // Some business logic
    Ok(Json(HashMap::from([(
        "result",
        "Event deleted successfully".to_string(),
    )])))
}

pub async fn events_for_day(
    Query(params): Query<EventQueryParams>,
) -> Result<Json<HashMap<&'static str, String>>, AppError> {
    // Some business logic
    Ok(Json(HashMap::from([(
        "result",
        "Events fetched for the day".to_string(),
    )])))
}

pub async fn events_for_week(
    Query(params): Query<EventQueryParams>,
) -> Result<Json<HashMap<&'static str, String>>, AppError> {
    // Some business logic
    Ok(Json(HashMap::from([(
        "result",
        "Events fetched for the week".to_string(),
    )])))
}

pub async fn events_for_month(
    Query(params): Query<EventQueryParams>,
) -> Result<Json<HashMap<&'static str, String>>, AppError> {
    // Some business logic
    Ok(Json(HashMap::from([(
        "result",
        "Events fetched for the month".to_string(),
    )])))
}

pub fn init_tracing() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
}

#[derive(Parser, Debug)]
#[command(
    name = "Calendar App",
    about = "An HTTP server for calendar events management."
)]
struct Cli {
    /// Путь к файлу конфигурации
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn from_file(path: &str) -> Self {
        let content = fs::read_to_string(path).expect("Failed to read config file");
        toml::from_str(&content).expect("Failed to parse config")
    }
}
#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let config = Config::from_file(&cli.config);

    init_tracing();

    let app = Router::new()
        .route("/create_event", post(create_event))
        .route("/update_event", post(update_event))
        .route("/delete_event", post(delete_event))
        .route("/events_for_day", get(events_for_day))
        .route("/events_for_week", get(events_for_week))
        .route("/events_for_month", get(events_for_month))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}", port = config.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
