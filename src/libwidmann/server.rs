extern mod extra;
extern mod http;

use std::rt::io::net::ip::{SocketAddr, Ipv4Addr};
use std::rt::io::Writer;
use extra::time;

use http::server::{Config, Server, Request, ResponseWriter};
use http::headers::content_type::MediaType;

use application::*;
use application::response::*;

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
      w.headers.content_type = Some(MediaType {
          type_: ~"text",
          subtype: ~"plain",
          parameters: ~[(~"charset", ~"UTF-8")]
      });
      w.headers.server = Some(~"Example");

      w.headers.content_length = Some(response.body.len());

      w.status = response.status;
      w.write(response.body.as_bytes());
    }
}