use rbatis::{self, impl_select, rbdc::DateTime, sql, RBatis};
use serde::{Deserialize, Serialize};

use crate::dto::auth_dto::RoleRelPathDto;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<u64>,
    pub create_time: Option<DateTime>,
    pub creator_id: Option<u64>,
    pub update_time: Option<DateTime>,
    pub updater_id: Option<u64>,
    pub is_delete: Option<u8>,
    pub del_unique_key: Option<i8>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
}
impl User {
    pub fn init_create(username: String, password: String, name: String) -> Self {
        User {
            id: None,
            create_time: Some(DateTime::now()),
            creator_id: Some(0),
            update_time: None,
            updater_id: None,
            is_delete: Some(0),
            del_unique_key: Some(0),
            username: Some(username),
            password: Some(password),
            name: Some(name),
        }
    }
}

rbatis::crud!(User {}, "sys_user");
impl_select!(User{select_by_username(username:String) -> Option => "`where username = #{username} limit 1`"}, "sys_user");

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: Option<u64>,
    pub create_time: Option<DateTime>,
    pub creator_id: Option<u64>,
    pub update_time: Option<DateTime>,
    pub updater_id: Option<u64>,
    pub is_delete: Option<u8>,
    pub del_unique_key: Option<i8>,
    pub role_name: Option<String>,
    pub role_desc: Option<String>,
}
rbatis::crud!(Role {}, "sys_role");

#[sql(
    "select sr.*
    from sys_user_rel_role surr 
    join sys_role sr on sr.id = surr.role_id and sr.is_delete = 0
    where surr.is_delete = 0
      and surr.user_id = ?"
)]
pub async fn select_roles_by_user_id(rb: &RBatis, user_id: u64) -> Vec<Role> {}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRelRole {
    pub id: Option<u64>,
    pub create_time: Option<DateTime>,
    pub creator_id: Option<u64>,
    pub update_time: Option<DateTime>,
    pub updater_id: Option<u64>,
    pub is_delete: Option<u8>,
    pub del_unique_key: Option<i8>,
    pub user_id: Option<u64>,
    pub role_id: Option<u64>,
}
rbatis::crud!(UserRelRole {}, "sys_user_rel_role");

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    pub id: Option<u64>,
    pub create_time: Option<DateTime>,
    pub creator_id: Option<u64>,
    pub update_time: Option<DateTime>,
    pub updater_id: Option<u64>,
    pub is_delete: Option<u8>,
    pub del_unique_key: Option<i8>,
    pub resource_value: Option<String>,
    pub resource_desc: Option<String>,
    pub resource_type: Option<u8>, // 1 path 2 page 3 button
    pub parent_id: Option<u64>,
}
rbatis::crud!(Resource {}, "sys_resource");

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleRelResource {
    pub id: Option<u64>,
    pub create_time: Option<DateTime>,
    pub creator_id: Option<u64>,
    pub update_time: Option<DateTime>,
    pub updater_id: Option<u64>,
    pub is_delete: Option<u8>,
    pub del_unique_key: Option<i8>,
    pub role_id: Option<u64>,
    pub resource_id: Option<u64>,
}
rbatis::crud!(RoleRelResource {}, "sys_role_rel_resource");

#[sql(
    "select sr.role_name, sre.resource_value
from sys_role sr
         join sys_role_rel_resource srrr on sr.id = srrr.role_id and srrr.is_delete = 0
         join sys_resource sre on sre.id = srrr.resourcec_id and sre.is_delete = 0
where sr.is_delete = 0 and sre.resource_type = 1"
)]
pub async fn select_all_role_rel_path(rb: &RBatis) -> Vec<RoleRelPathDto> {}
