use msfrpc::client::Msfrpc;

fn main() {
    println!("[+] Start msfrpcd...");
    let module_name: &str = "exploits";
    //wait for 10 second.
    let mut rpc = Msfrpc::new("127.0.0.1",55553,"msf","msfpasswd",false);
    rpc.login();
    let modules = rpc.modules(module_name).unwrap();
    for m in &modules {
        println!("[=] module: {}",m.clone());
        let options = rpc.info(module_name.to_string(),m.clone());
        println!("|-- options : {:?}",options);
    }
    rpc.logout();
}
