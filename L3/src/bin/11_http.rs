use std::sync::{Arc, RwLock};
use std::{fmt, fs};

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{extract::Form, extract::Query, Json};
use axum::{routing::get, routing::post, Router};
use chrono::{Datelike, NaiveDate};
use clap::{command, Parser};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber;

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
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////

#[derive(Deserialize)]
pub struct EventQueryParams {
    pub user_id: u64,
    pub date: NaiveDate,
}

#[derive(Deserialize)]
pub struct EventWithIdParams {
    pub user_id: u64,
    pub id: u64,
}

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

pub async fn create_event(
    State(state): State<Arc<dyn EventStore>>,
    Form(new_event): Form<NewEvent>,
) -> Result<Json<HashMap<&'static str, String>>, AppError> {
    state.create_event(new_event).await?;
    Ok(Json(HashMap::from([(
        "result",
        "Event successfully created".to_string(),
    )])))
}

pub async fn update_event(
    State(state): State<Arc<dyn EventStore>>,
    Form(event): Form<Event>,
) -> Result<Json<HashMap<&'static str, String>>, AppError> {
    state.update_event(event).await?;
    Ok(Json(HashMap::from([(
        "result",
        "Event successfully updated".to_string(),
    )])))
}

pub async fn delete_event(
    State(state): State<Arc<dyn EventStore>>,
    Form(params): Form<EventWithIdParams>,
) -> Result<Json<HashMap<&'static str, String>>, AppError> {
    state.delete_event(params.user_id, params.id).await?;
    Ok(Json(HashMap::from([(
        "result",
        "Event successfully deleted".to_string(),
    )])))
}

pub async fn events_for_day(
    Query(params): Query<EventQueryParams>,
    State(state): State<Arc<dyn EventStore>>,
) -> Result<Json<HashMap<&'static str, Vec<Event>>>, AppError> {
    let events_for_month = state
        .get_events_for_day(params.user_id, params.date)
        .await?;
    Ok(Json(HashMap::from([("result", events_for_month)])))
}

pub async fn events_for_week(
    Query(params): Query<EventQueryParams>,
    State(state): State<Arc<dyn EventStore>>,
) -> Result<Json<HashMap<&'static str, Vec<Event>>>, AppError> {
    let events_for_month = state
        .get_events_for_week(params.user_id, params.date)
        .await?;
    Ok(Json(HashMap::from([("result", events_for_month)])))
}

pub async fn events_for_month(
    Query(params): Query<EventQueryParams>,
    State(state): State<Arc<dyn EventStore>>,
) -> Result<Json<HashMap<&'static str, Vec<Event>>>, AppError> {
    let events_for_month = state
        .get_events_for_month(params.user_id, params.date)
        .await?;
    Ok(Json(HashMap::from([("result", events_for_month)])))
}

////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////

#[async_trait::async_trait]
pub trait EventStore: Send + Sync + 'static {
    async fn create_event(&self, event: NewEvent) -> Result<Event, AppError>;
    async fn update_event(&self, event: Event) -> Result<(), AppError>;
    async fn delete_event(&self, user_id: u64, event_id: u64) -> Result<(), AppError>;
    async fn get_events_for_day(
        &self,
        user_id: u64,
        date: NaiveDate,
    ) -> Result<Vec<Event>, AppError>;
    async fn get_events_for_week(
        &self,
        user_id: u64,
        date: NaiveDate,
    ) -> Result<Vec<Event>, AppError>;
    async fn get_events_for_month(
        &self,
        user_id: u64,
        date: NaiveDate,
    ) -> Result<Vec<Event>, AppError>;
}

// Ideally to use something more elaborate to utilze the data range search
// For example use Postgres with BTreeIndex on dates columns
pub struct InMemoryEventStore {
    events: RwLock<HashMap<u64, Event>>,
    next_id: RwLock<u64>,
}

