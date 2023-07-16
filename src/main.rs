#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use dotenvy::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

fn main() {
    dotenv().ok();

    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut connection = PgConnection::establish(&db_url).expect("Error connecting to db");

    use self::models::{NewPost, Post};
    use self::schema::posts;
    use self::schema::posts::dsl::*;

    let new_post = NewPost {
        title: "secont post",
        body: "this sis a second post",
        slug: "second-post",
    };

    let post: Post = diesel::insert_into(posts::table)
        .values(new_post)
        .get_result(&mut connection)
        .expect("The insertion not found");

    // Select * from posts
    let posts_result = posts
        .load::<Post>(&mut connection)
        .expect("Error loading posts");

    // Select * from posts
    let posts_result_limit = posts.limit(1)
    .load::<Post>(&mut connection)
    .expect("Error loading posts");

    for post in posts_result {
        println!("{}", post.title);
    }

    for post in posts_result_limit {
        println!("post limit 1: {}", post.title);
    }


}
