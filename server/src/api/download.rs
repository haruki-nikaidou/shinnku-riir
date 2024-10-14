use actix_web::{get, web, Responder};
use uuid::Uuid;
use crate::api::AppState;
use crate::drivers::DownloadProviderStateTrait;
use crate::file_list::FileDownloadSources;
use crate::hot::HotRecord;

#[get("/api/download/{id}")]
pub async fn download_handler(
    data: web::Data<AppState>,
    game_id: web::Path<Uuid>,
    req: actix_web::HttpRequest
) -> impl Responder {
    let game_id = game_id.into_inner();
    let ip = req.connection_info().realip_remote_addr().unwrap_or("unknown").to_string();
    let source = match data.download_provider.query_by_uuid(game_id).await {
        Ok(Some(v)) => v,
        Ok(None) => {
            return actix_web::HttpResponse::NotFound().finish();
        }
        Err(e) => {
            tracing::error!("Failed to get download source: {:?}", e);
            return actix_web::HttpResponse::InternalServerError().finish();
        }
    };
    match HotRecord::insert(HotRecord {
        game_id,
        ip
    }, &data.pg).await {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("Failed to insert hot record: {:?}", e);
        }
    }
    let link = data.download_provider.get_download_link(&source).await;
    todo!()
}