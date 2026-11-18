#!/usr/bin/env bash
tls=$WORK/tls
ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
a() { x main "$@"; }
repo_url() { echo "${URL[$1]}/r/$2"; }
h() { a server "$@" --repo "$(repo_url home main)"; }
account_of() { x "$1" account ls --json | jq_ "[a['id'] for a in r['accounts'] if a['user'] == '$2' and a['server'].startswith('${URL[$3]}')][0]"; }

start_server home home "$tls/ca.pem"
start_server own own "$tls/own.pem"
for n in home own; do admin_setup "$n"; done
new_user home ana
new_user home bob
new_user home mod moderator
new_user own carl
vault_make() {
  api home "$WORK/jar-home-admin" -d '{"name": "vault", "title": "Vault"}' "${URL[home]}/api/v1/admin/repos" >/dev/null
  api home "$WORK/jar-home-admin" -X PATCH -d '{"private": true}' "${URL[home]}/api/v1/admin/repos/vault" >/dev/null
  api home "$WORK/jar-home-admin" -X PUT -d '{"may_read": true}' "${URL[home]}/api/v1/admin/repos/vault/members/$(user_id home ana)" >/dev/null
}
vault_make
for n in home own; do api "$n" "$WORK/jar-$n-admin" -X POST "${URL[$n]}/api/v1/admin/repo/publish" >/dev/null; done
export ACCT_EXTRA=SSL_CERT_FILE=$tls/ca.pem
acct_home main
acct_start main
x main provider store file >/dev/null 2>&1 || true
own_print=$(openssl x509 -in "$tls/own.pem" -noout -fingerprint -sha256 | sed 's/.*=//;s/://g' | tr 'A-F' 'a-f')

sign_in main home ana main
[ "$(a account ls --json | jq_ "[x['user'] for x in r['accounts']]")" = "['ana']" ] || fail "the account of ana: $(a account ls)"

a remote add "$(repo_url own main)" --wait >"$WORK/own-plain.out" 2>&1 && fail "a self-signed server was added without its certificate"
a remote add "$(repo_url own main)" --trust-certificate "$own_print" --wait >"$WORK/pin.out" 2>&1 || fail "the pinned server was not added: $(cat "$WORK/pin.out")"
sign_in main own carl main
[ "$(a account ls --json | jq_ "sorted(x['user'] for x in r['accounts'])")" = "['ana', 'carl']" ] || fail "two accounts: $(a account ls)"

keys=$WORK/keys
mkdir -p "$keys"
(cd "$keys" && a repo keygen first --no-password >/dev/null)
[ "$(h author-key list --json | jq_ "len(r['items'])")" = 0 ] || fail "an author key existed before registering"
fingerprint=$(h author-key add "$keys/first.pub")
[ "${#fingerprint}" = 64 ] || fail "adding an author key printed: $fingerprint"
h author-key list | grep -q "^$fingerprint " || fail "the registered key is not listed"
if h author-key add "$keys/first.key" 2>/dev/null; then fail "a secret key was sent as a public key"; fi
noon=$WORK/noon
mkdir -p "$noon"
cp "$ROOT/tests/fixtures/green-64x32.png" "$noon/image.png"
cp "$ROOT/tests/fixtures/green-64x32.png" "$noon/preview.png"
cat >"$noon/manifest.json" <<'MANIFEST'
{"schema": "reiwand.wpak/2", "id": "org.example.noon", "version": "1.0.0", "title": "Noon",
 "content": {"rating": "general"}, "photosensitive": false, "origin": {"kind": "original"},
 "license": "CC0-1.0", "kind": "image", "entry": "image.png", "previews": {"thumbnail": "preview.png"}}
