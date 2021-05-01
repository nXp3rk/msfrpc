pub mod utils;
pub mod auth;
pub mod jobs;
pub mod sessions;
pub mod console;
pub mod modules;

pub mod client {
    use std::io::Read;
    use std::collections::HashMap;

    use serde::Serialize;
    use serde::de::DeserializeOwned;
    use rmp_serde::Serializer;
    use crate::auth::*;
    use crate::modules::*;
    use crate::console::*;
    use crate::jobs::*;

    pub struct Msfrpc {
        host: String,
        port: i64,
        ssl: bool,
        token: String,
        uri: String,
        user: String,
        pass: String,
    }

    impl Msfrpc {
        pub fn new(host: &str,port: i64,user: &str, pass: &str, ssl: bool) -> Self {
            return Msfrpc {
                uri: "/api/".to_string(),
                host: host.to_string(),
                port,
                ssl,
                token: "".to_string(),
                user: user.to_string(),
                pass: pass.to_string(),
            };
        }

        fn send(&mut self,buf: Vec<u8>) -> Result<Vec<u8>,String> {
            let mut out_buf = Vec::new();
            let scheme = if self.ssl { "https" } else { "http" };
            let url = format!("{}://{}:{}{}",scheme,self.host,self.port,self.uri);
            let res = ureq::post(&url).set("Content-Type", "binary/message-pack").send_bytes(&buf);
            match res.status() {
                200 => {
                    let mut reader = res.into_reader();
                    let _ = reader.read_to_end(&mut out_buf);
                    return Ok(out_buf)
                },
                500 => { return Err(format!("[-] Bad Status Code : {}",res.status())) },
                _ => { return Err(format!("[-] Bad Status Code : {}",res.status())) },
            }
        }

        fn commuicate<T,R>(&mut self,s: T) -> Result<R,String> where T: Serialize, R: DeserializeOwned {
            let mut buf = Vec::new();
            s.serialize(&mut Serializer::new(&mut buf)).unwrap();
            let out = self.send(buf);
            match out {
                Ok(res) => {
                    let data: R = rmp_serde::from_read_ref(&res).unwrap();
                    Ok(data)
                },
                Err(msg) => Err(msg),
            }
        }

        pub fn login(&mut self) {
            let login = AuthLoginReq{ method: "auth.login".into(), user: self.user.clone(), pass: self.pass.clone() };
            let login_result = self.commuicate::<AuthLoginReq,AuthLoginRes>(login);
            match login_result {
                Ok(res) => {
                    if &res.result == "success" {
                        self.token = res.token.clone();
                        println!("[+] Authentication Successful.");
                    }
                },
                Err(msg) => {
                    println!("[-] Unknown Error: {:?}",msg);
                }
            }
        }

        pub fn logout(&mut self) {
            let logout = AuthLogoutReq{ method: "auth.logout".into(), token: self.token.clone(), logout_token: self.token.clone() };
            let logout_result = self.commuicate::<AuthLogoutReq,AuthLogoutRes>(logout);
            match logout_result {
                Ok(res) => {
                    if &res.result == "success" {
                        println!("[+] Logout Successful."); 
                    }
                },
                Err(msg) => {
                    println!("[-] Unknown Error: {:?}",msg);
                }
            }
        }

        fn module_into<T>(&mut self,modules: T) -> Result<Vec<String>,String> where T: Serialize {
            let result = self.commuicate::<T,HashMap<String,Vec<String>>>(modules);
            match result {
                Ok(res) => Ok(res["modules"].clone()),
                Err(msg) => Err(format!("Unknown Error: {:?}",msg)),
            }
        }

        pub fn modules(&mut self, mtype: &'static str) -> Result<Vec<String>,String> {
            match mtype {
                "exploits" => {
                    let exploits = ModuleExploitsReq { method: "module.exploits".into(), token: self.token.clone() };
                    return self.module_into(exploits);
                },
                "posts" => {
                    let post = ModulePostReq { method: "module.post".into(), token: self.token.clone() };
                    return self.module_into(post);
                },
                "auxiliary" => {
                    let auxiliary = ModuleAuxiliaryReq { method: "module.auxiliary".into(), token: self.token.clone() };
                    return self.module_into(auxiliary);                    
                },
                "payloads" => {
                    let payloads = ModulePayloadsReq { method: "module.payloads".into(), token: self.token.clone() };
                    return self.module_into(payloads);
                },
                "encoders" => {
                    let encoders = ModuleEncodersReq { method: "module.encoders".into(), token: self.token.clone() };
                    return self.module_into(encoders);    
                },
                "nops" => {
                    let nops = ModuleNopsReq { method: "module.nops".into(), token: self.token.clone() };
                    return self.module_into(nops);    
                },
                _ => return Err(format!("not match module type: {}",mtype)),
            }
        }

