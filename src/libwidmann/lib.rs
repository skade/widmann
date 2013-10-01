#[link(name = "widmann",
       vers = "0.1-pre",
       url = "")];

#[crate_type = "lib"];
#[link_args = "-lpcre"];

extern mod pcre;
extern mod http;
extern mod extra;
extern mod knob;

pub mod application;