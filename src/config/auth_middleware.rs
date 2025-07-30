use crate::{
    model::{constance::REQUEST_HEAD_TOKEN, state::AppState},
    service::auth_service::verify_token_and_authority,
};
use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    error, web,
};
use futures_util::future::LocalBoxFuture;
use log::{debug, error};
use std::future::{Ready, ready};

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        debug!("auth middleware");

        // 访问路径
        let path = req.path();
        // 从header中获取token
        let token_option = req.headers().get(REQUEST_HEAD_TOKEN);
        // 从请求中获取应用数据
        let app_data_option = req.app_data::<web::Data<AppState>>();

        // 根据应用数据和Token的存在情况进行匹配处理
        match (app_data_option, token_option) {
            // 当应用数据和Token都存在时
            (Some(app_data), Some(token)) => {
                // 获取角色与资源的关系映射
                let role_rel_resource_map = &app_data.role_rel_resource_map;

                // 如果Token为空，则返回未授权错误
                if token.is_empty() {
                    Box::pin(async move { Err(error::ErrorUnauthorized("未授权")) })
                } else {
                    // 解析Token并验证权限
                    match token
                        .to_str()
                        .map_err(|_| error::ErrorBadRequest("无效的Token格式"))
                    {
                        // 如果Token解析成功，则进行权限验证
                        Ok(token_str) => {
                            match verify_token_and_authority(token_str, path, role_rel_resource_map)
                            {
                                // 如果验证成功，则调用下一个服务
                                Ok(_) => {
                                    let fut = self.service.call(req);

                                    Box::pin(async move {
                                        // 返回处理结果
                                        let res = fut.await?;
                                        // 记录日志，表示认证中间件处理完成
                                        debug!("auth middleware done");
                                        Ok(res)
                                    })
                                }
                                // 如果验证失败，则返回禁止访问错误
                                Err(err) => {
                                    error!("权限验证失败: {}", err);
                                    Box::pin(async move { Err(error::ErrorForbidden(err)) })
                                }
                            }
                        }
                        // 如果Token解析失败，则返回错误
                        Err(err) => {
                            error!("Token解析失败: {}", err);
                            Box::pin(async move { Err(err) })
                        }
                    }
                }
            }
            // 当应用数据存在但Token不存在时，返回未授权错误
            (Some(_), None) => {
                error!("未提供Token");
                Box::pin(async move { Err(error::ErrorUnauthorized("未授权")) })
            }
            // 当应用数据不存在时，返回未获取到资源错误
            (None, _) => {
                error!("未获取到资源");
                Box::pin(async move { Err(error::ErrorUnauthorized("未获取到资源")) })
            }
        }
    }
}

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}
