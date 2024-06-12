"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.clamp = exports.toUint64 = void 0;
const toUint64 = (data) => new DataView(data.buffer, data.byteOffset, data.byteLength).getBigUint64(0);
exports.toUint64 = toUint64;
const clamp = (n) => {
    if (n < 0)
        return 0;
    if (n > 255)
        return 255;
    return n;
};
exports.clamp = clamp;
