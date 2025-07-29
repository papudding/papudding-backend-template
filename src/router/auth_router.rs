use crate::handler::auth_handler::{login, register};
use actix_web::web::{self, ServiceConfig};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/auth").service(login).service(register));
}
