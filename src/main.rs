use std::process::Command;
use clap::Parser;

/// ssh tunnel tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of local port to tunnel
    #[arg(long)]
    local_port: u16,

    /// IP to tunnel on target machine
    #[arg(long)]
    remote_ip: String,

    /// Number of remote port (on target machine)
    #[arg(long)]
    remote_port: u16,

    /// IP the helper is supposed to connect 
    #[arg(long)]
    connect_ip: String,

    /// Username to use for the connection
    #[arg(long)]
    name: String,
}

fn main() {
    let args = Args::parse();

    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", format!("ssh -N -L {}:{}:{} {} -l {}", args.local_port, args.remote_ip, args.remote_port, args.connect_ip, args.name).as_str()])
            .output()
            .expect("failed to execute process");
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process");
    }    
}
