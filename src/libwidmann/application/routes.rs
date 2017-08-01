use super::context::Context;
use super::params::Params;

use pcre::pcre::search;
use pcre::consts::PCRE_ANCHORED;

use http::server::request::{Request, AbsolutePath};
use http::method::Method;

pub struct Route<T> {
  method: Method,
  path: ~str,
  priv f: extern fn(Context) -> T,
}

impl<T> Route<T> {
  pub fn call(&self, context: Context) -> T {
    let f = self.f;
    f(context)
  }
}

impl<T> Clone for Route<T> {
  fn clone(&self) -> Route<T> {
    Route { method: self.method.clone(), path: self.path.clone(), f: self.f }
  }
}

#[deriving(Clone)]
pub struct MatchedRoute<T> {
  route: Route<T>,
  params: Params,
}

impl<T> Route<T> {
  pub fn new(method: Method, path: ~str, routeFn: extern fn(Context) -> T) -> Route<T> {
    let mut pattern = path;
    if !pattern.ends_with("$") {
      pattern.push_char('$');
    }
    Route { method: method, path: pattern, f: routeFn }
  }
}

#[deriving(Clone)]
pub struct Routes<T> {
  routes: ~[Route<T>]
}

pub enum RouteMatchError {
  NotFoundError,
  MethodNotAllowedError
}

impl<T: Clone> Routes<T> {
  pub fn new() -> Routes<T> {
    Routes { routes: ~[] }
  }

  pub fn find(self, request: &Request) -> Result<(Route<T>, Params), RouteMatchError> {
    match request.request_uri {
      AbsolutePath(ref path) => {
        let mut matched_routes = ~[];
        for route in self.routes.iter() {
          let res = search(route.path.clone(), *path, PCRE_ANCHORED);
          match res {
            Ok(m) => {
              let params = Params::from_match(m);
              matched_routes.push((route.clone(), params));
            }
            Err(_) => { }
          }
        };

        if matched_routes.len() == 0 {
          return Err(NotFoundError)
        };

        let route = matched_routes.iter().find(|m| {
          match **m {
            (ref route, _) => route.method == request.method
          }
        });

        if route.is_none() {
          return Err(MethodNotAllowedError);
        } else {
          return Ok(route.unwrap().clone());
        }

      },
      _ => { Err(NotFoundError) }
    }
  }

  pub fn route(&mut self, method: Method, path: ~str, routeFn: extern fn(Context) -> T) {
    self.routes.push(Route::new(method, path, routeFn))
  }

  pub fn get(&mut self, path: ~str, routeFn: extern fn(Context) -> T) { self.route(Get, path, routeFn) }
  pub fn post(&mut self, path: ~str, routeFn: extern fn(Context) -> T) { self.route(Post, path, routeFn) }
  pub fn put(&mut self, path: ~str, routeFn: extern fn(Context) -> T) { self.route(Put, path, routeFn) }
  pub fn delete(&mut self, path: ~str, routeFn: extern fn(Context) -> T) { self.route(Delete, path, routeFn) }
  pub fn patch(&mut self, path: ~str, routeFn: extern fn(Context) -> T) { self.route(Patch, path, routeFn) }
}
