const { readFileSync, writeFileSync } = require('fs');
const { resolve, join } = require('path');
const { runTransform } = require('esm-to-cjs');

const pkgPath = resolve(__dirname, '../packages/unity-js-tools-wasm');
const webPkgPath = join(pkgPath, 'web');

const wasmBase64 = readFileSync(join(webPkgPath, 'index_bg.wasm')).toString('base64');

function base64Decode(input) {
  return Uint8Array.from(atob(input), m => m.codePointAt(0)).buffer;
}

const content =
  readFileSync(join(webPkgPath, 'index.js'))
    .toString()
    .replace(/\nasync function __wbg_load\([\s\S]+?\n}/, '')
    .replace(/\nasync function __wbg_init\([\s\S]+?\n}/, '')
    .replace(/__wbg_init\.[^;]+?;\s+/, '')
    .replace('export { initSync }\n', '')
    .replace('export default __wbg_init;\n', '') +
  `${base64Decode.toString()}

initSync(base64Decode('${wasmBase64}'));
`;

writeFileSync(
  join(pkgPath, 'index.browser.js'),
  runTransform(content, { quote: 'single', lenIdentifier: 30 }),
);

const packageJsonPath = join(pkgPath, 'package.json');
const packageJson = JSON.parse(readFileSync(packageJsonPath).toString());

packageJson.files.push('index.browser.js');
packageJson.browser = 'index.browser.js';

writeFileSync(packageJsonPath, JSON.stringify(packageJson, undefined, 2));

writeFileSync(
  join(pkgPath, 'README.md'),
  `# unity-js-tools-wasm

See [@arkntools/unity-js-tools](https://www.npmjs.com/package/@arkntools/unity-js-tools).
`,
);
