use wasm_bindgen::prelude::*;

type DecodeFunction = fn(&[u8], usize, usize, &mut [u32]) -> Result<(), &'static str>;

pub fn to_js_err(e: impl ToString) -> JsError {
    JsError::new(&e.to_string())
}

pub fn split_channel(image: Vec<u32>) -> Vec<u8> {
    image
        .into_iter()
        .flat_map(|x| x.to_le_bytes())
        .collect::<Vec<u8>>()
}

pub fn decode_texture(
    data: &[u8],
    width: usize,
    height: usize,
    decode_func: DecodeFunction,
) -> Result<Vec<u8>, JsError> {
    let mut image: Vec<u32> = vec![0; (width * height) as usize];

    let result = decode_func(data, width, height, &mut image);

    match result {
        Ok(()) => Ok(split_channel(image)),
        Err(e) => Err(to_js_err(e)),
    }
}

macro_rules! export_decode_texture {
    ($name: expr, $decode_func: ident) => {
        paste::item! {
            #[wasm_bindgen(js_name = $name)]
            pub fn [<export_ $decode_func>](data: &[u8], width: usize, height: usize) -> Result<Vec<u8>, JsError> {
                decode_texture(data, width, height, $decode_func)
            }
        }
    };
}

pub(crate) use export_decode_texture;
