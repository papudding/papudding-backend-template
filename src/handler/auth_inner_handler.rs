use crate::{
    dto::auth_dto::RoleRelPathDto,
    model::{response::ResponseResult, state::AppState},
    service::auth_service::load_all_role_rel_resource,
};
use actix_web::{get, web, HttpResponse, Responder, Result};
use dashmap::DashMap;
use std::{collections::HashMap, sync::Arc};

/// 异步重新加载所有角色关联资源信息
///
/// 本函数旨在从数据库中重新获取所有角色的关联资源信息，并更新应用状态中的缓存数据
/// 它首先从应用状态中获取数据库连接池和角色关联资源的共享引用，然后通过调用数据库操作
/// 加载所有角色的关联资源信息之后，它会清空当前应用状态中的缓存数据，并用新加载的数据进行填充
/// 最后，它会构建一个包含所有角色关联资源信息的HashMap，并作为HTTP响应返回
///
/// # 参数
/// * `app_state` - 包含数据库连接池和共享状态的Web应用状态
///
/// # 返回值
/// 返回一个实现了Responder trait的类型，表示HTTP响应，其中包含重新加载的角色关联资源信息
#[get("/reflush_role")]
pub async fn reflush_all_role_rel_resource(
    app_state: web::Data<AppState>,
) -> Result<impl Responder> {
    // 获取数据库连接池引用
    let rb = &app_state.db_pool;
    // 克隆角色关联资源的共享引用
    let role_rel_resource_map = Arc::clone(&app_state.role_rel_resource_map);
    // 异步加载所有角色的关联资源信息
    let map = load_all_role_rel_resource(rb).await;

    // 清空当前的角色关联资源缓存
    role_rel_resource_map.clear();
    // 遍历新加载的角色关联资源信息，并更新缓存
    for (role, vec) in map {
        role_rel_resource_map.insert(role, vec);
    }

    // 构建一个包含所有角色关联资源信息的HashMap，用于返回
    let response_map = convert_to_hash_map(role_rel_resource_map);

    // 返回HTTP响应，其中包含重新加载的角色关联资源信息
    Ok(HttpResponse::Ok().json(
        ResponseResult::<HashMap<String, Vec<RoleRelPathDto>>>::success_with_data(response_map),
    ))
}

/// 异步获取当前加载的角色关联资源信息
///
/// 该函数通过克隆存储在应用状态中的角色关联资源映射，并将其转换为HashMap格式返回。
/// 主要用于需要获取角色与资源关系信息的场景，以便进行权限控制或其他操作。
///
/// # 参数
///
/// * `app_state` - 应用状态的共享引用，其中包含角色关联资源的映射信息
///
/// # 返回
///
/// 返回一个HTTP响应，其中包含角色关联资源的信息。信息以JSON格式呈现，
/// 并使用`HttpResponse::Ok()`表示请求成功。
#[get("/get_current_role_rel_res")]
pub async fn get_current_loaded_role_rel_resource(
    app_state: web::Data<AppState>,
) -> Result<impl Responder> {
    // 克隆角色关联资源的共享引用
    let role_rel_resource_map = Arc::clone(&app_state.role_rel_resource_map);
    // 构建一个包含所有角色关联资源信息的HashMap，用于返回
    let response_map = convert_to_hash_map(role_rel_resource_map);
    // 返回HTTP响应，其中包含重新加载的角色关联资源信息
    Ok(HttpResponse::Ok().json(
        ResponseResult::<HashMap<String, Vec<RoleRelPathDto>>>::success_with_data(response_map),
    ))
}

fn convert_to_hash_map(
    map: Arc<DashMap<String, Vec<RoleRelPathDto>>>,
) -> HashMap<String, Vec<RoleRelPathDto>> {
    map.iter()
        .map(|r| (r.key().clone(), r.value().clone()))
        .collect()
}
