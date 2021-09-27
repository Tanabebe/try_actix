use actix_web::{get, HttpResponse, Result, http::header};
use serde::{Deserialize, Serialize};
use actix_cors::Cors;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: usize,
    name: String,
    age: u32,
    personal_color: String,
    hobbies: Vec<String>,
}

#[get("/users")]
async fn index() -> Result<HttpResponse> {
    let user = vec![
        User {
            id: 1,
            name: String::from("主田"),
            age: 24,
            personal_color: String::from("blue"),
            hobbies: vec![String::from(""), String::from("")],
        },
        User {
            id: 2,
            name: String::from("先岡"),
            age: 28,
            personal_color: String::from("pink"),
            hobbies: vec![String::from("")],
        },
        User {
            id: 3,
            name: String::from("後藤"),
            age: 23,
            personal_color: String::from("green"),
            hobbies: vec![String::from("game"), String::from("soccer")],
        },
    ];
    Ok(HttpResponse::Ok().json(user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(index)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}