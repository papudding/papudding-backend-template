use crate::{
    config::auth_middleware::Auth,
    handler::auth_inner_handler::{
        get_current_loaded_role_rel_resource, reflush_all_role_rel_resource,
    },
};
use actix_web::web::{self, ServiceConfig};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/auth_inner")
            .service(reflush_all_role_rel_resource)
            .service(get_current_loaded_role_rel_resource)
            .wrap(Auth),
    );
}
