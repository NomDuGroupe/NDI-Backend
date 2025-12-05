use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::{net::TcpListener, sync::Mutex};
use uuid::Uuid;

use crate::error::BackendError;

mod error;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Session {
    session_id: String,
    port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct AvailablePort {
    port: u16,
    is_available: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct DaemonState {
    running_sessions: Vec<Session>,
    available_ports: Vec<AvailablePort>,
}

impl Default for DaemonState {
    fn default() -> Self {
        DaemonState {
            running_sessions: Vec::new(),
            available_ports: {
                let mut vector = Vec::<AvailablePort>::with_capacity(10);
                (3500..3510).for_each(|x| {
                    vector.push(AvailablePort {
                        port: x,
                        is_available: true,
                    })
                });
                vector
            },
        }
    }
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(DaemonState::default()));

    let app = Router::new()
        .route("/", get(index))
        .route("/connect", get(connect_client))
        .route("/gen_session", post(create_session))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Result<String, BackendError> {
    tokio::fs::read_to_string("index.html")
        .await
        .map_err(|_| BackendError::InternalError)
}

async fn connect_client(
    State(state): State<Arc<Mutex<DaemonState>>>,
    jar: CookieJar,
) -> impl IntoResponse {
    if let Some(s_id) = jar.get("session_id") {
        let mut state = state.lock().await;
        //TODO implem server connect
        (StatusCode::OK, "").into_response()
    } else {
        Redirect::to("/gen_session").into_response()
    }
}

async fn create_session(
    State(state): State<Arc<Mutex<DaemonState>>>,
    jar: CookieJar,
) -> Result<(CookieJar, Redirect), BackendError> {
    let session_id = Uuid::new_v4();
    let mut state = state.lock().await;
    if let Some(port_state) = state
        .available_ports
        .iter()
        .copied()
        .find(|p| p.is_available)
    {
        //TODO: Create session on server !!
        state.running_sessions.push(Session {
            session_id: session_id.to_string(),
            port: port_state.port,
        });
        return Ok((
            jar.add(Cookie::new("session_id", session_id.to_string())),
            Redirect::to("/connect"),
        ));
    }
    Err(BackendError::NoSlotsAvailable)
}
