(g) => {
  const names = [
    "RTCPeerConnection",
    "webkitRTCPeerConnection",
    "RTCDataChannel",
    "RTCIceCandidate",
    "RTCSessionDescription",
    "RTCRtpSender",
    "RTCRtpReceiver",
    "RTCRtpTransceiver",
    "RTCDtlsTransport",
    "RTCIceTransport",
    "RTCSctpTransport",
    "RTCCertificate",
    "RTCPeerConnectionIceEvent",
    "RTCTrackEvent",
    "RTCDataChannelEvent",
  ];
  for (const name of names) {
    try {
      delete g[name];
    } catch (e) {}
    if (name in g) Object.defineProperty(g, name, { value: undefined, configurable: true, writable: true });
  }
  const devices = g.navigator && g.navigator.mediaDevices;
  if (!devices) return;
  const refuse = () => Promise.reject(new DOMException("not allowed", "NotAllowedError"));
  for (const name of ["getUserMedia", "getDisplayMedia", "enumerateDevices"]) {
    if (name in devices) Object.defineProperty(devices, name, { value: refuse, configurable: true, writable: true });
  }
}
