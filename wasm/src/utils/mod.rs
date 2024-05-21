pub mod attack;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug,Clone,Copy)]
pub enum Skill {
  Q,W,E,R
}