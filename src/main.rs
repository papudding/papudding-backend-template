mod config;
mod dto;
mod handler;
mod model;
mod router;
mod service;
mod util;

use actix_web::{middleware::Logger, web, App, HttpServer};
use log::{debug, warn};
use model::{config::Setting, state::AppState};
use router::{auth_inner_router, auth_router, test_router};
use service::auth_service::load_all_role_rel_resource;
use handler::default_handler::not_found;
use std::sync::Arc;
use util::{config_util::CFG, db_util};
use config::logger::init_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志系统，默认日志级别为 debug
    init_logger();

    // 获取数据库连接池实例，并处理可能的错误
    let db_pool = match db_util::get_db_instance().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to get database instance: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Database connection failed",
            ));
        }
    };

    // 加载角色资源映射，并处理可能的错误
    let role_rel_resource_map = Arc::new(load_all_role_rel_resource(&db_pool).await);

    // 获取配置文件中的设置
    let setting = match &CFG.setting {
        Some(setting) => setting,
        None => {
            warn!("配置文件中未设置setting，使用默认设置");
            &Setting::default()
        }
    };

    // 验证配置文件中的主机和端口
    if setting.host.is_empty() || setting.port == 0 {
        eprintln!("配置文件中的主机或端口无效");
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "无效的主机或端口",
        ));
    }

    // 打印实际使用的地址和端口
    debug!("Starting server on {}:{}", setting.host, setting.port);

    // 启动 HTTP 服务器
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState {
                db_pool: db_pool.clone(),
                role_rel_resource_map: role_rel_resource_map.clone(),
            }))
            .configure(auth_router::routes)
            .configure(test_router::routes)
            .configure(auth_inner_router::routes)
            // 设置默认服务处理未匹配的路由 
            .default_service ( web :: route ( ) . to ( not_found ) )
    })
    .bind((setting.host.as_str(), setting.port))?
    .run()
    .await
}