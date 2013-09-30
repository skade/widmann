use super::context::*;

use pcre::pcre::*;
use pcre::consts::*;

use std::hashmap::*;

use http::server::request::{Request, AbsolutePath};
use http::method::*;

pub struct Route<T> {
  method: Method,
  path: ~str,
  f: extern fn(&Context, &Request) -> T,
}

impl<T> Clone for Route<T> {
  fn clone(&self) -> Route<T> {
    Route { method: self.method.clone(), path: self.path.clone(), f: self.f }
  }
}

pub struct MatchedRoute<T> {
  params: HashMap<~str, Option<~str>>,
  f: extern fn(&Context, &Request) -> T,
}

impl<T> Clone for MatchedRoute<T> {
  fn clone(&self) -> MatchedRoute<T> {
    MatchedRoute { params: self.params.clone(), f: self.f }
  }
}

impl<T> Route<T> {
  pub fn new(method: Method, path: ~str, routeFn: extern fn(&Context, &Request) -> T) -> Route<T> {
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

  pub fn find<'a>(&'a self, request: &Request) -> Option<MatchedRoute<T>> {
    match request.request_uri {
      AbsolutePath(ref path) => {
        for route in self.routes.iter() {
          if route.method == request.method {
            let res = search(route.path.clone(), *path, PCRE_ANCHORED);
            match res {
              Ok(m) => {
                let mut map = HashMap::new();
                let group_names = m.group_names();
                for name in group_names.iter() {
                  let group = m.named_group(*name);
                  match group {
                    Some(str) => { map.insert(name.to_owned(), Some(str.to_owned())); }
                    None => { map.insert(name.to_owned(), None); }
                  }
                }
                return Some(MatchedRoute { params: map, f: route.f });
              }
              Err(_) => { }
            }
          }
        }
        return None
      },
      _ => { None }
    }
  }

  pub fn route(&mut self, method: Method, path: ~str, routeFn: extern fn(&Context, &Request) -> T) {
    self.routes.push(Route::new(method, path, routeFn))
  }

  pub fn get(&mut self, path: ~str, routeFn: extern fn(&Context, &Request) -> T) { self.route(Get, path, routeFn) }
  pub fn post(&mut self, path: ~str, routeFn: extern fn(&Context, &Request) -> T) { self.route(Post, path, routeFn) }
  pub fn put(&mut self, path: ~str, routeFn: extern fn(&Context, &Request) -> T) { self.route(Put, path, routeFn) }
  pub fn delete(&mut self, path: ~str, routeFn: extern fn(&Context, &Request) -> T) { self.route(Delete, path, routeFn) }
  pub fn patch(&mut self, path: ~str, routeFn: extern fn(&Context, &Request) -> T) { self.route(Patch, path, routeFn) }
}