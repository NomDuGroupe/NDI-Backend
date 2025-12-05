use axum::{Router, extract::State, routing::get};
use tokio::{net::TcpListener, sync::Mutex};
use serde::{Serialize, Deserialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
struct Session {
	cookie_user: String,
	username: String,
	port: u16,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct DaemonState {
	running_sessions: Vec<Session>,
}

#[tokio::main]
async fn main() {
	let state = Arc::new(Mutex::new(DaemonState::default()));

	let app = Router::new().route("/", get(recieve_client_request));

	let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
	axum::serve(listener, app).await.unwrap();
}

async fn recieve_client_request(
	State(state): State<Arc<Mutex<DaemonState>>>
) -> &'static str {
	//let mut state = state.lock().await;
	"hello"
}