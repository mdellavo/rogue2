use std::net::SocketAddr;
use tokio::sync::mpsc;

pub struct ClientConnection {
    pub id: u64,
    pub addr: SocketAddr,
    pub sender: mpsc::UnboundedSender<Vec<u8>>,
}

impl ClientConnection {
    pub fn send_message(&self, data: Vec<u8>) -> Result<(), mpsc::error::SendError<Vec<u8>>> {
        self.sender.send(data)
    }
}
