#[macro_use]
extern crate diesel;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub mod models;
pub mod schema;

use dotenvy::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

use self::models::Post;
use self::schema::posts;
use self::schema::posts::dsl::*;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn index(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    match web::block(move || posts.load::<Post>(&mut conn)).await {
        Ok(data) => {
            println!("{:?}", data);
            HttpResponse::Ok().body("Get data")
        }
        Err(err) => HttpResponse::Ok().body("Error"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = ConnectionManager::<PgConnection>::new(db_url);

    let pool = Pool::builder()
        .max_size(5)
        .build(connection)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .service(index)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
