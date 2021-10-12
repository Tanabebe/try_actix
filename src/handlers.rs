use super::models::{Post, NewPost};
use super::schema::posts::dsl::*;
use super::Pool;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{insert_into};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputPost {
    pub title: String,
    pub body: String,
}

pub async fn add_post(
    db: web::Data<Pool>,
    item: web::Json<InputPost>,
) -> Result<HttpResponse, Error> {
    // let mut title = String::new();
    //stdin().read_line(&mut title).unwrap();
    // let title = &title[..(title.len() - 1)]; // Drop the newline character
    //println!("\nOk! Let's write {} (Press {} when finished)\n", title, EOF);
    // let mut body = String::new();
    //stdin().read_to_string(&mut body).unwrap();

    // let post = create_post(&connection, title, &body);
    // println!("\nSaved draft {}", title);

    Ok(web::block(move || add_single_post(db, item))
        .await
        .map(|post| HttpResponse::Created().json(post))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn add_single_post(
    db: web::Data<Pool>,
    item: web::Json<InputPost>,
) -> Result<Post, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_post = NewPost {
        title: &item.title,
        body: &item.body,
    };
    let res = insert_into(posts).values(&new_post).get_result(&conn)?;
    Ok(res)
}