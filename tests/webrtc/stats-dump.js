// Paste into the devtools console of Tchap (WebKit inspector) or Element Web
// (Chrome) during a 1:1 call. Prints one JSON block per call with the selected
// candidate pair, transport (tlsGroup), codecs, inbound and outbound RTP.
(async () => {
  const client = window.mxMatrixClientPeg.get();
  const calls = [...client.callEventHandler.calls.values()];
  if (!calls.length) { console.warn('no matrix-js-sdk calls'); return; }
  for (const call of calls) {
    const report = await call.peerConn.getStats();
    const all = {}; report.forEach((s) => { all[s.id] = s; });
    const pairs = Object.values(all).filter((s) => s.type === 'candidate-pair' && (s.selected || s.nominated || s.state === 'succeeded'));
    const pick = (p) => p && { state: p.state, nominated: p.nominated, local: all[p.localCandidateId], remote: all[p.remoteCandidateId], rtt: p.currentRoundTripTime, bytesSent: p.bytesSent, bytesReceived: p.bytesReceived };
    const out = {
      callId: call.callId, type: call.type, state: call.state, hangupParty: call.hangupParty,
      when: new Date().toISOString(), ua: navigator.userAgent,
      shim: !!(window.__TAURI_WEBRTC__ && window.__TAURI_WEBRTC__.available),
      selectedPair: pairs.map(pick),
      transport: Object.values(all).filter((s) => s.type === 'transport').map((t) => ({ dtlsState: t.dtlsState, tlsVersion: t.tlsVersion, tlsGroup: t.tlsGroup, dtlsCipher: t.dtlsCipher, srtpCipher: t.srtpCipher })),
      codecs: Object.values(all).filter((s) => s.type === 'codec').map((c) => c.mimeType + (c.sdpFmtpLine ? ' ' + c.sdpFmtpLine : '')),
      inbound: Object.values(all).filter((s) => s.type === 'inbound-rtp').map((s) => ({ kind: s.kind, codec: all[s.codecId]?.mimeType, packetsReceived: s.packetsReceived, packetsLost: s.packetsLost, jitter: s.jitter, jitterBufferDelay: s.jitterBufferDelay, jitterBufferEmittedCount: s.jitterBufferEmittedCount, audioLevel: s.audioLevel, totalAudioEnergy: s.totalAudioEnergy, concealedSamples: s.concealedSamples, concealmentEvents: s.concealmentEvents, framesDecoded: s.framesDecoded, framesDropped: s.framesDropped, frameWidth: s.frameWidth, frameHeight: s.frameHeight, framesPerSecond: s.framesPerSecond })),
      outbound: Object.values(all).filter((s) => s.type === 'outbound-rtp').map((s) => ({ kind: s.kind, codec: all[s.codecId]?.mimeType, packetsSent: s.packetsSent, bytesSent: s.bytesSent, framesEncoded: s.framesEncoded, frameWidth: s.frameWidth, frameHeight: s.frameHeight, framesPerSecond: s.framesPerSecond, keyFramesEncoded: s.keyFramesEncoded })),
    };
    console.log('STATS ' + JSON.stringify(out, null, 1));
  }
})();
