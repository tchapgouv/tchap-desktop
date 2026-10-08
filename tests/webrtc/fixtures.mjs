// Local fixtures for the M2, M3 and M4 checks on one machine. Everything binds
// to loopback. Starts Synapse, coturn, a proxy that answers Tchap's identity
// lookup, and a static server for Element Web. Creates two users and a DM.
//
// Usage: node setup.mjs once, then node fixtures.mjs   (Ctrl-C stops everything)
//   SYNAPSE_PY  Python with matrix-synapse (default: .fixtures/synapse-venv/bin/python)
//   TURN_IP     address coturn listens and relays on (default 127.0.0.1)
import { spawn, execFileSync } from 'node:child_process';
import { createServer, request as httpRequest } from 'node:http';
import { createHmac, randomBytes } from 'node:crypto';
import { existsSync, mkdirSync, readFileSync, writeFileSync, appendFileSync, createReadStream, statSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import net from 'node:net';

const here = path.dirname(fileURLToPath(import.meta.url));
const state = path.join(here, '.fixtures'); // created by setup.mjs
const py = process.env.SYNAPSE_PY || path.join(state, 'synapse-venv/bin/python');
const turnIp = process.env.TURN_IP || '127.0.0.1';
const ports = { proxy: 8008, synapse: 8018, element: 8080, turn: 3478, jwt: 8009, call: 8081 };
const secrets = { registration: 'tchap-mac-e2e-registration', turn: 'tchap-mac-e2e-turn' };
const users = {
  tchap: { email: 'tchap@example.test', password: 'pw-tchap', admin: true },
  element: { email: 'element@example.test', password: 'pw-element', admin: false },
};
const children = [];
const stop = () => { for (const c of children) try { c.kill('SIGTERM'); } catch {} process.exit(0); };
process.on('SIGINT', stop); process.on('SIGTERM', stop);
const portOpen = (port, host = '127.0.0.1') => new Promise((res) => { const s = net.connect(port, host, () => { s.destroy(); res(true); }); s.on('error', () => res(false)); });
const waitPort = async (port, what, tries = 600) => { for (let i = 0; i < tries; i++) { if (await portOpen(port)) return; await new Promise((r) => setTimeout(r, 100)); } throw new Error(`${what} did not open port ${port}`); };

// ------------------------------------------------------------- Synapse
const hsDir = path.join(state, 'synapse');
const hsYaml = path.join(hsDir, 'homeserver.yaml');
if (!existsSync(hsYaml)) {
  mkdirSync(hsDir, { recursive: true });
  execFileSync(py, ['-m', 'synapse.app.homeserver', '--server-name', 'localhost', '--config-path', hsYaml,
    '--data-directory', hsDir, '--generate-config', '--report-stats=no'], { stdio: 'inherit' });
  appendFileSync(hsYaml, `
# tchap-mac-e2e overrides (the proxy on ${ports.proxy} fronts this listener)
listeners:
  - port: ${ports.synapse}
    bind_addresses: ['127.0.0.1']
    type: http
    x_forwarded: true
    resources:
      - names: [client, federation]
        compress: false
public_baseurl: "http://127.0.0.1:${ports.proxy}/"
serve_server_wellknown: true
registration_shared_secret: "${secrets.registration}"
enable_registration: false
rc_message: { per_second: 1000, burst_count: 1000 }
rc_login:
  address: { per_second: 1000, burst_count: 1000 }
  account: { per_second: 1000, burst_count: 1000 }
  failed_attempts: { per_second: 1000, burst_count: 1000 }
rc_joins:
  local: { per_second: 1000, burst_count: 1000 }
rc_invites: { per_room: { per_second: 1000, burst_count: 1000 }, per_user: { per_second: 1000, burst_count: 1000 } }
user_directory: { enabled: true, search_all_users: true }
# TURN: coturn with a shared secret, so /voip/turnServer hands out credentials
turn_uris: ["turn:${turnIp}:${ports.turn}?transport=udp", "turn:${turnIp}:${ports.turn}?transport=tcp"]
turn_shared_secret: "${secrets.turn}"
turn_user_lifetime: 86400000
turn_allow_guests: true
# M4: Element Call finds the LiveKit JWT service here (lk-jwt.mjs)
extra_well_known_client_content:
  org.matrix.msc4143.rtc_foci:
    - type: livekit
      livekit_service_url: "http://127.0.0.1:${ports.jwt}"
`);
}
const hs = spawn(py, ['-m', 'synapse.app.homeserver', '-c', hsYaml], { cwd: hsDir, stdio: ['ignore', 'ignore', 'inherit'] });
children.push(hs);

// ------------------------------------------------------------- coturn
const turnDir = path.join(state, 'coturn');
mkdirSync(turnDir, { recursive: true });
writeFileSync(path.join(turnDir, 'turnserver.conf'), [
  `listening-ip=${turnIp}`, `relay-ip=${turnIp}`, `listening-port=${ports.turn}`,
  'fingerprint', 'use-auth-secret', `static-auth-secret=${secrets.turn}`, 'realm=localhost',
  'no-tls', 'no-dtls', 'no-cli', 'min-port=50000', 'max-port=50200',
  `log-file=${turnDir}/turn.log`, 'simple-log', `pidfile=${turnDir}/turn.pid`, `userdb=${turnDir}/turndb`,
].join('\n') + '\n');
const turn = spawn('turnserver', ['-c', path.join(turnDir, 'turnserver.conf')], { stdio: ['ignore', 'ignore', 'inherit'] });
children.push(turn);

// ------------------------------------------------------------- proxy: Tchap's identity lookup, then Synapse
// tchap-web asks <homeserver>/_matrix/identity/api/v1/info?medium=email&address=<email>
// and expects { hs }. The patched bundle also honours base_url (see patch-tchap-web.mjs).
const proxy = createServer((req, res) => {
  const cors = { 'access-control-allow-origin': '*', 'access-control-allow-headers': '*', 'access-control-allow-methods': 'GET, POST, PUT, DELETE, OPTIONS' };
  // MatrixRTC transport discovery for Element Call (MSC4143): the unstable endpoint
  // and the client well-known, both pointing at the JWT service of m4-fixtures.mjs.
  const foci = [{ type: 'livekit', livekit_service_url: `http://127.0.0.1:${ports.jwt}` }];
  if (req.url.startsWith('/_matrix/client/unstable/org.matrix.msc4143/rtc/transports')) {
    res.writeHead(200, { 'content-type': 'application/json', ...cors }); res.end(JSON.stringify({ rtc_transports: foci })); return;
  }
  if (req.url.startsWith('/.well-known/matrix/client')) {
    res.writeHead(200, { 'content-type': 'application/json', ...cors }); res.end(JSON.stringify({ 'm.homeserver': { base_url: `http://127.0.0.1:${ports.proxy}` }, 'org.matrix.msc4143.rtc_foci': foci })); return;
  }
  if (req.url.startsWith('/_matrix/identity/api/v1/info')) {
    res.writeHead(200, { 'content-type': 'application/json', ...cors });
    res.end(JSON.stringify({ hs: 'localhost', base_url: `http://127.0.0.1:${ports.proxy}` }));
    return;
  }
  const up = httpRequest({ host: '127.0.0.1', port: ports.synapse, method: req.method, path: req.url, headers: { ...req.headers, host: `127.0.0.1:${ports.proxy}` } }, (r) => {
    res.writeHead(r.statusCode, r.headers); r.pipe(res);
  });
  up.on('error', (e) => { res.writeHead(502, cors); res.end(String(e)); });
  req.pipe(up);
});
proxy.listen(ports.proxy, '127.0.0.1');

// ------------------------------------------------------------- Element Web (Chrome side)
const ewDir = path.join(state, 'element-web');
writeFileSync(path.join(ewDir, 'config.json'), JSON.stringify({
  default_server_config: {
    'm.homeserver': { base_url: `http://127.0.0.1:${ports.proxy}`, server_name: 'localhost' },
  },
  brand: 'Element (local)', disable_custom_urls: false, disable_guests: true, default_country_code: 'FR',
  show_labs_settings: true,
  features: { feature_group_calls: true },
  element_call: { url: `http://127.0.0.1:${ports.call}`, use_exclusively: true },
  setting_defaults: { breadcrumbs: true },
  room_directory: { servers: ['localhost'] },
}, null, 2));
const types = { '.html': 'text/html', '.js': 'text/javascript', '.mjs': 'text/javascript', '.css': 'text/css', '.json': 'application/json', '.wasm': 'application/wasm', '.svg': 'image/svg+xml', '.png': 'image/png', '.woff2': 'font/woff2', '.woff': 'font/woff', '.ttf': 'font/ttf', '.ico': 'image/x-icon', '.map': 'application/json' };
const serveStatic = (dir) => (req, res) => {
  let p = decodeURIComponent(new URL(req.url, 'http://x').pathname);
  if (p.endsWith('/')) p += 'index.html';
  const f = path.join(dir, p);
  if (!f.startsWith(dir) || !existsSync(f) || statSync(f).isDirectory()) { res.writeHead(404); res.end(); return; }
  res.writeHead(200, { 'content-type': types[path.extname(f)] || 'application/octet-stream', 'cache-control': 'no-store' });
  createReadStream(f).pipe(res);
};
createServer(serveStatic(ewDir)).listen(ports.element, '127.0.0.1');

// ------------------------------------------------------------- users and a DM room
await waitPort(ports.synapse, 'synapse');
const hsUrl = `http://127.0.0.1:${ports.proxy}`;
const api = async (method, p, body, token) => {
  const r = await fetch(hsUrl + p, { method, headers: { 'content-type': 'application/json', ...(token ? { authorization: `Bearer ${token}` } : {}) }, body: body ? JSON.stringify(body) : undefined });
  const j = await r.json().catch(() => ({}));
  if (!r.ok) throw new Error(`${method} ${p}: ${r.status} ${JSON.stringify(j)}`);
  return j;
};
async function ensureUser(name, u) {
  const { nonce } = await api('GET', '/_synapse/admin/v1/register');
  const mac = createHmac('sha1', secrets.registration).update(`${nonce}\0${name}\0${u.password}\0${u.admin ? 'admin' : 'notadmin'}`).digest('hex');
  try { await api('POST', '/_synapse/admin/v1/register', { nonce, username: name, password: u.password, admin: u.admin, mac }); }
  catch (e) { if (!/User ID already taken/.test(e.message)) throw e; }
  const l = await api('POST', '/_matrix/client/v3/login', { type: 'm.login.password', identifier: { type: 'm.id.user', user: name }, password: u.password, initial_device_display_name: 'fixtures' });
  return { userId: l.user_id, token: l.access_token };
}
const tchap = await ensureUser('tchap', users.tchap);
const element = await ensureUser('element', users.element);
// Email threepids: Tchap logs in with { type: m.id.thirdparty, medium: email }.
for (const [name, u] of Object.entries(users)) {
  await api('PUT', `/_synapse/admin/v2/users/@${name}:localhost`, { threepids: [{ medium: 'email', address: u.email }], displayname: name }, tchap.token);
}
// One DM room between them, marked in m.direct on both sides.
const dmState = path.join(hsDir, 'dm-room.json');
let roomId = existsSync(dmState) ? JSON.parse(readFileSync(dmState, 'utf8')).roomId : null;
if (!roomId) {
  ({ room_id: roomId } = await api('POST', '/_matrix/client/v3/createRoom', { is_direct: true, preset: 'trusted_private_chat', invite: [tchap.userId] }, element.token));
  await api('POST', `/_matrix/client/v3/join/${encodeURIComponent(roomId)}`, {}, tchap.token);
  await api('PUT', `/_matrix/client/v3/user/${encodeURIComponent(element.userId)}/account_data/m.direct`, { [tchap.userId]: [roomId] }, element.token);
  await api('PUT', `/_matrix/client/v3/user/${encodeURIComponent(tchap.userId)}/account_data/m.direct`, { [element.userId]: [roomId] }, tchap.token);
  writeFileSync(dmState, JSON.stringify({ roomId }));
}
const turnInfo = await api('GET', '/_matrix/client/v3/voip/turnServer', null, tchap.token);

console.log(`
fixtures up (Ctrl-C stops them)
  homeserver   ${hsUrl}   (Synapse ${ports.synapse} behind the identity-lookup proxy)
  turn         ${JSON.stringify(turnInfo.uris)} ttl ${turnInfo.ttl}s
  element web  http://127.0.0.1:${ports.element}/   log in as  element@example.test / ${users.element.password}
  tchap        log in with  ${users.tchap.email} / ${users.tchap.password}
  dm room      ${roomId}
  chrome       open Element Web in Google Chrome, not Playwright's Chromium (no H.264 there)
  m4           run lk-jwt.mjs and livekit-server, and serve Element Call on ${ports.call}, before a group call
`);
await new Promise(() => {});
