extern crate wasm_bindgen;
extern crate qrcode_generator;

use wasm_bindgen::prelude::*;
use qrcode_generator::QrCodeEcc;

#[wasm_bindgen]
pub fn convert(url: &str) -> String {
  let result: String = qrcode_generator::to_svg_to_string(
    url,
    QrCodeEcc::Low,
    1024,
    None
  ).unwrap();

  return result;
}
