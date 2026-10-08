// M4 fixtures, alongside fixtures.mjs: livekit-server, a LiveKit JWT service
// (a Node port of lk-jwt-service's /sfu/get) and Element Call, all on loopback.
// Usage: node m4-fixtures.mjs
import { spawn } from 'node:child_process';
import { createServer } from 'node:http';
import { existsSync, mkdirSync, writeFileSync, createReadStream, statSync } from 'node:fs';
import path from 'node:path';
import net from 'node:net';
import { AccessToken } from 'livekit-server-sdk';

const here = path.dirname(new URL(import.meta.url).pathname);
const state = path.join(here, '.fixtures'); // created by setup.mjs
const ports = { hs: 8008, hsDirect: 8018, jwt: 8009, call: 8081, lk: 7880, lkTcp: 7881 };
const lkKey = 'devkey', lkSecret = 'tchap-mac-e2e-livekit-secret-32-bytes!';
const children = [];
const stop = () => { for (const c of children) try { c.kill('SIGTERM'); } catch {} process.exit(0); };
process.on('SIGINT', stop); process.on('SIGTERM', stop);

// ------------------------------------------------------------- livekit-server
const lkDir = path.join(state, 'livekit'); mkdirSync(lkDir, { recursive: true });
writeFileSync(path.join(lkDir, 'livekit.yaml'), `port: ${ports.lk}
bind_addresses: ["127.0.0.1"]
rtc:
  tcp_port: ${ports.lkTcp}
  port_range_start: 50300
  port_range_end: 50400
  use_external_ip: false
  node_ip: 127.0.0.1
keys:
  ${lkKey}: ${lkSecret}
logging:
  level: info
  json: false
room:
  auto_create: true
`);
const lk = spawn('livekit-server', ['--config', path.join(lkDir, 'livekit.yaml')], { stdio: ['ignore', 'inherit', 'inherit'] });
children.push(lk);

// ------------------------------------------------------------- JWT service (lk-jwt-service shape)
// POST /sfu/get { room, openid_token: { access_token, matrix_server_name }, device_id } -> { url, jwt }
const cors = { 'access-control-allow-origin': '*', 'access-control-allow-headers': '*', 'access-control-allow-methods': 'GET, POST, OPTIONS' };
const federation = (serverName) => (serverName === 'localhost' ? `http://127.0.0.1:${ports.hsDirect}` : `https://${serverName}`);
createServer(async (req, res) => {
  if (req.method === 'OPTIONS') { res.writeHead(204, cors); res.end(); return; }
  if (req.url === '/healthz') { res.writeHead(200, cors); res.end('ok'); return; }
  if (req.method !== 'POST' || !req.url.startsWith('/sfu/get')) { res.writeHead(404, cors); res.end(); return; }
  let body = ''; for await (const c of req) body += c;
  try {
    const { room, openid_token: tok, device_id } = JSON.parse(body);
    const r = await fetch(`${federation(tok.matrix_server_name)}/_matrix/federation/v1/openid/userinfo?access_token=${encodeURIComponent(tok.access_token)}`);
    if (!r.ok) throw new Error(`openid userinfo ${r.status}`);
    const { sub } = await r.json();
    const at = new AccessToken(lkKey, lkSecret, { identity: `${sub}:${device_id}`, ttl: '1h' });
    at.addGrant({ room, roomJoin: true, roomCreate: true, canPublish: true, canSubscribe: true, canPublishData: true });
    const jwt = await at.toJwt();
    console.log(`[jwt] ${sub} device ${device_id} -> room ${room}`);
    res.writeHead(200, { 'content-type': 'application/json', ...cors });
    res.end(JSON.stringify({ url: `ws://127.0.0.1:${ports.lk}`, jwt }));
  } catch (e) {
    console.error('[jwt]', e.message);
    res.writeHead(500, { 'content-type': 'application/json', ...cors }); res.end(JSON.stringify({ errcode: 'M_UNKNOWN', error: e.message }));
  }
}).listen(ports.jwt, '127.0.0.1');

// ------------------------------------------------------------- Element Call (SPA)
const ecDir = path.join(state, 'element-call');
writeFileSync(path.join(ecDir, 'config.json'), JSON.stringify({
  default_server_config: { 'm.homeserver': { base_url: `http://127.0.0.1:${ports.hs}`, server_name: 'localhost' } },
  livekit: { livekit_service_url: `http://127.0.0.1:${ports.jwt}` },
  matrix_rtc_session: { wait_for_key_rotation_ms: 5000 },
}, null, 2));
const types = { '.html': 'text/html', '.js': 'text/javascript', '.mjs': 'text/javascript', '.css': 'text/css', '.json': 'application/json', '.wasm': 'application/wasm', '.svg': 'image/svg+xml', '.png': 'image/png', '.woff2': 'font/woff2', '.woff': 'font/woff', '.ttf': 'font/ttf', '.ico': 'image/x-icon', '.map': 'application/json', '.mp3': 'audio/mpeg', '.ogg': 'audio/ogg' };
createServer((req, res) => {
  let p = decodeURIComponent(new URL(req.url, 'http://x').pathname);
  let f = path.join(ecDir, p);
  if (!f.startsWith(ecDir) || !existsSync(f) || statSync(f).isDirectory()) f = path.join(ecDir, 'index.html'); // SPA routes
  res.writeHead(200, { 'content-type': types[path.extname(f)] || 'application/octet-stream', 'cache-control': 'no-store', ...cors });
  createReadStream(f).pipe(res);
}).listen(ports.call, '127.0.0.1');

const open = (port) => new Promise((r) => { const s = net.connect(port, '127.0.0.1', () => { s.destroy(); r(true); }); s.on('error', () => r(false)); });
for (let i = 0; i < 100 && !(await open(ports.lk)); i++) await new Promise((r) => setTimeout(r, 100));
console.log(`
m4 fixtures up (Ctrl-C stops them)
  livekit       ws://127.0.0.1:${ports.lk}  (tcp ${ports.lkTcp}, udp 50300-50400)
  jwt service   http://127.0.0.1:${ports.jwt}/sfu/get
  element call  http://127.0.0.1:${ports.call}/   (Element Web and Tchap configs point here)
`);
await new Promise(() => {});
