use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
struct HeartbeatResponse {
    status: &'static str,
}

pub async fn heartbeat() -> HttpResponse {
    HttpResponse::Ok().json(HeartbeatResponse { status: "OK" })
}