impl InMemoryEventStore {
    pub fn new() -> Self {
        Self {
            events: RwLock::new(HashMap::new()),
            next_id: RwLock::new(1),
        }
    }
}

#[async_trait::async_trait]
impl EventStore for InMemoryEventStore {
    async fn create_event(&self, new_event: NewEvent) -> Result<Event, AppError> {
        let mut events = self.events.write().map_err(|_| {
            AppError::InternalServerError("Unable to take write lock on events".to_string())
        })?;
        let mut next_id = self.next_id.write().map_err(|_| {
            AppError::InternalServerError("Unable to take write lock on next_id".to_string())
        })?;

        let event = Event {
            id: *next_id,
            user_id: new_event.user_id,
            title: new_event.title,
            description: new_event.description,
            date: new_event.date,
        };

        events.insert(*next_id, event.clone());
        *next_id += 1;

        Ok(event)
    }

    async fn update_event(&self, updated_event: Event) -> Result<(), AppError> {
        let mut events = self.events.write().map_err(|_| {
            AppError::InternalServerError("Unable to take write lock on events".to_string())
        })?;
        if events.contains_key(&updated_event.id) {
            events.insert(updated_event.id, updated_event);
            Ok(())
        } else {
            Err(AppError::BusinessLogicError("Event not found".to_string()))
        }
    }

    async fn delete_event(&self, user_id: u64, event_id: u64) -> Result<(), AppError> {
        let mut events = self.events.write().map_err(|_| {
            AppError::InternalServerError("Unable to take write lock on events".to_string())
        })?;
        if let Some(event) = events.get(&event_id) {
            if event.user_id == user_id {
                events.remove(&event_id);
                Ok(())
            } else {
                Err(AppError::BusinessLogicError(
                    "Unauthorized to delete this event".to_string(),
                ))
            }
        } else {
            Err(AppError::BusinessLogicError("Event not found".to_string()))
        }
    }

    async fn get_events_for_day(
        &self,
        user_id: u64,
        date: NaiveDate,
    ) -> Result<Vec<Event>, AppError> {
        let events = self.events.read().map_err(|_| {
            AppError::InternalServerError("Unable to take write read on events".to_string())
        })?;
        let events_for_day = events
            .values()
            .filter(|event| event.user_id == user_id && event.date == date)
            .cloned()
            .collect();
        Ok(events_for_day)
    }

    async fn get_events_for_week(
        &self,
        user_id: u64,
        date: NaiveDate,
    ) -> Result<Vec<Event>, AppError> {
        let events = self.events.read().map_err(|_| {
            AppError::InternalServerError("Unable to take read lock on events".to_string())
        })?;
        let week = date.iso_week();

        let events_for_week = events
            .values()
            .filter(|event| {
                event.user_id == user_id
                    && event.date.iso_week().year() == week.year()
                    && event.date.iso_week().week() == week.week()
            })
            .cloned()
            .collect();
        Ok(events_for_week)
    }

    async fn get_events_for_month(
        &self,
        user_id: u64,
        date: NaiveDate,
    ) -> Result<Vec<Event>, AppError> {
        let events = self.events.read().map_err(|_| {
            AppError::InternalServerError("Unable to take read lock on events".to_string())
        })?;
        let events_for_month = events
            .values()
            .filter(|event| {
                event.user_id == user_id
                    && event.date.year() == date.year()
                    && event.date.month() == date.month()
            })
            .cloned()
            .collect();
        Ok(events_for_month)
    }
}

////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////

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

    let store = Arc::new(InMemoryEventStore::new());

    init_tracing();

    let app = Router::new()
        .route("/create_event", post(create_event))
        .route("/update_event", post(update_event))
        .route("/delete_event", post(delete_event))
        .route("/events_for_day", get(events_for_day))
        .route("/events_for_week", get(events_for_week))
        .route("/events_for_month", get(events_for_month))
        .with_state(store)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}", port = config.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
