use std::error::Error;

use astc_decode::{astc_decode, Footprint};
use lz4_flex::decompress;
use wasm_bindgen::prelude::*;

fn to_js_err(e: impl Error) -> JsError {
    JsError::new(&e.to_string())
}

#[wasm_bindgen(js_name = decodeAstc)]
pub fn decode_astc(
    data: &[u8],
    width: u32,
    height: u32,
    block_width: u32,
    block_height: u32,
) -> Result<Vec<u8>, JsError> {
    let footprint = Footprint::new(block_width, block_height);
    let mut result: Vec<u8> = vec![0; (width * height * 4) as usize];

    let astc_result = astc_decode(data, width, height, footprint, |x, y, v4| {
        let y = height - y - 1;
        let ri = ((y * width + x) * 4) as usize;
        for (i, v) in v4.iter().enumerate() {
            result[ri + i] = *v;
        }
    });

    match astc_result {
        Ok(()) => Ok(result),
        Err(e) => Err(to_js_err(e)),
    }
}

#[wasm_bindgen(js_name = decompressLz4)]
pub fn decompress_lz4(data: &[u8], size: usize) -> Result<Vec<u8>, JsError> {
    decompress(data, size).map_err(to_js_err)
}
