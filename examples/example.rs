//! A very simple HTTP server which responds with the plain text "Hello, World!" to every request.

extern mod extra;
extern mod http;
extern mod widmann;
extern mod widmannserver;
extern mod knob;

use http::server::ServerUtil;

use extra::getopts::groups::*;

use widmann::application::context::*;
use widmann::application::*;
use widmannserver::*;
use knob::Settings;

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

fn hello_world(context: Context) -> ~str {
  let params = &context.params;
  let id = params.get_copy(&~"id");
  match id {
    Some(m) => m.to_owned(),
    None => ~"pass an ID!"
  }
}

fn hello_post(_context: Context) -> ~str {
  ~"Thanks for the POST!\n"
}

fn main() {
    let app = do Application::new |app|
      {
        do app.settings |settings| {
          settings.opt(optopt("p", "port", "the port to bind to", "4000"));
          settings.set("ip", "127.0.0.1");
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
