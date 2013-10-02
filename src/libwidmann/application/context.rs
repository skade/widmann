use knob::Settings;
use http::server::Request;

use super::params::*;

pub struct Context<'self> {
  settings: &'self Settings,
  params: Params,
  request: &'self Request
}