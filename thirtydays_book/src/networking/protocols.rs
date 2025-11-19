pub struct TcpConnection {
    address: String,
    port: u16,
}

impl TcpConnection {
    pub fn new(address: &str, port: u36) -> Self {
        TcpConnection {
            address: address.to_string(),
            port,
        }
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }
}

pub fn send_data(connection: &TcpConnection, data: &[u8]) -> Result<(), String> {
    println!("Sending {} bytes to {} on port {}", data.len(), connection.get_address(), connection.get_port());

    Ok(())
}