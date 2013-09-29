use self::routes::*;
use self::response::*;

use http::server::Request;
use http::status::NotFound;

pub mod routes;
pub mod response;

#[deriving(Clone)]
pub struct Application<T> {
  routes: ~Routes<T>,
}

impl<T: ToResponse> Application<T> {
  pub fn new(draw: &fn (&mut Routes<T>)) -> Application<T> {
    let mut server = Application { routes: ~Routes::new() };
    draw(server.routes);
    server
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