use knob::Settings;
use http::server::Request;

use super::params::Params;

pub struct Context {
  settings: Settings,
  params: Params,
  request: Request
}
