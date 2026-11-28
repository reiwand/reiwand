#!/usr/bin/env bash
declare -A URL CA ENVS
tls=$WORK/tls
mkdir -p "$tls"
free_port() { python3 -c "import socket; s = socket.socket(); s.bind(('127.0.0.1', 0)); print(s.getsockname()[1])"; }
openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 -nodes -keyout "$tls/ca.key" -out "$tls/ca.pem" \
  -subj /CN=test-ca -days 2 -addext basicConstraints=critical,CA:TRUE 2>/dev/null
printf 'subjectAltName=IP:127.0.0.1\nbasicConstraints=CA:FALSE\n' >"$tls/leaf.ext"
leaf() {
  openssl req -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 -nodes -keyout "$tls/$1.key" -out "$tls/$1.csr" -subj /CN=127.0.0.1 2>/dev/null
  openssl x509 -req -in "$tls/$1.csr" -CA "$tls/ca.pem" -CAkey "$tls/ca.key" -CAcreateserial -days 2 \
    -extfile "$tls/leaf.ext" -out "$tls/$1.pem" 2>/dev/null
}
leaf home
leaf mirror
openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 -nodes -keyout "$tls/own.key" -out "$tls/own.pem" \
  -subj /CN=127.0.0.1 -days 2 -addext subjectAltName=IP:127.0.0.1 2>/dev/null
front() {
  python3 "$(dirname "${BASH_SOURCE[0]}")/accounts-proxy.py" "$@" >>"$WORK/front.log" 2>&1 &
  pids+=($!)
}
reachable() { curl -fsS --cacert "$1" "$2/healthz" >/dev/null; }
srv() { local e; read -ra e <<<"${ENVS[$1]}"; shift; env "${e[@]}" "$BIN/reiwand-server" "$@"; }
start_server() {
  local name=$1 cert=$2 anchor=$3 dir=$WORK/srv-$1 back tlsp
  back=$(free_port) tlsp=$(free_port)
  mkdir -p "$dir"
  URL[$name]=https://127.0.0.1:$tlsp
  CA[$name]=$anchor
  ENVS[$name]="REIWAND_DATA=$dir REIWAND_LISTEN=127.0.0.1:$back REIWAND_PUBLIC_URL=${URL[$name]} REIWAND_RATE_LIMITS=false REIWAND_AUTO_PUBLISH=never REIWAND_PUBLISH_DELAY_SECS=1 REIWAND_LOG=warn"
  srv "$name" serve >"$WORK/server-$name.log" 2>&1 &
  pids+=($!)
  front "$tlsp" "127.0.0.1:$back" "$tls/$cert.pem" "$tls/$cert.key"
  wait_for 30 reachable "$anchor" "${URL[$name]}" || fail "the server $name did not start"
}
api() { local name=$1 jar=$2; shift 2; curl -fsS --cacert "${CA[$name]}" -b "$jar" -H 'content-type: application/json' "$@"; }
admin_setup() {
  local name=$1 jar=$WORK/jar-$1-admin
  step() { curl -fsS -o /dev/null --cacert "${CA[$name]}" -b "$jar" -c "$jar" "$@"; }
  step --data-urlencode "token=$(srv "$name" setup-token)" "${URL[$name]}/setup"
  step -d network=own "${URL[$name]}/setup/network"
  step -d handle=admin -d password=correct-horse-battery "${URL[$name]}/setup/account"
  srv "$name" repo init --name Main >/dev/null
}
new_user() {
  local name=$1 handle=$2 role=${3:-user} code
  code=$(api "$name" "$WORK/jar-$name-admin" -d "{\"role\": \"$role\"}" "${URL[$name]}/api/v1/admin/invites" | jq_ "r['code']")
  api "$name" /dev/null -d "{\"handle\": \"$handle\", \"password\": \"a long enough password\", \"invite\": \"$code\"}" \
    "${URL[$name]}/api/v1/auth/register" >/dev/null
  curl -fsS -o /dev/null --cacert "${CA[$name]}" -c "$WORK/jar-$name-$handle" -d handle="$handle" \
    -d password="a long enough password" "${URL[$name]}/login"
}
approve() {
  api "$1" "$WORK/jar-$1-$2" -d "{\"user_code\": \"$3\", \"approve\": true}" "${URL[$1]}/api/v1/auth/device/approve" >/dev/null
}
sign_in() {
  local daemon=$1 name=$2 handle=$3 repo=$4 out=$WORK/signin-$2-$3.out job code
  x "$daemon" account sign-in "${URL[$name]}/r/$repo" >"$out" 2>&1 &
  job=$!
  wait_for 30 grep -qE "[A-Z0-9]{4}-[A-Z0-9]{4}" "$out" || fail "sign-in printed no code: $(cat "$out")"
  code=$(grep -oE "[A-Z0-9]{4}-[A-Z0-9]{4}" "$out" | head -n 1)
  approve "$name" "$handle" "$code"
  wait "$job" || fail "sign-in of $handle: $(cat "$out")"
}
tokens() { python3 -c "import sqlite3, sys; print(sqlite3.connect(sys.argv[1]).execute('SELECT count(*) FROM tokens').fetchone()[0])" "$WORK/srv-$1/db/reiwand.sqlite"; }
user_id() { python3 -c "import sqlite3, sys; print(sqlite3.connect(sys.argv[1]).execute('SELECT id FROM users WHERE handle = ?', [sys.argv[2]]).fetchone()[0])" "$WORK/srv-$1/db/reiwand.sqlite" "$2"; }
