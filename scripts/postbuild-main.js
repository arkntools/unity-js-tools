const { readFileSync, writeFileSync, copyFileSync } = require('fs');
const { resolve, join } = require('path');

const pkgPath = resolve(__dirname, '../packages/unity-js-tools');
const distPath = join(pkgPath, 'dist');
const packageJsonPath = join(pkgPath, 'package.json');
const wasmPackageJsonPath = resolve(__dirname, '../packages/unity-js-tools-wasm/package.json');

const packageJson = JSON.parse(readFileSync(packageJsonPath).toString());
const { version } = JSON.parse(readFileSync(wasmPackageJsonPath).toString());

packageJson.version = version;
packageJson.dependencies['@arkntools/unity-js-tools-wasm'] = version;

writeFileSync(join(distPath, 'package.json'), JSON.stringify(packageJson, undefined, 2));

['LICENSE', 'README.md'].forEach(file => {
  copyFileSync(resolve(__dirname, '..', file), join(distPath, file));
});
