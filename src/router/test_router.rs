use crate::config::auth_middleware::Auth;
use crate::handler::test_handler::{echo, hello, manual_hello, test_db, test_get_user_id};
use actix_web::web::{self, ServiceConfig};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/test")
            .service(echo)
            .service(hello)
            .service(test_db)
            .service(test_get_user_id)
            .route("/hey", web::get().to(manual_hello))
            .wrap(Auth),
    );
}
