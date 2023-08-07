use lz4_flex::decompress;
use texture2ddecoder::decode_astc;
use wasm_bindgen::prelude::*;

fn to_js_err(e: impl ToString) -> JsError {
    JsError::new(&e.to_string())
}

fn image_to_rgba(image: Vec<u32>, width: usize, height: usize) -> Vec<u8> {
    let mut lines: Vec<&[u32]> = Vec::with_capacity(height);
    for i in 0..height {
        let start = i * width;
        lines.push(&image[start..start + width]);
    }
    lines
        .iter()
        .copied()
        .rev()
        .flatten()
        .flat_map(|x| {
            let v = x.to_le_bytes();
            [v[2], v[1], v[0], v[3]]
        })
        .collect::<Vec<u8>>()
}

#[wasm_bindgen(js_name = decodeAstc)]
pub fn export_decode_astc(
    data: &[u8],
    width: usize,
    height: usize,
    block_width: usize,
    block_height: usize,
) -> Result<Vec<u8>, JsError> {
    let mut image: Vec<u32> = vec![0; (width * height) as usize];

    let result = decode_astc(data, width, height, block_width, block_height, &mut image);

    match result {
        Ok(()) => Ok(image_to_rgba(image, width, height)),
        Err(e) => Err(to_js_err(e)),
    }
}

#[wasm_bindgen(js_name = decompressLz4)]
pub fn export_decompress_lz4(data: &[u8], size: usize) -> Result<Vec<u8>, JsError> {
    decompress(data, size).map_err(to_js_err)
}
