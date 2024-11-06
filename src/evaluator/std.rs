use std::collections::HashMap;

use crate::evaluator::formatting::display_value;

use super::value::{create_null, Value};

pub type StdFn = fn(Vec<Value>) -> Value;

pub fn create_std_fn() -> HashMap<&'static str, StdFn> {
  let mut map: HashMap<&'static str, StdFn> = HashMap::new();
  // io
  map.insert("println", |args| {
    let output = args.iter().map(|arg| display_value(arg).to_string()).collect::<Vec<String>>().join("");
    println!("{}", output);
    create_null()
  });
  map.insert("print", |args| {
    let output = args.iter().map(|arg| display_value(arg).to_string()).collect::<Vec<String>>().join(" ");
    println!("{}", output);
    create_null()
  });

  // network
  return map;
}
