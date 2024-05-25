# unity-js-tools

[![NPM version](https://img.shields.io/npm/v/@arkntools/unity-js-tools?style=flat-square)](https://www.npmjs.com/package/@arkntools/unity-js-tools)

一些给 [unity-js](https://github.com/arkntools/unity-js) 用的工具方法，从 rust 编译为 wasm

## 包含功能

- [PSeitz/lz4_flex](https://github.com/PSeitz/lz4_flex)
- [UniversalGameExtraction/texture2ddecoder](https://github.com/UniversalGameExtraction/texture2ddecoder)

## 注意

wasm-bindgen 貌似有某种缺陷，rust 中极少数位运算场合编译成 wasm 后运算结果不正确，这导致 `decodeEtc2Rgba8` 解码后 alpha 通道数值错误，需要手动实现来修正，不排除还有其他情况有问题
