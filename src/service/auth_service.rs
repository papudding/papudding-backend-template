use crate::dto::auth_dto::{RegisterDto, RoleRelPathDto};
use crate::model::user::{select_all_role_rel_path, select_roles_by_user_id};
use crate::model::{constance::DEFAULT_BCRYPT_COST, user::User};
use crate::util::{
    config_util::CFG,
    token_util::{generate_jwt_token, validate_jwt_token},
};
use bcrypt;
use chrono::{TimeDelta, Utc};
use dashmap::DashMap;
use log::{debug, error, info, warn};
use rbatis::RBatis;
use regex::Regex;
use std::collections::HashMap;

/// 异步验证用户登录
///
/// 该函数通过用户名和密码验证用户身份，并返回用户ID
/// 它首先根据用户名查询用户信息，然后使用bcrypt算法验证密码
///
/// 参数:
/// - `rb`: &RBatis - 数据库连接实例引用，用于执行数据库操作
/// - `username`: &str - 用户名字符串引用，用于查询用户信息
/// - `password`: &str - 密码字符串引用，用于验证用户密码
///
/// 返回:
/// - `Result<u64, String>` - 如果登录成功，返回用户ID（u64）；
///   如果登录失败，返回错误信息（String）
pub async fn verify_login(rb: &RBatis, username: &str, password: &str) -> Result<u64, String> {
    // 根据用户名查询用户信息
    match User::select_by_username(rb, username.to_string()).await {
        Ok(user_option) => {
            if let Some(user) = user_option {
                // 验证用户密码
                match bcrypt::verify(password, user.password.as_deref().unwrap_or("")) {
                    Ok(valid) => {
                        if valid {
                            // 如果密码正确，返回用户ID
                            user.id.ok_or_else(|| "用户ID为空".to_string())
                        } else {
                            // 如果密码不正确，返回错误信息
                            Err("密码错误".to_string())
                        }
                    }
                    Err(_) => {
                        // 如果密码验证失败，返回错误信息
                        Err("密码验证失败".to_string())
                    }
                }
            } else {
                // 如果用户不存在，返回错误信息
                Err("用户不存在".to_string())
            }
        }
        Err(err) => {
            // 如果查询用户信息失败，返回错误信息
            Err(format!("查询用户失败：{}", err.to_string()))
        }
    }
}

/// 生成JWT token
///
/// # Arguments
///
/// * `rb` - RBatis实例，用于数据库操作
/// * `user_id` - 用户ID
/// * `username` - 用户名
///
/// # Returns
///
/// * `Ok(String)` - 生成的JWT token字符串
/// * `Err(String)` - 生成token失败的原因
pub async fn gen_token(rb: &RBatis, user_id: u64, username: &str) -> Result<String, String> {
    // 计算token的过期时间
    let expiration_time = Utc::now()
        .checked_add_signed(TimeDelta::hours(CFG.jwt.expiration_time))
        .ok_or("时间计算失败")?
        .timestamp() as usize;

    // 获取角色信息
    match select_roles_by_user_id(rb, user_id).await {
        Ok(roles) => {
            // 提取角色名称，过滤掉没有角色名称的项
            let role_name_vec = roles
                .iter()
                .filter_map(|item| item.role_name.clone())
                .collect();

            // 日志记录生成的token信息
            debug!("gen_token: {:?}", role_name_vec);
            // 生成JWT token
            match generate_jwt_token(user_id, username, expiration_time, &role_name_vec) {
                Ok(token) => Ok(token),
                Err(err) => Err(format!("token生成失败：{}", err.to_string())),
            }
        }
        Err(err) => {
            // 日志记录获取角色信息失败
            debug!("gen_token_err: {:?}", err);
            Err(format!("token生成失败，获取角色失败"))
        }
    }
}

/// 用户注册函数
///
/// 本函数负责处理用户注册流程，包括验证用户输入、检查用户是否存在、密码加密和保存用户信息
///
/// 参数:
/// - rb: &RBatis - 数据库连接实例
/// - register_dto: RegisterDto - 包含用户注册信息的数据传输对象
///
/// 返回:
/// - Result<User, String> - 注册成功返回用户对象，失败返回错误信息字符串
pub async fn user_register(rb: &RBatis, register_dto: RegisterDto) -> Result<User, String> {
    // 1. 验证输入
    if register_dto.password != register_dto.recheck_password {
        return Err("两次输入密码不一致".to_string());
    }
    if register_dto.username.is_empty() || register_dto.name.is_empty() {
        return Err("用户名或姓名不能为空".to_string());
    }

    // 2. 检查用户是否已存在
    match User::select_by_username(rb, register_dto.username.clone()).await {
        Ok(Some(_)) => {
            warn!("用户名已存在");
            return Err("用户名已存在".to_string());
        }
        Ok(None) => {} // 用户名可用
        Err(err) => {
            error!("查询用户失败: {}", err);
            return Err(format!("查询用户失败: {}", err));
        }
    }

    // 3. 加密密码
    let encrypted_password = match bcrypt::hash(register_dto.password, DEFAULT_BCRYPT_COST) {
        Ok(hash) => hash,
        Err(err) => return Err(format!("加密密码失败: {}", err)),
    };

    // 4. 保存用户信息
    let mut user = User::init_create(register_dto.username, encrypted_password, register_dto.name);

    match User::insert(rb, &user).await {
        Ok(res) => {
            if res.rows_affected == 0 {
                Err("保存用户失败: 数据写入失败".to_string())
            } else {
                user.id = Some(res.last_insert_id.as_u64().unwrap());
                Ok(user)
            }
        }
        Err(err) => Err(format!("用户保存失败: {}", err)),
    }
}

