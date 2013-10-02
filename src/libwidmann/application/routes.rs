use super::context::*;
use super::params::*;

use pcre::pcre::*;
use pcre::consts::*;

use http::server::request::{Request, AbsolutePath};
use http::method::*;

pub struct Route<T> {
  method: Method,
  path: ~str,
  f: extern fn(Context) -> T,
}

impl<T> Clone for Route<T> {
  fn clone(&self) -> Route<T> {
    Route { method: self.method.clone(), path: self.path.clone(), f: self.f }
  }
}

pub struct MatchedRoute<T> {
  method: Method,
  params: Params,
  f: extern fn(Context) -> T,
}

impl<T> Clone for MatchedRoute<T> {
  fn clone(&self) -> MatchedRoute<T> {
    MatchedRoute { method: self.method.clone(), params: self.params.clone(), f: self.f }
  }
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

impl<T> Routes<T> {
  pub fn new() -> Routes<T> {
    Routes { routes: ~[] }
  }

  pub fn find<'a>(&'a self, request: &Request) -> Result<MatchedRoute<T>, RouteMatchError> {
    match request.request_uri {
      AbsolutePath(ref path) => {
        let matched_routes = self.routes.iter().filter_map(|route| {
          let res = search(route.path.clone(), *path, PCRE_ANCHORED);
          match res {
            Ok(m) => {
              let params = Params::from_match(m);
              Some(MatchedRoute { method: route.method.clone(), params: params, f: route.f })
            }
            Err(_) => { None }
          }
        }).to_owned_vec();

        if matched_routes.len() == 0 {
          return Err(NotFoundError)
        };

        let route = matched_routes.iter().find(|r| {
          r.method == request.method
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