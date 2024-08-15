mod core {
    pub mod ssh_helper;
}

use clap::Parser;
use core::ssh_helper::SSHHelperTunnel;

/// SSH Tunnel Tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of local port to tunnel
    #[arg(long, short = 'l')]
    local_port: u16,

    /// IP to tunnel on target machine
    #[arg(long, short = 'r')]
    remote_ip: String,

    /// Number of remote port (on target machine)
    #[arg(long, short = 'p')]
    remote_port: u16,

    /// IP the helper is supposed to connect 
    #[arg(long, short = 'c')]
    connect_ip: String,

    /// Username to use for the connection
    #[arg(long, short = 'n')]
    name: String,
}


fn main() {
    let args = Args::parse();
    let sshh = SSHHelperTunnel::new(args.local_port, args.remote_ip, args.remote_port, args.connect_ip, args.name);
    sshh.run();
}
