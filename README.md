# Widmann

[![Build Status](https://travis-ci.org/skade/widmann.png)](https://travis-ci.org/skade/widmann)

Sinatra's law: given enough time, every programming language will see an attempt at a Sinatra clone. Here's mine in Rust.

This is obviously a proof of concept, with no features beyond routing and settings.

## Usage

```rust
// Some imports

// Handler functions have the signature (&Context, &Request) -> T: ToResponse
// So you are free to invent your own return types.
fn hello_world(context: &Context, _request: &Request) -> ~str {
  let params = &context.params;
  let id = params.get_copy(&~"id");
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
          settings.set("ip", "127.0.0.1");
          settings.set("port", 4000);
          // you can even use:
          // settings.opt(optopt("p", "port", "the port to bind to", "default: 4000"))
          // the long name is the settings name
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

Make sure to have a recent Rust, `0.8` is not enough. I currently test with mozilla/rust@5409983;

Currently, no `rustpkg` is available, as both projects this one depends on cannot be built using `rustpkg`. A Makefile is provided to get you started:

```
git submodule update --init
make
```

Should do the trick. Have a look at the `examples` folder. If all went well, you can try it out:

```
build/example

curl localhost:4000/foo/bar
```

## TODO

* Templating

## License

MIT, see `LICENSE.md`.
