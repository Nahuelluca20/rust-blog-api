#[macro_use]
extern crate diesel;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub mod models;
pub mod schema;

use dotenvy::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

#[get("/")]
async fn hello_word() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&db_url).expect("Error connecting to db");

    HttpServer::new(|| App::new().service(hello_word))
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run()
        .await
}
