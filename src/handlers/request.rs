use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SignUpReq {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct SignUpResp {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginReq {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResp {
    pub token: String,
}
