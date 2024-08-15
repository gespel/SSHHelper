use std::process::Command;

pub struct SSHHelper {
    local_port: u16,
    remote_ip: String,
    remote_port: u16,
    connect_ip: String,
    name: String
}

impl SSHHelper {
    pub fn new(
        local_port: u16,
        remote_ip: String, 
        remote_port: u16, 
        connect_ip: String, 
        name: String
    ) -> SSHHelper {
        SSHHelper {
            local_port,
            remote_ip,
            remote_port,
            connect_ip,
            name
        }
    }

    pub fn run(&self) {
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", format!("ssh -N -L {}:{}:{} {} -l {}", self.local_port, self.remote_ip, self.remote_port, self.connect_ip, self.name).as_str()])
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
}