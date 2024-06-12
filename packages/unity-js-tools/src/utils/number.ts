export const toUint64 = (data: Uint8Array) =>
  new DataView(data.buffer, data.byteOffset, data.byteLength).getBigUint64(0);

export const clamp = (n: number) => {
  if (n < 0) return 0;
  if (n > 255) return 255;
  return n;
};
