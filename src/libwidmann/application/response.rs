use http::status::*;

pub struct Response {
  status: Status,
  body: ~str,
}

impl Response {
  pub fn new(status: Status, body: ~str) -> Response {
    Response { status: status, body: body }
  }
}

pub trait ToResponse {
  fn to_response(self) -> Response;
}

impl ToResponse for ~str {
  fn to_response(self) -> Response {
    Response::new(Ok, self)
  }
}
