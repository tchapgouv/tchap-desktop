// Points the fetched tchap-web build (src/) at the local Synapse for the call
// checks. src/ is a build input that fetch-package regenerates, so run this
// again after every `npm run fetch-package`, then rebuild.
// Usage: node patch-tchap-web.mjs [repo root, default ../..]
import { readFileSync, writeFileSync, readdirSync } from 'node:fs';
import path from 'node:path';
const root = path.resolve(process.argv[2] || path.join(path.dirname(new URL(import.meta.url).pathname), '../..'));
const src = path.join(root, 'src');
const hs = 'http://127.0.0.1:8008';
const changes = [];

// 1. config.json: one homeserver, the local Synapse behind the proxy.
const cfgPath = path.join(src, 'config.json');
const cfg = JSON.parse(readFileSync(cfgPath, 'utf8'));
cfg.default_server_config = { 'm.homeserver': { base_url: hs, server_name: 'Local' }, 'm.identity_server': { base_url: hs } };
cfg.homeserver_list = [{ base_url: hs, server_name: 'Local' }];
cfg.enable_presence_by_hs_url = { [hs]: false };
cfg.element_call = { ...(cfg.element_call || {}), url: 'http://127.0.0.1:8081' };
delete cfg.posthog; // no analytics traffic from the test build
// DM calls: the dev config routes 1:1 calls through Element Call (feature_use_ec_in_dm).
// M2 and M3 test the matrix-js-sdk 1:1 path, so turn that off. Set ['*'] again to
// test Element Call in DMs.
cfg.tchap_features = { ...(cfg.tchap_features || {}), feature_use_ec_in_dm: [] };
writeFileSync(cfgPath, JSON.stringify(cfg, null, 2) + '\n');
changes.push('config.json: default_server_config, homeserver_list, enable_presence_by_hs_url -> ' + hs + '; element_call.url -> http://127.0.0.1:8081');

// 2. The bundle: TchapUtils.fetchHomeserverForEmail builds "https://matrix." + hs
//    from the identity lookup. Let the lookup answer carry base_url instead.
const needle = 'const t="https://matrix."+e.hs;';
const patched = 'const t=e.base_url||"https://matrix."+e.hs;';
let hits = 0;
for (const dir of readdirSync(path.join(src, 'bundles'))) {
  for (const f of readdirSync(path.join(src, 'bundles', dir))) {
    if (!f.endsWith('.js')) continue;
    const p = path.join(src, 'bundles', dir, f);
    const s = readFileSync(p, 'utf8');
    if (s.includes(patched)) { hits++; continue; }
    if (!s.includes(needle)) continue;
    writeFileSync(p, s.replace(needle, patched));
    hits++;
    changes.push(`bundles/${dir}/${f}: fetchHomeserverForEmail honours base_url from the identity lookup`);
  }
}
if (!hits) throw new Error('fetchHomeserverForEmail not found in the bundles; tchap-web changed');
console.log(changes.join('\n') || 'already patched');

// 3. Login screen: tchap-web routes #/login to #/email-precheck-sso, which needs
//    an OIDC (MAS) homeserver. Send those links to the classic password login
//    instead, which still exists in the bundle (Login.onPasswordLogin).
{
  const files = [];
  for (const dir of readdirSync(path.join(src, 'bundles'))) for (const f of readdirSync(path.join(src, 'bundles', dir))) if (f.endsWith('.js')) files.push(path.join(src, 'bundles', dir, f));
  for (const f of readdirSync(src)) if (f.endsWith('.html')) files.push(path.join(src, f));
  let n = 0;
  for (const p of files) {
    const s = readFileSync(p, 'utf8');
    if (!s.includes('#/email-precheck-sso')) continue;
    writeFileSync(p, s.split('#/email-precheck-sso').join('#/login'));
    n++; changes.push(`${path.relative(src, p)}: "#/email-precheck-sso" links and redirects -> "#/login"`);
  }
  if (n) console.log(changes.slice(-n).join('\n')); else console.log('login redirect: already patched');
}

// 4. MatrixChat.showScreen("login") dispatches the pre-check action. Dispatch
//    the classic start_login instead.
{
  const needle4 = '"login"===e)x.A.dispatch({action:"email_precheck_sso"})';
  const patched4 = '"login"===e)x.A.dispatch({action:"start_login",params:t})';
  let n = 0;
  for (const dir of readdirSync(path.join(src, 'bundles'))) for (const f of readdirSync(path.join(src, 'bundles', dir))) {
    if (!f.endsWith('.js')) continue;
    const p = path.join(src, 'bundles', dir, f); const s = readFileSync(p, 'utf8');
    if (s.includes(patched4)) { n++; continue; }
    if (!s.includes(needle4)) continue;
    writeFileSync(p, s.replace(needle4, patched4)); n++;
    changes.push(`bundles/${dir}/${f}: showScreen("login") dispatches start_login instead of email_precheck_sso`);
    console.log(changes[changes.length - 1]);
  }
  if (!n) throw new Error('showScreen login dispatch not found; tchap-web changed');
}
