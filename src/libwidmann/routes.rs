use http::server::request::{Request, AbsolutePath};
use http::method::*;

pub struct Route<T> {
  method: Method,
  path: ~str,
  f: extern fn(&Request) -> T,
}

impl<T> Clone for Route<T> {
  fn clone(&self) -> Route<T> {
    Route { method: self.method.clone(), path: self.path.clone(), f: self.f }
  }
}

impl<T> Route<T> {
  pub fn new(method: Method, path: ~str, routeFn: extern fn(&Request) -> T) -> Route<T> {
    Route { method: method, path: path, f: routeFn }
  }
}

#[deriving(Clone)]
pub struct Routes<T> {
  routes: ~[Route<T>]
}

impl<T> Routes<T> {
  pub fn new() -> Routes<T> {
    Routes { routes: ~[] }
  }

  pub fn find<'a>(&'a self, request: &Request) -> Option<&'a Route<T>> {
    match request.request_uri {
      AbsolutePath(ref path) => {
        do self.routes.iter().find |route|
          { route.method == request.method && route.path == path.to_str() }
      },
      _ => { None }
    }
  }

  pub fn route(&mut self, method: Method, path: ~str, routeFn: extern fn(&Request) -> T) {
    self.routes.push(Route::new(method, path, routeFn))
  }

  pub fn get(&mut self, path: ~str, routeFn: extern fn(&Request) -> T) { self.route(Get, path, routeFn) }
  pub fn post(&mut self, path: ~str, routeFn: extern fn(&Request) -> T) { self.route(Post, path, routeFn) }
  pub fn put(&mut self, path: ~str, routeFn: extern fn(&Request) -> T) { self.route(Put, path, routeFn) }
  pub fn delete(&mut self, path: ~str, routeFn: extern fn(&Request) -> T) { self.route(Delete, path, routeFn) }
  pub fn patch(&mut self, path: ~str, routeFn: extern fn(&Request) -> T) { self.route(Patch, path, routeFn) }
}