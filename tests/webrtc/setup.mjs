// One-time setup for the local call checks: a Python venv with Synapse, and the
// Element Web and Element Call builds for the browser side. State lives in
// .fixtures/ (git-ignored). coturn and livekit-server come from the system.
// Usage: npm install && node setup.mjs
import { execFileSync, spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import path from 'node:path';

const here = path.dirname(fileURLToPath(import.meta.url));
const state = path.join(here, '.fixtures');
mkdirSync(state, { recursive: true });
const versions = {
  elementWeb: process.env.ELEMENT_WEB_VERSION || 'v1.12.29',
  elementCall: process.env.ELEMENT_CALL_VERSION || 'v0.26.0',
};

const have = (bin) => spawnSync('sh', ['-c', `command -v ${bin}`], { stdio: 'ignore' }).status === 0;
let missing = false;
for (const [bin, hint] of [['turnserver', 'brew install coturn'], ['livekit-server', 'brew install livekit'], ['uv', 'brew install uv'], ['cargo', 'https://rustup.rs']]) {
  if (!have(bin)) { console.error(`missing ${bin}: ${hint}`); missing = true; }
}
if (missing) process.exit(1);

// Synapse: a Python fixture, not a deliverable.
const venv = path.join(state, 'synapse-venv');
if (!existsSync(path.join(venv, 'bin', 'python'))) {
  console.log('creating', venv);
  execFileSync('uv', ['venv', venv], { stdio: 'inherit' });
}
execFileSync('uv', ['pip', 'install', '--python', path.join(venv, 'bin', 'python'), 'matrix-synapse'], { stdio: 'inherit' });

// Element Web and Element Call release tarballs from GitHub.
async function fetchTarball(name, url, dir) {
  if (existsSync(path.join(dir, 'index.html'))) { console.log(`${name}: present`); return; }
  console.log(`${name}: downloading ${url}`);
  const r = await fetch(url, { redirect: 'follow' });
  if (!r.ok) throw new Error(`${url}: ${r.status}`);
  const tgz = path.join(state, `${name}.tar.gz`);
  writeFileSync(tgz, Buffer.from(await r.arrayBuffer()));
  mkdirSync(dir, { recursive: true });
  execFileSync('tar', ['-xzf', tgz, '-C', dir, '--strip-components=1'], { stdio: 'inherit' });
}
const ew = versions.elementWeb.replace(/^v/, ''), ec = versions.elementCall.replace(/^v/, '');
await fetchTarball('element-web', `https://github.com/element-hq/element-web/releases/download/${versions.elementWeb}/element-v${ew}.tar.gz`, path.join(state, 'element-web'));
await fetchTarball('element-call', `https://github.com/element-hq/element-call/releases/download/${versions.elementCall}/element-call-${ec}.tar.gz`, path.join(state, 'element-call'));

console.log(`
setup done. Next:
  node patch-tchap-web.mjs        # after every 'npm run fetch-package' at the repo root
  (cd ../../src-tauri && cargo build)
  node fixtures.mjs               # then ./run-tchap.sh, and Element Web in Chrome
`);
