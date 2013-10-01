use std::hashmap::*;
use std::rt::io::net::ip::{SocketAddr, IpAddr, Ipv4Addr};
use std::os;
use extra::getopts::groups::*;

#[deriving(Clone)]
pub struct Settings {
  priv store: HashMap<~str,~str>,
  priv options: ~[OptGroup],
}

impl Settings {
  pub fn new() -> Settings {
    Settings { store: HashMap::new(), options: ~[] }
  }

  pub fn compile(&mut self) {
    debug!("compiling options");

    let args = os::args();


    let matches = match getopts(args.tail(), self.options) {
      Ok(m) => { m }
      Err(f) => { fail!(f.to_err_msg()) }
    };

    debug!(matches);

    let given_options = self.options.clone();
    for opt in given_options.iter() {
      let opt_strings = &[opt.short_name.clone(), opt.long_name.clone()];
      self.set_opt(opt.long_name.clone(), matches.opts_str(opt_strings))
    };
  }

  pub fn opt(&mut self, opt: OptGroup) {
    self.options.push(opt);
  }

  pub fn set<A: ToStr, T: ToStr>(&mut self, setting: A, value: T) {
    self.store.swap(setting.to_str(), value.to_str());
  }

  pub fn set_opt<A: ToStr, T: ToStr>(&mut self, setting: A, value: Option<T>) {
    if !value.is_none() {
      self.store.swap(setting.to_str(), value.unwrap().to_str());
    }
  }

  pub fn fetch_with<A: ToStr, T: FromStr>(&self, setting: A, f: &fn(Option<T>) -> T) -> T {
    let value = self.fetch(setting.to_str());
    f(value)
  }

  pub fn fetch<A: ToStr, T: FromStr>(&self, setting: A) -> Option<T> {
    match self.store.find(&setting.to_str()) {
      Some(string) => { from_str(string.to_owned()) },
      None => { None }
    }
  }
}

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