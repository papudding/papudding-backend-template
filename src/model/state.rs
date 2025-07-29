use crate::dto::auth_dto::RoleRelPathDto;
use dashmap::DashMap;
use rbatis::RBatis;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db_pool: RBatis,
    pub role_rel_resource_map: Arc<DashMap<String, Vec<RoleRelPathDto>>>,
}
