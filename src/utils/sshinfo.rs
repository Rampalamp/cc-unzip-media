#[derive(Debug)]
pub struct SSHInfo {
    host: String,
    port: i32,
    username: Option<String>,
    password: Option<String>,
}

impl SSHInfo {
    pub fn parse_ssh_info(path: &str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = path.splitn(2, '@').collect();

        let (username, host_with_port) = match parts.len() {
            1 => (None, parts[0]),
            2 => (Some(parts[0].to_string()), parts[1]),
            _ => return Err("Invalid SSH command"),
        };

        let host_port_parts: Vec<&str> = host_with_port.splitn(2, ':').collect();

        // Extract host and port
        let (host, port_str) = match host_port_parts.len() {
            1 => (host_port_parts[0], "22"), // Default SSH port is 22
            2 => (host_port_parts[0], host_port_parts[1]),
            _ => return Err("Invalid host or port"),
        };

        // Parse port string into i32
        let port: i32 = port_str.parse().map_err(|_| "Invalid port")?;

        Ok(SSHInfo {
            host: host.to_string(),
            port,
            username,
            password: None, // Default value for password, you can add logic to extract password if needed
        })
    }
}
