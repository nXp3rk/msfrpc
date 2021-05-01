use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct ConsoleCreateReq {
	pub method: String,
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct ConsoleCreateRes {
    pub id: i64,
	pub prompt: String,
    pub busy: bool
}

#[derive(Serialize, Debug)]
pub struct ConsoleDestroyReq {
	pub method: String,
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct ConsoleDestroyRes {
    pub result: String
}

#[derive(Serialize, Debug)]
pub struct ConsoleListReq {
	pub method: String,
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct ConsoleListRes {
    pub consoles: HashMap<i32,ConsoleCreateRes>
}

#[derive(Serialize, Debug)]
pub struct ConsoleWriteReq {
	pub method: String,
    pub token: String,
    pub console_id: String,
    pub data: String
}

#[derive(Deserialize, Debug)]
pub struct ConsoleWriteRes {
    pub wrote: i64
}

#[derive(Serialize, Debug)]
pub struct ConsoleReadReq {
	pub method: String,
    pub token: String,
    pub console_id: String,
}

#[derive(Deserialize, Debug)]
pub struct ConsoleReadRes {
    pub data: String,
    pub prompt: String,
    pub busy: bool
}

#[derive(Serialize, Debug)]
pub struct ConsoleSessionDetachReq {
	pub method: String,
    pub token: String,
    pub console_id: String,
}

#[derive(Deserialize, Debug)]
pub struct ConsoleSessionDetachRes {
    pub result: String
}

#[derive(Serialize, Debug)]
pub struct ConsoleSessionKillReq {
	pub method: String,
    pub token: String,
    pub console_id: String,
}

#[derive(Deserialize, Debug)]
pub struct ConsoleSessionKillRes {
    pub result: String
}

#[derive(Serialize, Debug)]
pub struct ConsoleTabsReq {
	pub method: String,
    pub token: String,
    pub console_id: String,
    pub input_line: String
}

#[derive(Deserialize, Debug)]
pub struct ConsoleTabsRes {
    pub tabs: Vec<String>
}
