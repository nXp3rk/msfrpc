use subprocess::{Exec,Redirection,Popen,PopenError};

#[cfg(target_os = "linux")]
pub fn launch(user: &'static str, pass:&'static str,ssl: bool) -> Result<Popen,PopenError>{
    let cmd = Exec::cmd("/usr/bin/msfrpcd");
    let mut args = vec!["-U",user,"-P",pass];
    if !ssl {
        args.push("-S");
    }
    let cmd = cmd.args(&args).stderr(Redirection::Pipe).popen();
    return cmd;
}

pub fn stop(p: &mut Popen) {
    p.kill().unwrap();
    Exec::shell("killall ruby").join().unwrap();
}
