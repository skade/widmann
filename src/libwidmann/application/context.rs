use std::hashmap::*;
use knob::Settings;

pub struct Context<'self> {
  settings: &'self Settings,
  params: HashMap<~str, Option<~str>>,
}