use crate::dto::auth_dto::{LoginDto, RegisterDto};
use crate::model::user::User;
use crate::model::{response::ResponseResult, state::AppState};
use crate::service::auth_service;
use actix_web::{HttpResponse, Responder, Result, post, web};
use log::{error, info};

/// 异步处理用户登录请求
///
/// 该函数接收用户登录信息，验证用户凭据，并在验证成功后生成并返回用户令牌。
///
/// # 参数
///
/// - `app_state`: 应用状态的共享数据，包含数据库连接池等信息
/// - `login_dto`: 包含用户登录信息的数据传输对象
///
/// # 返回
///
/// - 登录成功时返回包含用户令牌的HttpResponse
/// - 登录失败或令牌生成失败时返回相应的错误信息
#[post("/login")]
pub async fn login(
    app_state: web::Data<AppState>,
    login_dto: web::Json<LoginDto>,
) -> Result<impl Responder, actix_web::Error> {
    // 获取数据库连接池
    let db = &app_state.db_pool;

    // 调用认证服务验证用户登录信息
    match auth_service::verify_login(db, &login_dto.username, &login_dto.password).await {
        // 验证成功，获取用户ID
        Ok(user_id) => {
            // 生成用户令牌
            match auth_service::gen_token(db, user_id, &login_dto.username).await {
                // 令牌生成成功，返回用户令牌
                Ok(token) => {
                    info!("User {} logged in successfully", login_dto.username);
                    Ok(HttpResponse::Ok().json(ResponseResult::<String>::success_with_data(token)))
                }
                // 令牌生成失败，记录错误信息并返回错误响应
                Err(e) => {
                    error!(
                        "Failed to generate token for user {}: {}",
                        login_dto.username, e
                    );
                    Ok(HttpResponse::InternalServerError()
                        .json(ResponseResult::<bool>::fail("token 颁发失败".to_string())))
                }
            }
        }
        // 验证失败，记录错误信息并返回错误响应
        Err(e) => {
            error!(
                "Failed to verify login for user {}: {}",
                login_dto.username, e
            );
            Ok(HttpResponse::Unauthorized()
                .json(ResponseResult::<bool>::fail("用户名或密码错误".to_string())))
        }
    }
}

/// 注册新用户
///
/// 该函数处理用户注册请求，使用提供的用户信息DTO进行数据库操作以创建新用户
///
/// # 参数
/// * `app_state`: 应用程序状态的共享数据，包含数据库连接池等信息
/// * `register_dto`: 包含用户注册信息的数据传输对象
///
/// # 返回
/// * `Result<impl Responder>`: 返回一个HTTP响应，包含注册结果或错误信息
#[post("/register")]
pub async fn register(
    app_state: web::Data<AppState>,
    register_dto: web::Json<RegisterDto>,
) -> Result<impl Responder> {
    // 调用用户注册服务，传入数据库连接池和用户注册信息
    match auth_service::user_register(&app_state.db_pool, register_dto.into_inner()).await {
        // 如果注册成功，返回包含用户信息的HTTP 200响应
        Ok(user) => {
            info!("User registered successfully: {:?}", user);
            Ok(HttpResponse::Ok().json(ResponseResult::<User>::success_with_data(user)))
        }
        // 如果注册失败，返回包含错误信息的HTTP 500响应
        Err(err) => {
            Ok(HttpResponse::InternalServerError().json(ResponseResult::<String>::fail(err)))
        }
    }
}
