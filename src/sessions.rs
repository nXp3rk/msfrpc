use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct SessionListReq {
	pub method: String,
	pub token: String
}

#[derive(Deserialize, Debug)]
pub struct SessionInfoRes {
    pub stype: String,
    pub tunnel_local: String,
    pub tunnel_peer: String,
    pub via_exploit: String,
    pub via_payload: String,
    pub description: String,
    pub info: String,
    pub workspace: String,
    pub session_host: String,
    pub session_port: i64,
    pub username: String,
    pub uuid: String,
    pub exploit_uuid: String
}

#[derive(Deserialize, Debug)]
pub struct SessionListRes {
	pub sessions: HashMap<i64,SessionInfoRes>,
}

#[derive(Serialize, Debug)]
pub struct SessionStopReq {
	pub method: String,
    pub token: String,
    pub session_id: String
}

#[derive(Deserialize, Debug)]
pub struct SessionStopRes {
	pub result: String
}

#[derive(Serialize, Debug)]
pub struct SessionCompatibleModulesReq {
	pub method: String,
    pub token: String,
    pub session_id: String
}

#[derive(Deserialize, Debug)]
pub struct SessionCompatibleModulesRes {
	pub modules: Vec<String>
}

#[derive(Serialize, Debug)]
pub struct SessionShellReadReq {
	pub method: String,
    pub token: String,
    pub session_id: String
}

#[derive(Deserialize, Debug)]
pub struct SessionShellReadRes {
    pub seq: i64,
    pub data: String
}

#[derive(Serialize, Debug)]
pub struct SessionShellWriteReq {
	pub method: String,
    pub token: String,
    pub session_id: String,
    pub data: String
}

#[derive(Deserialize, Debug)]
pub struct SessionShellWriteRes {
    pub write_count: i64
}

#[derive(Serialize, Debug)]
pub struct SessionShellUpgradeReq {
	pub method: String,
    pub token: String,
    pub session_id: String,
    pub data: String,
    pub conn_host: String,
    pub conn_port: i64
}

#[derive(Deserialize, Debug)]
pub struct SessionShellUpgradeRes {
    pub result: String
}

#[derive(Serialize, Debug)]
pub struct SessionMeterpreterReadReq {
	pub method: String,
    pub token: String,
    pub session_id: String
}

#[derive(Deserialize, Debug)]
pub struct SessionMeterpreterReadRes {
    pub data: String
}

#[derive(Serialize, Debug)]
pub struct SessionMeterpreterWriteReq {
	pub method: String,
    pub token: String,
    pub session_id: String,
    pub data: String
}

#[derive(Deserialize, Debug)]
pub struct SessionMeterpreterWriteRes {
    pub result: String
}

#[derive(Serialize, Debug)]
pub struct SessionMeterpreterTabsReq {
	pub method: String,
    pub token: String,
    pub session_id: String,
    pub input_line: String
}

#[derive(Deserialize, Debug)]
pub struct SessionMeterpreterTabsRes {
    pub tabs: Vec<String>
}

#[derive(Serialize, Debug)]
pub struct SessionMeterpreterRunSingleReq {
	pub method: String,
    pub token: String,
    pub session_id: String,
    pub data: String
}

#[derive(Deserialize, Debug)]
pub struct SessionMeterpreterRunSingleRes {
    pub result: String
}

#[derive(Serialize, Debug)]
pub struct SessionMeterpreterScriptReq {
	pub method: String,
    pub token: String,
    pub session_id: String,
    pub script_name: String
}

#[derive(Deserialize, Debug)]
pub struct SessionMeterpreterScriptRes {
    pub result: String
}

#[derive(Serialize, Debug)]
pub struct SessionMeterpreterSessionDetachReq {
	pub method: String,
    pub token: String,
    pub session_id: String,
}

#[derive(Deserialize, Debug)]
pub struct SessionMeterpreterSessionDetachRes {
    pub result: String
}

#[derive(Serialize, Debug)]
pub struct SessionMeterpreterSessionKillReq {
	pub method: String,
    pub token: String,
    pub session_id: String,
}

#[derive(Deserialize, Debug)]
pub struct SessionMeterpreterSessionKillRes {
    pub result: String
}

#[derive(Serialize, Debug)]
pub struct SessionRingClearReq {
	pub method: String,
    pub token: String,
    pub session_id: String,
}

#[derive(Deserialize, Debug)]
pub struct SessionRingClearRes {
    pub result: String
}

#[derive(Serialize, Debug)]
pub struct SessionRingLastReq {
    pub method: String,
    pub token: String,
    pub session_id: String,
}

#[derive(Deserialize, Debug)]
pub struct SessionRingLastRes {
    pub seq: i64
}

#[derive(Serialize, Debug)]
pub struct SessionRingPutReq {
	pub method: String,
    pub token: String,
    pub session_id: String,
}

#[derive(Deserialize, Debug)]
pub struct SessionRingPutRes {
    pub write_count: i64
}
