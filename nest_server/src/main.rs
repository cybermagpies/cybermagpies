use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

// 1. Define the Report Structure
#[derive(Debug, Deserialize, Serialize)]
struct DeviceReport {
    device_name: String,
    logs_found: String,
    risk_level: String,
}

// 2. The Main Function
#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Define Routes
    let app = Router::new()
        .route("/", get(root))
        .route("/submit_report", post(receive_report));

    // Start Server on Port 3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("The Nest is listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "CyberMagpies Nest is Online."
}

async fn receive_report(Json(payload): Json<DeviceReport>) -> Json<serde_json::Value> {
    println!("\n--> REPORT RECEIVED FROM: {}", payload.device_name);
    println!("    RISK LEVEL: {}", payload.risk_level);
    println!("    LOGS: {}", payload.logs_found);
    
    Json(serde_json::json!({ "status": "Report Filed", "id": 101 }))
}
