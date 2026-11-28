#!/usr/bin/env bash
# Verifies the signature of an APK and, with CERT_SHA256 (the SHA-256 of the release certificate,
# hex with or without colons), that its signer is that certificate. With REQUIRE_CERT=true a missing
# CERT_SHA256 is an error: a published APK is always checked against the release certificate.
#   [CERT_SHA256=<sha256>] [REQUIRE_CERT=true] ANDROID_HOME=<sdk> scripts/apk-signer.sh <apk>
set -euo pipefail
APK=${1:?usage: [CERT_SHA256=<sha256>] [REQUIRE_CERT=true] $0 <apk>}
apksigner=$(find "${ANDROID_HOME:?ANDROID_HOME is not set}/build-tools" -mindepth 2 -maxdepth 2 -name apksigner | sort -V | tail -n 1)
[ -n "$apksigner" ] || { echo "apksigner is not in $ANDROID_HOME/build-tools"; exit 1; }
certs=$("$apksigner" verify --print-certs "$APK" 2>&1) || { echo "the signature of $(basename "$APK") does not verify: $certs"; exit 1; }
# "Signer #1 certificate …" or, per scheme, "V3.0 Signer: certificate …": one certificate in all.
mapfile -t found < <(sed -n 's/^.*[Ss]igner[^:]*:\? certificate SHA-256 digest: //p' <<<"$certs" | sort -u)
[ "${#found[@]}" -eq 1 ] || { echo "$(basename "$APK") has ${#found[@]} signers instead of one"; exit 1; }
want=$(tr -d ': ' <<<"${CERT_SHA256:-}" | tr '[:upper:]' '[:lower:]')
if [ -z "$want" ]; then
  [ "${REQUIRE_CERT:-}" != true ] || { echo "ANDROID_CERT_SHA256 is not set: a published APK is checked against the release certificate"; exit 1; }
  echo "signature verifies (certificate ${found[0]} not compared: no CERT_SHA256)"
  exit 0
fi
[ "${found[0]}" = "$want" ] || { echo "$(basename "$APK") is signed by ${found[0]}, not by the release certificate $want"; exit 1; }
echo "signed by the release certificate"
