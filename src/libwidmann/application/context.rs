use std::hashmap::*;
use super::settings::*;

pub struct Context {
  settings: ~Settings,
  params: HashMap<~str, Option<~str>>,
}