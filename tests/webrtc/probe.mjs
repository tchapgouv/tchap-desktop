// Probe the Tchap test build through its test hook (TCHAP_WEBRTC_E2E_*): is the shim installed,
// is the login patch embedded, and what happens when the login form is submitted.
// Usage: node probe.mjs
import { WebSocketServer } from 'ws';
import { spawn } from 'node:child_process';
import { writeFileSync } from 'node:fs';
import path from 'node:path';
const here = path.dirname(new URL(import.meta.url).pathname);
const bin = path.join(here, '../../src-tauri/target/debug/tchap-desktop');
const wss = new WebSocketServer({ host: '127.0.0.1', port: 0 });
await new Promise((r) => wss.on('listening', r));
const url = `ws://127.0.0.1:${wss.address().port}`;
const script = path.join(here, 'probe-page.js');
writeFileSync(script, `
window.__probe = () => {
  const sock = new WebSocket(window.__E2E_WS__);
  const send = (o) => { try { sock.send(JSON.stringify(o)); } catch {} };
  const realFetch = window.fetch.bind(window);
  window.fetch = async (input, init) => {
    const u = String(input && input.url ? input.url : input);
    try { const r = await realFetch(input, init); if (!/\\/sync|\\/bundles\\//.test(u)) send({ t: 'fetch', u: u.slice(0, 160), status: r.status }); return r; }
    catch (e) { send({ t: 'fetch', u: u.slice(0, 160), error: String(e) }); throw e; }
  };
  window.addEventListener('error', (e) => send({ t: 'error', msg: String(e.message).slice(0, 300) }));
  window.addEventListener('unhandledrejection', (e) => send({ t: 'rejection', msg: String(e.reason && (e.reason.stack || e.reason)).slice(0, 400) }));
  const origErr = console.error; console.error = (...a) => { send({ t: 'console.error', msg: a.map(String).join(' ').slice(0, 300) }); origErr(...a); };
  sock.onopen = async () => {
    const rtc = window.RTCPeerConnection;
    let patched = null; try { patched = (await (await realFetch('/bundles/30986244ff9fc5484916/4099.js')).text()).includes('e.base_url||"https://matrix."'); } catch (e) { patched = String(e); }
    send({ t: 'hello', shim: window.__TAURI_WEBRTC__, rtcIsFunction: typeof rtc === 'function', rtcSrc: rtc ? String(rtc).slice(0, 80) : null, bundlePatched: patched,
      ua: navigator.userAgent, gum: typeof navigator.mediaDevices?.getUserMedia, webcodecs: typeof VideoEncoder, worklet: typeof AudioWorkletNode, href: location.href });
    const setVal = (el, v) => { const d = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value'); d.set.call(el, v); el.dispatchEvent(new Event('input', { bubbles: true })); el.dispatchEvent(new Event('change', { bubbles: true })); };
    let n = 0; let submitted = false; let lastHash = '';
    const tick = () => {
      const inputs = [...document.querySelectorAll('input')];
      const pw = inputs.find((i) => i.type === 'password');
      const email = inputs.find((i) => i.type === 'email' || i.type === 'text');
      if (!pw && email && !submitted) { setVal(email, "tchap@example.test"); setTimeout(() => { const f = email.closest("form"); const btn = f && f.querySelector("button[type=submit],input[type=submit]"); if (btn) btn.click(); else if (f) f.requestSubmit(); send({ t: "submit-email", form: !!f, btn: btn ? (btn.textContent || btn.value).trim() : null, inputs: inputs.map((i) => ({ type: i.type, name: i.name, id: i.id, ph: i.placeholder })), buttons: [...document.querySelectorAll("button")].map((b) => b.textContent.trim()).filter(Boolean).slice(0, 10) }); }, 300); submitted = "email"; }
      if (submitted !== true && pw && email) {
        send({ t: 'dom', inputs: inputs.map((i) => ({ type: i.type, name: i.name, id: i.id, ph: i.placeholder })), buttons: [...document.querySelectorAll('button,[role=button],input[type=submit]')].map((b) => (b.textContent || b.value).trim()).filter(Boolean).slice(0, 12) });
        setVal(email, 'tchap@example.test'); setVal(pw, 'pw-tchap');
        setTimeout(() => {
          const btn = [...document.querySelectorAll('button,input[type=submit]')].find((b) => b.type === 'submit' || /connexion|connecter|sign in|log in/i.test(b.textContent || b.value));
          if (btn) { btn.click(); send({ t: 'submit', via: (btn.textContent || btn.value).trim() }); } else { const f = pw.closest('form'); if (f) { f.requestSubmit(); send({ t: 'submit', via: 'form' }); } else send({ t: 'submit', via: 'none' }); }
        }, 300);
        submitted = true;
      }
      if (location.hash === "#/welcome" && !window.__wentLogin) { window.__wentLogin = true; location.hash = "#/login"; }
      if (location.hash !== lastHash) { lastHash = location.hash; send({ t: 'hash', hash: lastHash }); }
      const errs = [...document.querySelectorAll('[class*="rror"], [role=alert]')].map((e) => e.textContent.trim()).filter(Boolean);
      if (errs.length) send({ t: 'ui-errors', errs: [...new Set(errs)].slice(0, 5) });
      if (++n < 90) setTimeout(tick, 500); else send({ t: 'end', hash: location.hash, body: document.body.innerText.slice(0, 600) });
    };
    tick();
  };
};`);
const app = spawn(bin, [], { env: { ...process.env, TCHAP_WEBRTC_FORCE_SHIM: '1', TCHAP_WEBRTC_E2E_WS: url, TCHAP_WEBRTC_E2E_SCRIPTS: script, TCHAP_WEBRTC_E2E_BOOT: 'window.__probe()' }, stdio: ['ignore', 'ignore', 'pipe'] });
let err = ''; app.stderr.on('data', (d) => { err += d; });
await new Promise((res) => {
  const seen = new Set();
  wss.on('connection', (c) => c.on('message', (m) => { const s = String(m); if (seen.has(s)) return; seen.add(s); console.log(s); if (s.includes('"t":"end"') || s.includes('"hash":"#/home"')) setTimeout(res, 3000); }));
  setTimeout(res, 60000);
});
app.kill('SIGTERM');
if (err.trim()) console.error('[app stderr]', err.slice(0, 1500));
process.exit(0);
