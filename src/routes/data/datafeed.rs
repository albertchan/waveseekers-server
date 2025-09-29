use std::fs;

use actix_web::{HttpResponse, Responder};

pub async fn get_datafeed_config() -> impl Responder {
    match fs::read_to_string("data/datafeed.json") {
        Ok(val) => {
            let json = serde_json::from_str::<serde_json::Value>(&val);
            if json.is_err() {
                return HttpResponse::InternalServerError()
                    .body("Failed to parse JSON from datafeed.json");
            }
            HttpResponse::Ok().json(json.unwrap())
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to read file: {}", err))
        }
    }
}
