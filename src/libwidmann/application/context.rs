use std::hashmap::*;
use knob::Settings;
use http::server::Request;

pub struct Context<'self> {
  settings: &'self Settings,
  params: HashMap<~str, Option<~str>>,
  request: &'self Request
}