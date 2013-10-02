use self::routes::*;
use self::response::*;
use self::settings::*;
use self::context::*;

use http::server::Request;
use http::*;
use knob::Settings;

pub mod routes;
pub mod response;
pub mod settings;
pub mod context;
pub mod params;

#[deriving(Clone)]
pub struct Application<T> {
  routes: Routes<T>,
  settings: Settings
}

impl<T: ToResponse + Clone> Application<T> {
  pub fn new(create: &fn (&mut Application<T>)) -> Application<T> {
    let mut app = ~Application { routes: Routes::new(), settings: Settings::new() };
    create(app);
    app.settings.load_os_args();
    *app
  }

  pub fn settings<'a>(&'a mut self, config: &fn(&'a mut Settings)) {
    config(&'a mut self.settings);
  }

  pub fn routes<'a>(&'a mut self, draw: &fn(&'a mut Routes<T>)) {
    draw(&'a mut self.routes)
  }

  pub fn call(&self, request: &Request) -> Response {
    match self.routes.clone().find(request) {
      Ok(route) => {
        let real_route = route.route.clone();
        let ctx = Context { settings: &self.settings, params: route.params, request: request };
        let result = real_route.call(ctx);
        result.to_response()
      },
      Err(error) => {
        match error {
          NotFoundError => { Response::new(status::NotFound, ~"Not Found") },
          MethodNotAllowedError => { Response::new(status::MethodNotAllowed, ~"Method not allowed") }
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::context::*;

  use http::method::*;
  use http::status::*;
  use http::server::request::*;

  use std::rt::io::net::ip::{SocketAddr, Ipv4Addr};
  use http::headers::request::HeaderCollection;

  fn get(path: ~str) -> Request {
    Request {
        remote_addr: Some(SocketAddr { ip: Ipv4Addr(127,0,0,1), port:4000 }),
        headers: ~HeaderCollection::new(),
        body: ~"",
        method: Get,
        request_uri: AbsolutePath(path),
        close_connection: true,
        version: (1, 1),
    }
  }

  fn dummy(_ctx: Context) {

  }

  #[test]
  fn test_full_match() {
    let app = do Application::new |app| {
      do app.routes |routes| {
        routes.get(~"/foo", dummy)
      }
    };
    let res = app.call(&get(~"/foo"));
    assert_eq!(res.status, Ok);
  }

  #[test]
  fn test_longer_url() {
    let app = do Application::new |app| {
      do app.routes |routes| {
        routes.get(~"/foo", dummy)
      }
    };
    let res = app.call(&get(~"/foobar"));
    assert_eq!(res.status, NotFound);
  }

  #[test]
  fn test_url_prefix() {
    let app = do Application::new |app| {
      do app.routes |routes| {
        routes.get(~"/foo", dummy)
      }
    };
    let res = app.call(&get(~"/fo"));
    assert_eq!(res.status, NotFound);
  }

  #[test]
  fn test_wrong_method() {
    let app = do Application::new |app| {
      do app.routes |routes| {
        routes.post(~"/foo", dummy)
      }
    };
    let res = app.call(&get(~"/foo"));
    assert_eq!(res.status, MethodNotAllowed);
  }

  fn assert_id(context: Context) {
    let params = context.params;
    let id: Option<~str> = params.fetch(~"id");

    assert_eq!(id, Some(~"123"))
  }

  #[test]
  fn test_parameters() {
    let app = do Application::new |app| {
      do app.routes |routes| {
        routes.get(~"/foo/(?<id>\\d+)", assert_id)
      }
    };
    let res = app.call(&get(~"/foo/123"));
    assert_eq!(res.status, Ok);
    let res = app.call(&get(~"/foo"));
    assert_eq!(res.status, NotFound);
  }

  fn assert_ids_given(context: Context) {
    let params = context.params;
    let id: Option<~str> = params.fetch("id");
    let bar_id: Option<~str> = params.fetch("bar_id");

    assert_eq!(id, Some(~"123"))
    assert_eq!(bar_id, Some(~"456"))
  }

  #[test]
  fn test_optional_parameters_given() {
    let app = do Application::new |app| {
      do app.routes |routes| {
        routes.get(~"/foo/(?<id>\\d+)(?:/bar/(?<bar_id>\\d+))?", assert_ids_given)
      }
    };
    let res = app.call(&get(~"/foo/123/bar/456"));
    assert_eq!(res.status, Ok);
  }

  fn assert_ids_missing(context: Context) {
    let params = context.params;
    let id: Option<~str> = params.fetch("id");
    let bar_id: Option<~str> = params.fetch("bar_id");

    assert_eq!(id, Some(~"123"))
    assert_eq!(bar_id, None)
  }

  #[test]
  fn test_optional_parameters_missing() {
    let app = do Application::new |app| {
      do app.routes |routes| {
        routes.get(~"/foo/(?<id>\\d+)(?:/bar/(?<bar_id>\\d+))?", assert_ids_missing)
      }
    };
    let res = app.call(&get(~"/foo/123"));
    assert_eq!(res.status, Ok);
  }
}