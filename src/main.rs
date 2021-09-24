use actix_web::{get, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: usize,
    name: String,
    age: u32,
    personal_color: String,
}

#[get("/users")]
async fn index() -> Result<HttpResponse> {
    let user = vec![
        User {
            id: 1,
            name: String::from("主田"),
            age: 24,
            personal_color: String::from("blue"),
        },
        User {
            id: 2,
            name: String::from("先岡"),
            age: 28,
            personal_color: String::from("pink"),
        },
        User {
            id: 3,
            name: String::from("後藤"),
            age: 23,
            personal_color: String::from("green"),
        },
    ];
    Ok(HttpResponse::Ok().json(user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}