#[link(name = "widmannserver",
       vers = "0.1-pre",
       url = "")];

#[crate_type = "lib"];

extern mod extra;
extern mod http;
extern mod widmann;

use std::rt::io::net::ip::{SocketAddr, Ipv4Addr};
use std::rt::io::Writer;
use extra::time;

use http::server::{Config, Server, Request, ResponseWriter};

use widmann::application::*;
use widmann::application::response::*;

#[deriving(Clone)]
pub struct WidmannServer<T> {
  application: Application<T>
}

impl<T> WidmannServer<T> {
  pub fn new(application: Application<T>) -> WidmannServer<T> {
    WidmannServer { application: application }
  }
}

impl<T: ToResponse> Server for WidmannServer<T> {
  fn get_config(&self) -> Config {
    Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8001 } }
  }

  fn handle_request(&self, r: &Request, w: &mut ResponseWriter) {
    let response = self.application.call(r);

    w.headers.date = Some(time::now_utc());
    w.headers.server = Some(~"Widmann");

    match response {
      Response { status, body, headers } => {
        w.status = status;
        for header in headers.iter() {
          w.headers.insert(header);
        }
        w.write(body.as_bytes());
      }
    }
  }
}