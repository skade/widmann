#[link(name = "widmann",
       vers = "0.1-pre",
       url = "")];

#[crate_type = "lib"];

extern mod http;
extern mod extra;

pub mod application;
pub mod server;