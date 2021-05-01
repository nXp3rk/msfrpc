use std::collections::HashMap;
use serde::{self,Deserialize, Serialize, Deserializer};
use chrono::{DateTime,Utc,TimeZone};

#[derive(Serialize, Debug)]
pub struct JobListReq {
	pub method: String,
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct JobListRes {
    pub jobs: Vec<HashMap<i64,String>>
}

#[derive(Serialize, Debug)]
pub struct JobInfoReq {
	pub method: String,
    pub token: String,
    pub job_id: String
}

#[derive(Deserialize,Debug)]
pub enum JobType {
    Bool(bool),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Str(String),
}

#[derive(Deserialize, Debug)]
pub struct JobInfoRes {
    pub job_id: i64,
    pub name: String,
    #[serde(deserialize_with = "date_format")]
    pub start_time: DateTime<Utc>,
    pub uri_path: String,
    pub datastore: HashMap<String,JobType>,
}

fn date_format<'de, D>(deserializer: D) -> Result<DateTime<Utc>,D::Error> where D: Deserializer<'de> {
    let datefmt = "%Y-%m-%d %H:%M:%S";
    let s = String::deserialize(deserializer)?;
    Utc.datetime_from_str(&s, datefmt).map_err(serde::de::Error::custom)
}

#[derive(Serialize, Debug)]
pub struct JobStopReq {
	pub method: String,
    pub token: String,
    pub job_id: String
}

#[derive(Deserialize, Debug)]
pub struct JobStopRes {
    pub result: String,
}
