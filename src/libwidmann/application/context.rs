use std::hashmap::*;
use super::settings::*;

pub struct Context<'self> {
  settings: &'self Settings,
  params: HashMap<~str, Option<~str>>,
}