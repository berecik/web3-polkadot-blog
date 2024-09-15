// main.rs

use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// Data structures for request and response
#[derive(Deserialize)]
struct InputData {
    value: u32,
}

#[derive(Serialize)]
struct OutputData {
    message: String,
}

// Shared application state
struct AppState {
    counter: Mutex<u32>,
}

// Handler for the root route
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Rust Actix Web Backend!")
}

// Handler to get the current counter value
async fn get_counter(data: web::Data<AppState>) -> impl Responder {
    let counter = data.counter.lock().unwrap();
    let response = OutputData {
        message: format!("Current counter value: {}", *counter),
    };
    HttpResponse::Ok().json(response)
}

// Handler to increment the counter
async fn increment_counter(
    data: web::Data<AppState>,
    json: web::Json<InputData>,
) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += json.value;
    let response = OutputData {
        message: format!("Counter incremented by {}", json.value),
    };
    HttpResponse::Ok().json(response)
}

// Main function to start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Server address and port
    let server_address = "127.0.0.1:8080";

    println!("Starting server at http://{}", server_address);

    // Initialize shared state
    let app_state = web::Data::new(AppState {
        counter: Mutex::new(0),
    });

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default()) // Enable logging
            .app_data(app_state.clone()) // Add shared state
            // Configure routes
            .route("/", web::get().to(index))
            .route("/counter", web::get().to(get_counter))
            .route("/counter/increment", web::post().to(increment_counter))
    })
    .bind(server_address)?
    .run()
    .await
}

