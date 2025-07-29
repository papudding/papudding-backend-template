use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct LoginDto {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RegisterDto {
    pub username: String,
    pub password: String,
    pub recheck_password: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoleRelPathDto {
    pub role_name: String,
    pub resource_value: String,
}
