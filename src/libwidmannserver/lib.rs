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
use http::headers::content_type::MediaType;

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
      w.headers.content_type = Some(MediaType {
          type_: ~"text",
          subtype: ~"plain",
          parameters: ~[(~"charset", ~"UTF-8")]
      });
      w.headers.server = Some(~"Example");

      w.headers.content_length = Some(response.body.len());

      match response {
        Response { status, body } => {
          w.status = status;
          w.write(body.as_bytes());
        }
      }
    }
}