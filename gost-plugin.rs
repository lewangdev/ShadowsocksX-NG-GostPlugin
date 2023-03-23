use std::env;
use std::process::Command;

fn main() {
    let ss_local_host = env::var("SS_LOCAL_HOST").unwrap();
    let ss_local_port = env::var("SS_LOCAL_PORT").unwrap();
    let ss_remote_host = env::var("SS_REMOTE_HOST").unwrap();
    let ss_remote_port = env::var("SS_REMOTE_PORT").unwrap();
    let ss_plugin_options = env::var("SS_PLUGIN_OPTIONS").unwrap();

    let cmd = format!(
        "./gost/gost -L=ss://chacha20-ietf-poly1305:123456@{}:{} -F=wss://{}@{}:{}",
        ss_local_host, ss_local_port, ss_plugin_options, ss_remote_host, ss_remote_port
    );

    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("运行命令失败");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("命令执行失败：{}", stderr);
    }
}
