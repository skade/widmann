//! A very simple HTTP server which responds with the plain text "Hello, World!" to every request.

extern mod extra;
extern mod http;
extern mod widmann;
extern mod widmannserver;

use http::server::{ServerUtil, Request};
use std::os;

use extra::getopts::*;

use widmann::application::settings::*;
use widmann::application::context::*;
use widmann::application::*;
use widmannserver::*;

enum Environment {
  Development,
  Production,
  Test,
  Other(~str)
}

impl ToStr for Environment {
  fn to_str(&self) -> ~str {
    match *self {
      Development => { ~"development" },
      Production => { ~"production" },
      Test => { ~"test" },
      Other(ref string) => { string.to_owned() }
    }
  }
}

impl FromStr for Environment {
  fn from_str(string: &str) -> Option<Environment> {
    let env = match string {
      "development" => { Development },
      "production" => { Production },
      "test" => { Test },
      string => { Other(string.to_owned()) }
    };
    Some(env)
  }
}

trait MySettings {
  fn environment(&self) -> Environment;
}

impl MySettings for Settings {
  fn environment(&self) -> Environment {
    self.fetch("environment").expect("environment has to be provided!")
  }
}

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

    let port = matches.opts_str([~"p", ~"port"]);

    let app = do Application::new |app|
      {
        do app.settings |settings| {
          settings.opt("port", port.clone());
          settings.set("environment", Production);
        }
        do app.routes |routes| {
          routes.get(~"/foo/(?<id>.*)", hello_world);
          routes.post(~"/", hello_post);
        }
      };
    let server = WidmannServer::new(app);
    server.serve_forever();
}