        pub fn info(&mut self,mtype: String, module: String) -> Result<ModuleInfoRes,String> {
            let info = ModuleInfoReq { method: "module.info".into(), token: self.token.clone(), modtype: mtype, modname: module };
            self.commuicate::<ModuleInfoReq,ModuleInfoRes>(info)
        }

        pub fn options(&mut self, mtype: String, module: String) -> Result<ModuleOptionsRes,String> {
            let options = ModuleOptionsReq { method: "module.options".into(), token: self.token.clone(), modtype: mtype, modname: module };
            self.commuicate::<ModuleOptionsReq,ModuleOptionsRes>(options)
        }

        pub fn console_create(&mut self) -> Result<ConsoleCreateRes,String> {
            let create = ConsoleCreateReq { method: "console.create".into(), token: self.token.clone() };
            self.commuicate::<ConsoleCreateReq,ConsoleCreateRes>(create)
        }

        pub fn console_read(&mut self,cid: i64) -> Result<ConsoleReadRes,String> {
            let c_id = if cid < 0 { 0 } else { cid };
            let read = ConsoleReadReq { 
                method: "console.read".into(), 
                token: self.token.clone(), 
                console_id: c_id.to_string()
            };
            self.commuicate::<ConsoleReadReq,ConsoleReadRes>(read)
        }

        pub fn console_write(&mut self,cid: i64, cmd: String) -> Result<ConsoleWriteRes,String> {
            let c_id = if cid < 0 { 0 } else { cid };
            let write = ConsoleWriteReq {
                method: "console.write".into(),
                token: self.token.clone(),
                console_id: c_id.to_string(),
                data: cmd
            };
            self.commuicate::<ConsoleWriteReq,ConsoleWriteRes>(write)
        }

        pub fn compatible_payloads(&mut self, m_name: String) -> Result<ModuleCompatiblePayloadsRes,String> {
            let payloads = ModuleCompatiblePayloadsReq {
                method: "module.compatible_payloads".into(),
                token: self.token.clone(),
                modname: m_name
            };
            self.commuicate::<ModuleCompatiblePayloadsReq,ModuleCompatiblePayloadsRes>(payloads)
        }

        pub fn target_compatible_payloads(&mut self, m_name: String, tgt: i64) -> Result<ModuleTargetCompatiblePayloadsRes,String> {
            let target = if tgt < 0 { 0 } else { tgt };
            let payloads = ModuleTargetCompatiblePayloadsReq {
                method: "module.target_compatible_payloads".into(),
                token: self.token.clone(),
                modname: m_name,
                tgt: target.to_string()
            };
            self.commuicate::<ModuleTargetCompatiblePayloadsReq,ModuleTargetCompatiblePayloadsRes>(payloads)
        }

        pub fn module_execute(&mut self, m_type: String, m_name: String, options: HashMap<String,String>) -> Result<ModuleExecuteRes,String> {
            let execute = ModuleExecuteReq {
                method: "module.execute".into(),
                token: self.token.clone(),
                modtype: m_type,
                modname: m_name,
                options
            };
            self.commuicate::<ModuleExecuteReq,ModuleExecuteRes>(execute)
        }

        pub fn jobs(&mut self) -> Result<JobListRes,String> {
            let jobs = JobListReq {
                method: "job.list".into(),
                token: self.token.clone(),
            };
            self.commuicate::<JobListReq,JobListRes>(jobs)
        }

        pub fn job_stop(&mut self,job_id: String) -> Result<JobStopRes,String> {
            let job_stop = JobStopReq {
                method: "job.stop".into(),
                token: self.token.clone(),
                job_id
            };
            self.commuicate::<JobStopReq,JobStopRes>(job_stop)
        }
    }
}
