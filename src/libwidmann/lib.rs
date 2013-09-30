#[link(name = "widmann",
       vers = "0.1-pre",
       url = "")];

#[crate_type = "lib"];
#[link_args = "-lpcre"];

extern mod pcre = "github.com/skade/rust-pcre";
extern mod http;
extern mod extra;

pub mod application;