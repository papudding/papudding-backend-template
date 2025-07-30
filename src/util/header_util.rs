use crate::model::response::ResponseResult;
use crate::util::token_util;
use actix_web::{HttpRequest, HttpResponse};

pub fn get_header_value<'a>(req: &'a HttpRequest, header_name: &str) -> Option<&'a str> {
    req.headers().get(header_name).and_then(|v| v.to_str().ok())
}

pub fn get_user_id_from_request(req: &HttpRequest) -> Result<u64, HttpResponse> {
    let token =
        get_header_value(req, crate::model::constance::REQUEST_HEAD_TOKEN).ok_or_else(|| {
            HttpResponse::InternalServerError()
                .json(ResponseResult::<bool>::fail("token is empty".to_string()))
        })?;

    let user_id = token_util::get_user_id_by_token(token).map_err(|e| {
        HttpResponse::InternalServerError().json(ResponseResult::<bool>::fail(e.to_string()))
    })?;

    Ok(user_id)
}
