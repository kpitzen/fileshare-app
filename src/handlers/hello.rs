use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::fmt::Debug;

use crate::models;

#[derive(Serialize, Deserialize, Debug)]
pub struct HelloRequest {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HelloResponse {
    text: String,
}

pub async fn manual_hello(body: Json<HelloRequest>) -> HttpResponse {
    println!("Processing request body: {:?}", body.text);
    HttpResponse::Ok()
        .content_type("application/json")
        .json(HelloResponse {
            text: format!("Hey there: {text}", text = body.text),
        })
}

pub async fn manual_hello_with_db(body: Json<HelloRequest>, db_pool: Data<PgPool>) -> HttpResponse {
    println!("Processing request body: {:?}", body.text);
    let files = models::files::File::get_all(&db_pool).await.unwrap();
    println!("Here's some files: {:?}", files);
    HttpResponse::Ok()
        .content_type("application/json")
        .json(HelloResponse {
            text: format!("Hey there: {text}", text = body.text),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use http;

    #[actix_rt::test]
    async fn test_manual_hello() {
        let test_request_body = HelloRequest {
            text: String::from("test"),
        };
        let resp = manual_hello(Json(test_request_body)).await;
        println!("{:?}", resp);
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_post_hello() {
        let mut app =
            test::init_service(App::new().route("/hey", web::post().to(manual_hello))).await;
        let test_request_body = HelloRequest {
            text: String::from("test"),
        };

        let sample_request = test::TestRequest::post()
            .uri("/hey")
            .set_json(&test_request_body)
            .to_request();
        let resp: HelloResponse = test::read_response_json(&mut app, sample_request).await;

        assert_eq!(resp.text, "Hey there: test");
    }
}
