#!/usr/bin/env bash
set -euo pipefail
declare WORK site rid daemon
tls=$WORK/tls
mkdir -p "$tls/r"
openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 -nodes -keyout "$tls/ca.key" -out "$tls/ca.pem" \
  -subj /CN=test-ca -days 2 -addext basicConstraints=critical,CA:TRUE 2>/dev/null
printf 'subjectAltName=IP:127.0.0.1\nbasicConstraints=CA:FALSE\n' >"$tls/leaf.ext"
openssl req -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 -nodes -keyout "$tls/public.key" -out "$tls/public.csr" \
  -subj /CN=127.0.0.1 2>/dev/null
openssl x509 -req -in "$tls/public.csr" -CA "$tls/ca.pem" -CAkey "$tls/ca.key" -CAcreateserial -days 2 \
  -extfile "$tls/leaf.ext" -out "$tls/public.pem" 2>/dev/null
home_cert() {
  openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 -nodes -keyout "$tls/$1.key" -out "$tls/$1.pem" \
    -subj /CN=127.0.0.1 -days 2 -addext subjectAltName=IP:127.0.0.1 2>/dev/null
}
home_cert home
home_cert other
cat >"$tls/serve.py" <<'PY'
import functools, http.server, ssl, sys
port, root, cert, key = int(sys.argv[1]), sys.argv[2], sys.argv[3], sys.argv[4]
handler = functools.partial(http.server.SimpleHTTPRequestHandler, directory=root)
server = http.server.ThreadingHTTPServer(("127.0.0.1", port), handler)
context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
context.load_cert_chain(cert, key)
server.socket = context.wrap_socket(server.socket, server_side=True)
server.serve_forever()
PY
free_port() { python3 -c "import socket; s = socket.socket(); s.bind(('127.0.0.1', 0)); print(s.getsockname()[1])"; }
serving() { python3 -c "import ssl, sys, urllib.request
c = ssl.create_default_context(cafile=sys.argv[2]); c.verify_flags &= ~ssl.VERIFY_X509_STRICT
urllib.request.urlopen(sys.argv[1], context=c)" "$1" "$2"; }
other=$WORK/other-site
mkdir -p "$other"
(
  cd "$other"
  ctl repo keygen root --no-password >/dev/null
  ctl repo keygen online --no-password >/dev/null
  ctl repo init repo --name Other --root-key root.pub --online-key online.pub >/dev/null 2>&1
  ctl repo sign-root repo --key root.key >/dev/null
  ctl repo add repo "$site/dawn.wpak" >/dev/null
  ctl repo publish repo --key online.key >/dev/null
)
ln -s "$site/repo" "$tls/r/wallpapers"
mkdir -p "$tls/srv" "$tls/.well-known"
public=$(free_port) home=$(free_port)
printf '{"name": "Home", "repositories": [{"name": "wallpapers", "url": "https://127.0.0.1:%s/r/wallpapers"}]}' "$public" >"$tls/.well-known/reiwand-server"
python3 "$tls/serve.py" "$public" "$tls" "$tls/public.pem" "$tls/public.key" >"$WORK/tls-public.log" 2>&1 & pids+=($!)
python3 "$tls/serve.py" "$home" "$tls" "$tls/home.pem" "$tls/home.key" >"$WORK/tls-home.log" 2>&1 & homed=$!; pids+=("$homed")
wait_for 10 serving "https://127.0.0.1:$public/r/wallpapers/entry/latest" "$tls/ca.pem" || fail "the https repository did not start"
stop_daemon "$daemon" "$WORK/reiwandd.log"
SSL_CERT_FILE=$tls/ca.pem start_daemon "$WORK/trust.log"
repo_id=$rid
listed() { ctl remote ls --json | jq_ "$1"; }
failing() {
  if ctl remote add "$@" --wait >"$WORK/trust.out" 2>&1; then fail "added: $*: $(cat "$WORK/trust.out")"; fi
}
ctl remote add "https://127.0.0.1:$public/r/wallpapers" --wait >/dev/null
[ "$(listed "[(x['repo_id'] == '$repo_id', x['url'].endswith('/r/wallpapers')) for x in r['repos']]")" = "[(True, True)]" ] ||
  fail "a plain https address: $(ctl remote ls --json)"
ctl remote refresh "${repo_id:0:8}" --wait >/dev/null
ctl remote rm "${repo_id:0:8}"
failing "http://example.org/r/wallpapers"
grep -q "use an https" "$WORK/trust.out" || fail "plain http to another host: $(cat "$WORK/trust.out")"
failing "https://127.0.0.1:$public/srv"
grep -q "this is a server, not a repository" "$WORK/trust.out" || fail "a server address: $(cat "$WORK/trust.out")"
grep -q "wallpapers (https://127.0.0.1:$public/r/wallpapers)" "$WORK/trust.out" || fail "the repositories of the server: $(cat "$WORK/trust.out")"
failing "https://127.0.0.1:$home/r/wallpapers"
grep -q "not from a known authority" "$WORK/trust.out" || fail "an unknown certificate: $(cat "$WORK/trust.out")"
print=$(sed -n 's/.*fingerprint is \([0-9a-f]\{64\}\).*/\1/p' "$WORK/trust.out")
[ -n "$print" ] || fail "the certificate was not offered"
[ "$(listed "len(r['repos'])")" = 0 ] || fail "something was added"
[ ! -e "$XDG_DATA_HOME/reiwand/repo-pins.json" ] || fail "a certificate was pinned without confirmation"
failing "https://127.0.0.1:$home/r/wallpapers" --trust-certificate "$(printf '%064d' 0)"
grep -q "not the one you trusted" "$WORK/trust.out" || fail "a wrong confirmation: $(cat "$WORK/trust.out")"
ctl remote add "https://127.0.0.1:$home/r/wallpapers" --trust-certificate "$print" --wait >/dev/null
grep -q "$print" "$XDG_DATA_HOME/reiwand/repo-pins.json" || fail "the confirmed certificate was not pinned"
ctl remote refresh "${repo_id:0:8}" --wait >/dev/null
kill "$homed"; wait "$homed" 2>/dev/null || true
python3 "$tls/serve.py" "$home" "$tls" "$tls/other.pem" "$tls/other.key" >"$WORK/tls-other.log" 2>&1 & homed=$!; pids+=("$homed")
wait_for 10 serving "https://127.0.0.1:$home/r/wallpapers/entry/latest" "$tls/other.pem" || fail "the second certificate did not start"
if ctl remote refresh "${repo_id:0:8}" --wait >"$WORK/trust.out" 2>&1; then fail "a refresh went through a changed certificate"; fi
grep -q "changed" "$WORK/trust.out" || fail "a changed certificate: $(cat "$WORK/trust.out")"
[ "$(listed "[(x['warnings'], x['packages']) for x in r['repos']]")" = "[(['certificate_changed'], 4)]" ] ||
  fail "the changed certificate is not warned: $(ctl remote ls --json)"
ctl remote ls | grep -q certificate_changed || fail "remote ls does not show the changed certificate"
kill "$homed"; wait "$homed" 2>/dev/null || true
python3 "$tls/serve.py" "$home" "$tls" "$tls/home.pem" "$tls/home.key" >"$WORK/tls-home2.log" 2>&1 & pids+=($!)
wait_for 10 serving "https://127.0.0.1:$home/r/wallpapers/entry/latest" "$tls/home.pem" || fail "the first certificate did not return"
ctl remote refresh "${repo_id:0:8}" --wait >/dev/null
[ "$(listed "r['repos'][0]['warnings']")" = "[]" ] || fail "the warning stayed after the certificate returned"
ctl remote rm "${repo_id:0:8}"
ctl remote add "https://127.0.0.1:$public/r/wallpapers" --wait >/dev/null
ln -sfn "$other/repo" "$tls/r/wallpapers"
if ctl pkg install org.example.night --wait >"$WORK/trust.out" 2>&1; then fail "a package came from another signature"; fi
grep -q "signature of the repository" "$WORK/trust.out" || fail "installing from a changed repository: $(cat "$WORK/trust.out")"
if ctl remote refresh "${repo_id:0:8}" --wait >"$WORK/trust.out" 2>&1; then fail "a refresh went through another signature"; fi
grep -q "signature of the repository" "$WORK/trust.out" || fail "a changed signature: $(cat "$WORK/trust.out")"
[ "$(listed "[(x['repo_id'] == '$repo_id', x['warnings'], x['packages']) for x in r['repos']]")" = "[(True, ['signature_changed'], 4)]" ] ||
  fail "the changed signature is not warned or something was applied: $(ctl remote ls --json)"
ln -sfn "$site/repo" "$tls/r/wallpapers"
ctl remote refresh "${repo_id:0:8}" --wait >/dev/null
[ "$(listed "r['repos'][0]['warnings']")" = "[]" ] || fail "the signature warning stayed after the repository returned"
ctl remote rm "${repo_id:0:8}"
echo "repository trust checks passed"