/// 验证用户令牌和权限
///
/// 该函数首先验证JWT令牌的有效性，然后检查令牌对应的用户是否具有访问指定资源的权限
/// 它用于确保用户不仅拥有有效的身份验证令牌，而且还被授权访问特定的资源或执行特定的操作
///
/// # 参数
///
/// * `token`: &str - 用户的JWT令牌
/// * `path`: &str - 用户试图访问的资源路径
/// * `role_rel_resource_map`: &DashMap<String, Vec<RoleRelPathDto>> - 角色与资源路径关系的映射
///
/// # 返回值
///
/// * `Result<bool, String>` - 如果用户拥有有效的令牌和适当的权限，则返回Ok(true)，否则返回错误信息
pub fn verify_token_and_authority(
    token: &str,
    path: &str,
    role_rel_resource_map: &DashMap<String, Vec<RoleRelPathDto>>,
) -> Result<bool, String> {
    // 验证JWT令牌
    let validate_result = validate_jwt_token(token);
    match validate_result {
        Ok(claims) => {
            // 日志记录验证结果和角色资源映射信息
            debug!(
                "auth_service==>verify_token_and_authority=>claims:{:?}, role_rel_resource_map:{:?}",
                claims, role_rel_resource_map
            );
            // 获取令牌的过期时间
            let expiration_time = claims.exp;
            // 获取当前时间戳
            let current_time = Utc::now().timestamp() as usize;
            // 检查令牌是否已过期
            if current_time > expiration_time {
                Err("token已过期".to_string())
            } else {
                // 验证资源
                let roles = claims.roles;

                // 检查用户是否具有访问资源的权限
                let has_permission =
                    check_resource_permission(&roles, path, role_rel_resource_map)?;
                if has_permission {
                    Ok(true)
                } else {
                    Err("权限验证失败".to_string())
                }
            }
        }
        Err(err) => Err(format!("token验证失败: {}", err)),
    }
}

// 检查用户角色是否具有访问指定资源路径的权限
fn check_resource_permission(
    roles: &[String],                                             // 用户角色列表
    path: &str,                                                   // 请求的资源路径
    role_rel_resource_map: &DashMap<String, Vec<RoleRelPathDto>>, // 角色与资源路径的关系映射
) -> Result<bool, String> {
    // 创建一个哈希图用于缓存正则表达式，以提高匹配效率
    let mut regex_cache = HashMap::new();
    // 遍历用户的所有角色
    for role in roles {
        // 根据角色获取对应的资源路径列表
        if let Some(role_rel_resource_vec) = role_rel_resource_map.get(role) {
            // 遍历资源路径列表
            for rel_path_dto in role_rel_resource_vec.iter() {
                // 获取资源的匹配值（正则表达式字符串）
                let resource_value = &rel_path_dto.resource_value;
                // 在缓存中查找或插入正则表达式
                let reg = regex_cache
                    .entry(resource_value.clone())
                    .or_insert_with(|| {
                        // 尝试编译正则表达式，如果失败则返回错误信息
                        Regex::new(resource_value)
                            .map_err(|e| format!("无效的正则表达式: {}", e))
                            .unwrap()
                    });
                // 如果正则表达式匹配请求路径，则表示用户有访问权限
                if reg.is_match(path) {
                    return Ok(true);
                }
            }
        }
    }
    // 如果没有找到匹配的资源路径，表示用户没有访问权限
    Ok(false)
}

/// 异步加载所有角色关联资源信息
///
/// 本函数通过数据库操作获取所有角色关联资源信息，并将其组织成哈希表形式，便于根据角色名快速查询关联资源
///
/// # 参数
/// * `rb`: &RBatis - 数据库连接实例引用，用于执行数据库操作
///
/// # 返回
/// 返回一个哈希表，其中键为角色名，值为与该角色关联的资源路径列表如果数据库中没有找到任何角色关联信息，将返回一个空的哈希表
///
/// # 错误处理
/// 如果数据库操作失败，将打印错误日志，并返回一个空的哈希表根据业务需求，也可以选择返回错误或执行其他错误处理策略
pub async fn load_all_role_rel_resource(rb: &RBatis) -> DashMap<String, Vec<RoleRelPathDto>> {
    // 尝试从数据库中选择所有角色关联资源路径信息
    match select_all_role_rel_path(rb).await {
        Ok(role_rel_path) => {
            // 如果查询结果为空，则记录日志并返回一个空的哈希表
            if role_rel_path.is_empty() {
                info!("未找到角色资源相关数据");
                return DashMap::new();
            }

            // 初始化一个空的哈希表来存储角色名与关联资源路径的映射
            let map: DashMap<String, Vec<RoleRelPathDto>> = DashMap::new();
            // 遍历查询结果，将每个角色关联的资源路径添加到哈希表中对应的键下
            for item in role_rel_path {
                let key = item.role_name.clone();
                map.entry(key).or_default().push(item);
            }
            // 返回填充好的哈希表
            map
        }
        Err(e) => {
            // 如果数据库操作失败，记录错误日志，并根据业务需求决定是否返回空哈希表或传播错误
            error!("角色资源关联数据加载失败: {}", e);
            DashMap::new()
        }
    }
}
