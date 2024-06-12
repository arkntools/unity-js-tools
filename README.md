# unity-js-tools

[![NPM version](https://img.shields.io/npm/v/@arkntools/unity-js-tools?style=flat-square)](https://www.npmjs.com/package/@arkntools/unity-js-tools)

Some tools for [unity-js](https://github.com/arkntools/unity-js), compiled from Rust to WASM.

## Include

- [PSeitz/lz4_flex](https://github.com/PSeitz/lz4_flex)
- [UniversalGameExtraction/texture2ddecoder](https://github.com/UniversalGameExtraction/texture2ddecoder)

## Notice

wasm-bindgen seems to have some kind of flaw that causes bit operations to be incorrect in rare cases.

This causes some texture decoding methods need to be manually implemented some parts to modify its results.

The following methods have been fixed:

- `decodeEtc2Rgba8`

Be careful, maybe there will be errors in other methods but have not been found yet.
