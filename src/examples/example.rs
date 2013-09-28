//! A very simple HTTP server which responds with the plain text "Hello, World!" to every request.

extern mod extra;
extern mod http;
extern mod widmann;

use std::rt::io::net::ip::{SocketAddr, Ipv4Addr};
use std::rt::io::Writer;
use extra::time;

use http::server::{Config, Server, ServerUtil, Request, ResponseWriter};
use http::headers::content_type::MediaType;
use http::status::{Ok, NotFound};

use widmann::routes::*;

#[deriving(Clone)]
struct HelloWorldServer {
  routes: ~Routes<~str>,
}

impl HelloWorldServer {
  fn new(draw: &fn (&mut Routes<~str>)) -> HelloWorldServer {
    let mut server = HelloWorldServer { routes: ~Routes::new() };
    draw(server.routes);
    server
  }
}

impl Server for HelloWorldServer {
    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8001 } }
    }

    fn handle_request(&self, r: &Request, w: &mut ResponseWriter) {
        match self.routes.find(r) {
          Some(route) => {
            let f = route.f;
            let result = f(r);
            w.status = Ok;
            w.headers.content_length = Some(result.len());
            w.write(result.as_bytes())
          },
          None => {
            w.status = NotFound;
            w.headers.content_length = Some(17);
            w.write(bytes!("sorry, not here!\n"))
          }
        };
        w.headers.date = Some(time::now_utc());
        w.headers.content_type = Some(MediaType {
            type_: ~"text",
            subtype: ~"plain",
            parameters: ~[(~"charset", ~"UTF-8")]
        });
        w.headers.server = Some(~"Example");

        w.write(bytes!("Hello, World!\n"));
    }
}

fn hello_world(_request: &Request) -> ~str {
  ~"Hello World!\n"
}

fn hello_post(_request: &Request) -> ~str {
  ~"Thanks for the POST!\n"
}

fn main() {
    let app = do HelloWorldServer::new |routes| {
      routes.get(~"/", hello_world);
      routes.post(~"/", hello_post);
    };
    app.serve_forever();
}

