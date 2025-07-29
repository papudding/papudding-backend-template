use crate::{
    model::{constance::REQUEST_HEAD_TOKEN, response::ResponseResult, state::AppState},
    util::token_util::validate_jwt_token,
};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder, Result};

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/test_db")]
async fn test_db(app_state: web::Data<AppState>) -> impl Responder {
    let rb = &app_state.db_pool;
    let count: u64 = rb
        .query_decode("select count(1) from t_dic", vec![])
        .await
        .unwrap();
    HttpResponse::Ok().body(format!("count is {}", count))
}

#[get("/user_id")]
async fn test_get_user_id(req: HttpRequest) -> Result<impl Responder> {
    if let Some(token) = req.headers().get(REQUEST_HEAD_TOKEN) {
        let user_id = validate_jwt_token(token.to_str().unwrap()).unwrap().user_id;
        Ok(HttpResponse::Ok().json(ResponseResult::<u64>::success_with_data(user_id)))
    } else {
        Ok(HttpResponse::InternalServerError()
            .json(ResponseResult::<String>::fail("获取token失败".to_string())))
    }
}
