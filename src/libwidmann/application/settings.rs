use std::rt::io::net::ip::SocketAddr;

#[deriving(Clone)]
pub struct Settings {
  socket: Option<SocketAddr>
}

impl Settings {
  pub fn new() -> Settings {
    Settings { socket: None }
  }
}
