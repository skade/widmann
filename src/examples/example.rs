//! A very simple HTTP server which responds with the plain text "Hello, World!" to every request.

extern mod extra;
extern mod http;
extern mod widmann;
extern mod widmannserver;

use http::server::{ServerUtil, Request};
use std::rt::io::net::ip::{SocketAddr, Ipv4Addr};

use widmann::application::*;
use widmannserver::*;

fn hello_world(_request: &Request) -> ~str {
  ~"Hello World!\n"
}

fn hello_post(_request: &Request) -> ~str {
  ~"Thanks for the POST!\n"
}

fn main() {
    let app = do Application::new |app|
      {
        do app.settings |settings| {
          settings.socket = Some(SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 4000 })
        }
        do app.routes |routes| {
          routes.get(~"/foo/(?<bar>.*)", hello_world);
          routes.post(~"/", hello_post);
        }
      };
    let server = WidmannServer::new(app);
    server.serve_forever();
}

