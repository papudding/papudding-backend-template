use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ResponseResult<T> {
    code: i32,
    msg: String,
    data: Option<T>,
}

const MSG_SUCCESS: &str = "success";

#[allow(dead_code)]
impl<T> ResponseResult<T> {
    pub fn success(data_opt: Option<T>) -> Self {
        Self {
            code: 0,
            msg: MSG_SUCCESS.to_string(),
            data: data_opt,
        }
    }

    pub fn success_without_data() -> Self {
        Self::success(Option::None)
    }

    pub fn success_with_data(data: T) -> Self {
        Self {
            code: 0,
            msg: MSG_SUCCESS.to_string(),
            data: Option::Some(data),
        }
    }

    pub fn fail(msg: String) -> Self {
        Self {
            code: -1,
            msg,
            data: None,
        }
    }
}
