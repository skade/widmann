use http::status::*;
use http::headers::response::HeaderCollection;
use http::headers::content_type::MediaType;

pub struct Response {
  status: Status,
  body: ~str,
  headers: ~HeaderCollection
}

impl Response {
  pub fn new(status: Status, body: ~str) -> Response {
    Response { status: status, body: body, headers: ~HeaderCollection::new() }
  }
}

pub trait ToResponse {
  fn to_response(self) -> Response;
}

impl ToResponse for ~str {
  fn to_response(self) -> Response {
    let mut response = Response::new(Ok, self);
    response.headers.content_type = Some(MediaType {
      type_: ~"text",
      subtype: ~"plain",
      parameters: ~[(~"charset", ~"UTF-8")]
    });
    response.headers.content_length = Some(response.body.len());
    response
  }
}

impl ToResponse for () {
  fn to_response(self) -> Response {
    Response::new(Ok, ~"")
  }
}