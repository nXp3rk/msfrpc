use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct ModuleExploitsReq{
	pub method: String,
	pub token: String,
}

#[derive(Serialize, Debug)]
pub struct ModulePayloadsReq {
	pub method: String,
	pub token: String,
}

#[derive(Serialize, Debug)]
pub struct ModuleAuxiliaryReq {
	pub method: String,
	pub token: String,
}

#[derive(Serialize, Debug)]
pub struct ModuleEncodersReq {
	pub method: String,
	pub token: String,
}

#[derive(Serialize, Debug)]
pub struct ModulePostReq {
	pub method: String,
	pub token: String,
}

#[derive(Serialize, Debug)]
pub struct ModuleNopsReq {
	pub method: String,
	pub token: String,
}

#[derive(Serialize, Debug)]
pub struct ModuleInfoReq {
	pub method: String,
	pub token: String,
	pub modtype: String,
	pub modname: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Reference {
  StrInt(String,i64),
  StrStr(String,String)
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Action {
  StrInt(i64,String),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ModuleInfoRes {
	ModuleExploitInfoRes {
		r#type: String,
		name: String,
		fullname: String,
		rank: String,
		disclosuredate: String,
		description: String,
		license: String,
		filepath: String,
		arch: Vec<String>,
		platform: Vec<String>,
		authors: Vec<String>,
		privileged: bool,
		references: Vec<Reference>,
		targets: HashMap<i64, String>,
		default_target: i64,
		stance: String,
	},
	ModuleAuxiliaryInfoRes {
		r#type: String,
		name: String,
		fullname: String,
		rank: String,
		disclosuredate: String,
		description: String,
		license: String,
		filepath: String,
		arch: Vec<String>,
		platform: Vec<String>,
		authors: Vec<String>,
		privileged: bool,
		references: Vec<Reference>,
		stance: String,
	},
	ModuleNopsInfoRes {
		r#type: String,
		name: String,
		fullname: String,
		rank: String,
		disclosuredate: String,
		description: String,
		license: String,
		filepath: String,
		arch: Vec<String>,
		platform: Vec<String>,
		authors: Vec<String>,
		privileged: bool,
		references: Vec<Reference>,
	},
    ModulePostInfoRes {
   		r#type: String,
		name: String,
		fullname: String,
		rank: String,
		disclosuredate: String,
		description: String,
		license: String,
		filepath: String,
		arch: Vec<String>,
		platform: Vec<String>,
		authors: Vec<String>,
		privileged: bool,
		references: Vec<Reference>,
        actions: Vec<Action>,
        default_action: String,
    }
}

#[derive(Serialize, Debug)]
pub struct ModuleOptionsReq {
	pub method: String,
	pub token: String,
	pub modtype: String,
	pub modname: String,
}

pub type ModuleOptionsRes = HashMap<String,ModuleOptionRet>;

#[derive(Serialize, Debug)]
pub struct ModuleCompatiblePayloadsReq {
	pub method: String,
	pub token: String,
	pub modname: String,
}

#[derive(Deserialize, Debug)]
pub struct ModuleCompatiblePayloadsRes {
	pub payloads: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct ModuleTargetCompatiblePayloadsReq {
	pub method: String,
	pub token: String,
	pub modname: String,
    pub tgt: String,
}

#[derive(Deserialize, Debug)]
pub struct ModuleTargetCompatiblePayloadsRes {
	pub payloads: Vec<String>,
}


#[derive(Serialize, Debug)]
pub struct ModuleCompatibleSessionsReq {
	pub method: String,
	pub token: String,
	pub modname: String,
}

#[derive(Deserialize, Debug)]
pub struct ModuleCompatibleSessionsRes {
	pub sessions: Vec<i64>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ModuleOptionRet {
  DefaultBool { r#type: String, required: bool, advanced: bool, desc: String, default: bool },
  DefaultInt { r#type: String, required: bool, advanced: bool, desc: String, default: u32 },
  DefaultEnum { r#type: String, required: bool, advanced: bool, desc: String, default: String, enums: Vec<String> },
  NoDefault { r#type: String, required: bool, advanced: bool, desc: String },
}

#[derive(Serialize, Debug)]
pub struct ModuleExecuteReq {
	pub method: String,
	pub token: String,
	pub modtype: String,
	pub modname: String,
	pub options: HashMap<String,String>,
}

#[derive(Deserialize, Debug)]
pub struct ModuleExecuteRes {
	pub jobid: i64,
	pub uuid: String,
}
