use actix_web::{HttpResponse, Responder};
use serde_json::json ;
// 自定义404处理函数
pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().json(json!({
        "code": 404,
        "msg": "Not Found",
        "data": None::<String>,
    }))
}