extern crate aiid;
extern crate wasm_bindgen;
extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::prelude::*;

pub type JsResult<T> = Result<T, JsValue>;
macro_rules! jserr {
    ($code:expr) => {
        match $code {
            Ok(v) => Ok(v),
            Err(e) => Err(JsValue::from_str(&format!("{:?}", e))),
        }
    };
}

#[wasm_bindgen]
pub struct Encoding(aiid::aiidEncoding);

#[wasm_bindgen]
impl Encoding {
    #[wasm_bindgen(constructor)]
    pub fn new(encoding_name: &str) -> JsResult<Encoding> {
        Ok(Encoding(jserr!(aiid::aiidEncoding::with_kind(encoding_name))?))
    }

    pub fn encode(&self, data: &[u8]) -> JsResult<String> {
        jserr!(self.0.encode(data))
    }

    pub fn decode(&self, data: &str) -> JsResult<Vec<u8>> {
        jserr!(self.0.decode(data))
    }

    pub fn is_corrupt(&self, data: &str) -> JsResult<bool> {
        jserr!(self.0.is_corrupt(data))
    }
}
