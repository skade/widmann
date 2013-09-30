//! A very simple HTTP server which responds with the plain text "Hello, World!" to every request.

extern mod extra;
extern mod http;
extern mod widmann;
extern mod widmannserver;

use http::server::{ServerUtil, Request};
use std::rt::io::net::ip::{SocketAddr, Ipv4Addr};
use std::os;

use extra::getopts::*;

use widmann::application::context::*;
use widmann::application::*;
use widmannserver::*;

fn hello_world(context: &Context, _request: &Request) -> ~str {
  let params = &context.params;
  let id = params.get_copy(&~"id");
  match id {
    Some(m) => m.to_owned(),
    None => ~"pass an ID!"
  }
}

fn hello_post(_context: &Context, _request: &Request) -> ~str {
  ~"Thanks for the POST!\n"
}

fn main() {
    let args = os::args();

    let opts = ~[
      optopt("p"),
      optopt("port"),
    ];

    let matches = match getopts(args.tail(), opts) {
      Ok(m) => { m }
      Err(f) => { fail!(f.to_err_msg()) }
    };

    let port_option = matches.opts_str([~"p", ~"port"]);
    let port: int = match port_option {
      Some(option) => { from_str(option).expect("--port given but not a proper number") }
      None => { 4000 }
    };

    let app = do Application::new |app|
      {
        do app.settings |settings| {
          settings.socket = Some(SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: port as u16 })
        }
        do app.routes |routes| {
          routes.get(~"/foo/(?<id>.*)", hello_world);
          routes.post(~"/", hello_post);
        }
      };
    let server = WidmannServer::new(app);
    server.serve_forever();
}