MANIFEST
complete "$noon"
frost=$WORK/frost
mkdir -p "$frost"
cp "$noon"/*.png "$frost/"
sed 's/org.example.noon/org.example.frost/; s/"Noon"/"Frost"/' "$noon/manifest.json" >"$frost/manifest.json"
a pack sign --key "$keys/first.key" "$frost" >/dev/null
a pack build "$frost" -o "$WORK/frost.wpak" >/dev/null
if refused=$(h upload "$WORK/frost.wpak" --wait 2>&1); then fail "a repository that binds its authors took a key registered only in an account"; fi
grep -q "author key" <<<"$refused" || fail "the refusal does not name the signature: $refused"
h author-key remove "$fingerprint" >/dev/null
[ "$(h author-key list --json | jq_ "len(r['items'])")" = 0 ] || fail "a removed key is still listed"
a pack build "$noon" -o "$WORK/noon.wpak" >/dev/null
h upload "$WORK/noon.wpak" --unlisted --wait --json | jq_ "r['output']['state']" | grep -qx pending_review || fail "the unlisted upload did not wait for review"
case_id=$(api home "$WORK/jar-home-mod" "${URL[home]}/api/v1/mod/queue" | jq_ "r['items'][0]['id']")
api home "$WORK/jar-home-mod" -d '{"decision": "approve", "statement": "Looks fine to publish."}' "${URL[home]}/api/v1/mod/cases/$case_id/decision" >/dev/null
api home "$WORK/jar-home-admin" -X POST "${URL[home]}/api/v1/admin/repo/publish" >/dev/null
h packages | grep -q 'org.example.noon 1.0.0 published unlisted' || fail "own packages: $(h packages)"
[ "$(curl -fsS --cacert "$tls/ca.pem" "${URL[home]}/api/v1/r/main/catalog?q=noon" | jq_ "len(r['items'])")" = 0 ] || fail "the catalog lists an unlisted package"
h visibility org.example.noon public
api home "$WORK/jar-home-admin" -X POST "${URL[home]}/api/v1/admin/repo/publish" >/dev/null
a remote add "$(repo_url home main)" --wait >/dev/null || fail "the public repository was not added"
a remote refresh --wait >/dev/null
a remote vote org.example.noon up | grep -q '"up":1' || fail "the vote was not counted"
a remote vote org.example.noon down | grep -q '"down":1,.*"up":0' || fail "a changed vote was counted twice"
if a remote vote org.example.noon sideways 2>/dev/null; then fail "a vote other than up, down or none was sent"; fi

private=$(repo_url home vault)
a account unlink "$private" 2>/dev/null || true
a remote add "$private" --wait >"$WORK/vault-first.out" 2>&1 && fail "a private repository was added without an account"
a remote add "$private" --wait >"$WORK/vault-first.out" 2>&1 || true
grep -q "ana" "$WORK/vault-first.out" || fail "the needed account is not listed: $(cat "$WORK/vault-first.out")"
sign_in main home ana vault
a remote add "$private" --wait >/dev/null || fail "the private repository was not read with the right account"
ana_home=$(account_of main ana home)
vault_id=$(a remote ls --json | jq_ "[x['repo_id'] for x in r['repos'] if x['url'].rstrip('/').endswith('/r/vault')][0][:8]")
a account unlink "$private" || fail "the repository was not unlinked"
a remote refresh "$vault_id" --wait >"$WORK/unlinked.out" 2>&1 && fail "a private repository was read after its account was unlinked"
grep -q "ana" "$WORK/unlinked.out" || fail "the known accounts are not listed: $(cat "$WORK/unlinked.out")"
a account use "$private" "$ana_home" || fail "the account was not linked again"
a remote refresh "$vault_id" --wait >/dev/null || fail "the private repository was not read after linking again"

site=$WORK/mirror-site
mkdir -p "$site"
(
  cd "$site" || exit
  a repo keygen root --no-password >/dev/null
  a repo keygen online --no-password >/dev/null
  a repo init repo --name Mirror --root-key root.pub --online-key online.pub >/dev/null 2>&1
  a repo sign-root repo --key root.key >/dev/null
  a repo add repo "$WORK/noon.wpak" >/dev/null
  a repo publish repo --key online.key >/dev/null
)
mkdir -p "$site/r"
ln -s "$site/repo" "$site/r/mirror"
mirror_port=$(free_port)
front "$mirror_port" "dir:$site" "$tls/mirror.pem" "$tls/mirror.key" "$WORK/mirror-requests.log"
wait_for 20 reachable "$tls/ca.pem" "https://127.0.0.1:$mirror_port" 2>/dev/null || true
mirror=https://127.0.0.1:$mirror_port/r/mirror
a remote add "$mirror" --wait >/dev/null || fail "the mirror was not added"
if a account use "$mirror" "$ana_home" 2>"$WORK/mirror-use.out"; then fail "an account was linked to a repository of another origin"; fi
grep -q "another server" "$WORK/mirror-use.out" || fail "the refusal: $(cat "$WORK/mirror-use.out")"
a remote refresh --wait >/dev/null || fail "the mirror was not refreshed"
grep -q "^GET /r/mirror" "$WORK/mirror-requests.log" || fail "the mirror got no request"
grep -qi "^authorization:" "$WORK/mirror-requests.log" && fail "the token was sent to another origin"
grep -qi "ana" "$WORK/mirror-requests.log" && fail "the account name reached another origin"

a export "$WORK/carried.reiwand" --secrets --password-file <(printf 'pw\n') >/dev/null || fail "the export failed"
before=$(tokens home)
acct_home copy
acct_start copy
x copy provider store file >/dev/null 2>&1 || true
x copy import "$WORK/carried.reiwand" --take repositories --take secrets --password-file <(printf 'pw\n') >"$WORK/carried.out" 2>&1 ||
  fail "the import failed: $(cat "$WORK/carried.out")"
[ "$(tokens home)" -gt "$before" ] || fail "the server did not copy the session: $(cat "$WORK/carried.out")"
[ "$(x copy account ls --json | jq_ "sorted((x['user'], x['pending']) for x in r['accounts'])")" = "[('ana', False), ('carl', False)]" ] ||
  fail "the carried accounts: $(x copy account ls)"
