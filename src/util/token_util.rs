use crate::util::config_util::CFG;
use jsonwebtoken::Algorithm;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub user_id: u64,
    pub username: String,
    pub exp: usize,
    pub roles: Vec<String>,
}

// 生成一个 JWT token
pub fn generate_jwt_token(
    user_id: u64,
    username: &str,
    expiration_time: usize,
    roles: &Vec<String>,
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = TokenClaims {
        user_id: user_id,
        username: username.to_string(),
        exp: expiration_time,
        roles: roles.clone(),
    };
    let header = Header::new(Algorithm::HS512);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(CFG.jwt.token_secret.as_ref()),
    )
}

// 验证 JWT token
pub fn validate_jwt_token(token: &str) -> Result<TokenClaims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(CFG.jwt.token_secret.as_ref());
    // Algorithm::HS256 指定算法，根据实际使用的算法进行调整
    let mut validation = Validation::new(Algorithm::HS512);
    // 启用过期时间验证
    validation.validate_exp = true;

    let result = decode::<TokenClaims>(token, &decoding_key, &validation)?;
    Ok(result.claims)
}

// 从token中获取用户ID
pub fn get_user_id_by_token(token: &str) -> Result<u64, jsonwebtoken::errors::Error> {
    let claims = validate_jwt_token(token)?;
    Ok(claims.user_id)
}
