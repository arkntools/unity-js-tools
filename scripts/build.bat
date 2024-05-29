wasm-pack build --release --target nodejs --scope arkntools --out-name index
wasm-pack build --release --target web --out-name index --out-dir pkg/web
node scripts/process.js
