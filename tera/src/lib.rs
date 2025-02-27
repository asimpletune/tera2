mod args;
mod context;
mod errors;
mod filters;
mod functions;
#[cfg(feature = "glob_fs")]
mod globbing;
mod parsing;
mod reporting;
mod template;
mod tera;
mod tests;
mod utils;
pub mod value;
pub(crate) mod vm;

pub use crate::tera::{EscapeFn, Tera};
pub use args::Kwargs;
pub use context::Context;
pub use errors::{Error, ErrorKind, TeraResult};
pub use parsing::parser::Parser;
pub use utils::escape_html;
pub use value::number::Number;
pub use value::Value;
pub use vm::state::State;

#[cfg(feature = "fast_hash")]
pub(crate) use ahash::{AHashMap as HashMap, AHashSet as HashSet};
#[cfg(not(feature = "fast_hash"))]
pub(crate) use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod snapshot_tests;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render(template: String, data: JsValue) -> String {
  let obj: HashMap<String, serde_json::Value> = serde_wasm_bindgen::from_value(data).expect("Expected a JSON object");
  let mut ctx = Context::new();
  for (key, value) in obj {
    ctx.insert(key, &value);
  };
  let mut tera = Tera::default();
  return tera.render_str(&template, &ctx).unwrap()
}