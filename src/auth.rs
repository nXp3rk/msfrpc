use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct AuthLoginReq {
    pub method: String,
    pub user: String,
    pub pass: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthLoginRes {
    pub result: String,
    pub token: String,
}

#[derive(Serialize, Debug)]
pub struct AuthLogoutReq {
    pub method: String,
    pub token: String,
    pub logout_token: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthLogoutRes {
    pub result: String,
}

#[derive(Serialize, Debug)]
pub struct AuthTokenAddReq {
    pub method: String,
    pub token: String,
    pub new_token: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthTokenAddRes {
    pub result: String,
}

#[derive(Serialize, Debug)]
pub struct AuthTokenGenerateReq {
    pub result: String,
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthTokenGenerateRes {
    pub result: String,
}

#[derive(Serialize, Debug)]
pub struct AuthTokenListReq {
    pub result: String,
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthTokenListRes {
    pub tokens: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct AuthTokenRemoveReq {
    pub result: String,
    pub token: String,
    pub token_remove: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthTokenRemoveRes {
    pub result: String,
}
