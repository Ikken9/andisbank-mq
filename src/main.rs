use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::fmt::Display;
use log::info;
use crate::models::loan::Loan;

mod models;

// Shared message queue
struct AppState<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
}

impl<T> AppState<T> {
    fn new() -> Self {
        AppState {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
}

async fn get_message<T: Serialize + Display + Clone>(state: web::Data<AppState<T>>) -> impl Responder {
    let mut queue = state.queue.lock().unwrap();
    if let Some(msg) = queue.pop_front() {
        info!("{}", msg.clone());
        HttpResponse::Ok().json(msg)
    } else {
        info!("No messages in queue");
        HttpResponse::NotFound().body("No messages in queue")
    }
}

async fn add_message<T>(state: web::Data<AppState<T>>, msg: web::Json<T>) -> impl Responder {
    let mut queue = state.queue.lock().unwrap();
    queue.push_back(msg.into_inner());
    HttpResponse::Ok().body("Message added")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState::<Loan>::new());

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/loans/send", web::post().to(add_message::<Loan>))
            .route("/loans/receive", web::get().to(get_message::<Loan>))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}