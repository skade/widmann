use self::routes::*;
use self::response::*;

use http::server::Request;
use http::status::NotFound;
use std::rt::io::net::ip::SocketAddr;

pub mod routes;
pub mod response;

#[deriving(Clone)]
pub struct Application<T> {
  routes: ~Routes<T>,
  settings: ~Settings
}

#[deriving(Clone)]
pub struct Settings {
  socket: Option<SocketAddr>
}

impl Settings {
  fn new() -> Settings {
    Settings { socket: None }
  }
}

impl<T: ToResponse> Application<T> {
  pub fn new(create: &fn (&mut Application<T>)) -> Application<T> {
    let mut app = ~Application { routes: ~Routes::new(), settings: ~Settings::new() };
    create(app);
    *app
  }

  pub fn settings<'a>(&'a mut self, config: &fn(&'a mut Settings)) {
    config(self.settings)
  }

  pub fn routes<'a>(&'a mut self, draw: &fn(&'a mut Routes<T>)) {
    draw(self.routes)
  }

  pub fn call(&self, request: &Request) -> Response {
    match self.routes.find(request) {
      Some(route) => {
        let f = route.f;
        let result = f(request);
        result.to_response()
      },
      None => {
        Response::new(NotFound, ~"Not Found")
      }
    }
  }
}