#[macro_use]
extern crate diesel;

use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use actix_cors::Cors;
use actix_web::http::header;

mod handlers;
mod models;
mod schema;


// #[derive(Serialize, Deserialize, Debug)]
// struct User {
//     id: usize,
//     name: String,
//     age: u32,
//     personal_color: String,
//     hobbies: Vec<String>,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// struct NameLists {
//     id: usize,
//     firstname: String,
//     lastname: String,
//     age: u32,
// }
//
// #[get("/name/lists")]
// async fn name_lists() -> Result<HttpResponse, E> {
//     let result = vec![
//         NameLists {
//             id: 1,
//             firstname: String::from("勉"),
//             lastname: String::from("主田"),
//             age: 24,
//         },
//         NameLists {
//             id: 2,
//             firstname: String::from("未来"),
//             lastname: String::from("先岡"),
//             age: 28,
//         },
//         NameLists {
//             id: 3,
//             firstname: String::from("一郎"),
//             lastname: String::from("後藤"),
//             age: 23,
//         },
//     ];
//     Ok(HttpResponse::Ok().json(result))
// }
//
// #[get("/users")]
// async fn index() -> Result<HttpResponse, E> {
//     let result = vec![
//         User {
//             id: 1,
//             name: String::from("主田"),
//             age: 24,
//             personal_color: String::from("blue"),
//             hobbies: vec![],
//         },
//         User {
//             id: 2,
//             name: String::from("先岡"),
//             age: 28,
//             personal_color: String::from("pink"),
//             hobbies: vec![String::from("cooking")],
//         },
//         User {
//             id: 3,
//             name: String::from("後藤"),
//             age: 23,
//             personal_color: String::from("green"),
//             hobbies: vec![String::from("game"), String::from("soccer")],
//         },
//     ];
//     Ok(HttpResponse::Ok().json(result))
// }

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    use actix_web::{App, HttpServer};
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .data(pool.clone())
            .wrap(cors)
            // .service(index)
            // .service(name_lists)
            .route("/post", web::post().to(handlers::add_post))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}