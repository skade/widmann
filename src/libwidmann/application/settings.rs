use std::rt::io::net::ip::{SocketAddr, IpAddr, Ipv4Addr};
use knob::Settings;

pub trait SocketSettings {
  fn socket(&self) -> SocketAddr;
  fn port(&self) -> u16;
  fn ip(&self) -> IpAddr;
}

impl SocketSettings for Settings {
  fn socket(&self) -> SocketAddr {
    do self.fetch_with("addr") |addr| {
      match addr {
        Some(socket_addr) => { socket_addr },
        None => {
          let port: u16 = self.port();
          let ip: IpAddr = self.ip();
          SocketAddr { ip: ip, port: port }
        }
      }
    }
  }

  fn port(&self) -> u16 {
    self.fetch("port").unwrap_or(8080)
  }

  fn ip(&self) -> IpAddr {
    self.fetch("ip").unwrap_or(Ipv4Addr(127,0,0,1))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_insert() {
    let mut settings = Settings::new();
    settings.set("port", "12345");
    settings.set("ip", "127.0.0.1");
    settings.socket();
  }
}