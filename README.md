# Widmann


Every language it's Sinatra clone, so here is one for Rust.

Consider this a proof of concept, it has some glaring bugs. (try POSTing to `/foo`)

## Usage

```rust
// Some imports

// Handler functions have the signature (&Context, &Request) -> T: ToResponse
// So you are free to invent your own return types.
fn hello_world(context: &Context, _request: &Request) -> ~str {
  let params = &context.params;
  let id = params.get(&~"id").clone();
  match id {
    Some(m) => m.to_owned(),
    None => ~"pass an ID!"
  }
}

fn hello_post(_context: &Context, _request: &Request) -> ~str {
  ~"Thanks for the POST!\n"
}

fn main() {
    let app = do Application::new |app|
      {
        do app.settings |settings| {
          settings.socket = Some(SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 4000 })
        }
        do app.routes |routes| {
          routes.get(~"/foo/(?<id>.*)", hello_world);
          routes.post(~"/", hello_post);
        }
      };
    let server = WidmannServer::new(app);
    server.serve_forever();
}
```

## Installation

Currently, no `rustpkg` is available, as both projects this one depends on cannot be built using `rustpkg`. A Makefile is provided to get you started:

```
git submodule update --init
make
```

Should do the trick. Have a look at the `examples` folder.

## License

MIT, see `LICENSE.md`.