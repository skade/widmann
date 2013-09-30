use self::routes::*;
use self::response::*;
use self::settings::*;
use self::context::*;

use http::server::Request;
use http::status::NotFound;

pub mod routes;
pub mod response;
pub mod settings;
pub mod context;

#[deriving(Clone)]
pub struct Application<T> {
  routes: ~Routes<T>,
  settings: ~Settings
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
        match route {
          MatchedRoute { params, f } => {
            let ctx = Context { settings: self.settings.clone(), params: params };
            let result = f(&ctx, request);
            result.to_response()
          }
        }

      },
      None => {
        Response::new(NotFound, ~"Not Found")
      }
    }
  }
}