mod tools;

use lz4_flex::decompress;
use lz4_flex::block::decompress_size_prepended;
use texture2ddecoder::*;
use tools::*;
use wasm_bindgen::prelude::*;

// ATC
export_decode_texture!("decodeAtcRgb4", decode_atc_rgb4);
export_decode_texture!("decodeAtcRgba8", decode_atc_rgba8);

// BCn
export_decode_texture!("decodeBc1", decode_bc1);
export_decode_texture!("decodeBc3", decode_bc3);
export_decode_texture!("decodeBc4", decode_bc4);
export_decode_texture!("decodeBc5", decode_bc5);
export_decode_texture!("decodeBc6Signed", decode_bc6_signed);
export_decode_texture!("decodeBc6Unsigned", decode_bc6_unsigned);
export_decode_texture!("decodeBc7", decode_bc7);

// ETC
export_decode_texture!("decodeEtc1", decode_etc1);
export_decode_texture!("decodeEtc2Rgb", decode_etc2_rgb);
export_decode_texture!("decodeEtc2Rgba1", decode_etc2_rgba1);
export_decode_texture!("decodeEtc2Rgba8", decode_etc2_rgba8);
export_decode_texture!("decodeEacr", decode_eacr);
export_decode_texture!("decodeEacrSigned", decode_eacr_signed);
export_decode_texture!("decodeEacrg", decode_eacrg);
export_decode_texture!("decodeEacrgSigned", decode_eacrg_signed);

// PVRTC
export_decode_texture!("decodePvrtc2bpp", decode_pvrtc_2bpp);
export_decode_texture!("decodePvrtc4bpp", decode_pvrtc_4bpp);

// ASTC
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
        Ok(()) => Ok(split_channel(image)),
        Err(e) => Err(to_js_err(e)),
    }
}

// LZ4
#[wasm_bindgen(js_name = decompressLz4)]
pub fn export_decompress_lz4(data: &[u8], size: usize) -> Result<Vec<u8>, JsError> {
    decompress(data, size).map_err(to_js_err)
}
#[wasm_bindgen(js_name = decompressLz4SizePrepended)]
pub fn export_decompress_lz4_size_prepended(data: &[u8]) -> Result<Vec<u8>, JsError> {
    decompress_size_prepended(data).map_err(to_js_err)
}
