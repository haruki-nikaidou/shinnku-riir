use actix_web::{HttpResponse, Responder};
use tracing::error;
use crate::hot::{HotRecord, TopCountItem};

type HotTopResponse = Vec<TopCountItem>;

#[actix_web::get("/api/hot")]
async fn hot_top_handler(data: actix_web::web::Data<super::AppState>) -> impl Responder {
    match HotRecord::get_top10(&data.pg).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => {
            error!("Failed to get top 10 hot games: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}