use std::hashmap::HashMap;
use pcre::pcre::Match;

#[deriving(Clone)]
pub struct Params {
  get: HashMap<~str, ~str>,
}

impl Params {
  pub fn new(get: HashMap<~str, ~str>) -> Params {
    Params { get: get }
  }

  pub fn from_match(m: Match) -> Params {
    let mut map = HashMap::new();
    let group_names = m.group_names();
    for name in group_names.iter() {
      let group = m.named_group(*name);
      match group {
        Some(str) => { map.insert(name.to_owned(), str.to_owned()); }
        None => { }
      }
    }

    Params::new(map)
  }

  pub fn fetch<K: ToStr, T: FromStr>(&self, key: K) -> Option<T> {
    match self.get.find(&key.to_str()) {
      Some(string) => from_str(string.to_owned()),
      None => None
    }
  }
}
