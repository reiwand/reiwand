#!/usr/bin/env bash
# Adds the release's Flatpak bundles to the signed Flatpak repository published on GitHub Pages and
# writes the bundles again so that a bundle installed by hand also updates from that repository with
# `flatpak update`. Each architecture keeps only its newest commit plus a static delta from the
# previous one (a bundle is about 140 MB and Pages serves at most 1 GB); older installations pull
# the objects one by one.
#   FLATPAK_GPG_PRIVATE_KEY=<armored key> PAGES_URL=https://<owner>.github.io/<repo> \
#     scripts/flatpak-repo.sh <bundles-dir> <pages-dir> <out-dir>
# The repository lands in <pages-dir>/flatpak/repo, next to reiwand.flatpakrepo and
# io.github.reiwand.Reiwand.flatpakref; nothing else in <pages-dir> is touched.
set -euo pipefail
IN=${1:?usage: $0 <bundles-dir> <pages-dir> <out-dir>} SITE=${2:?} OUT=${3:?}
APP=io.github.reiwand.Reiwand
FPR=FFF21B32D7D030B3840D6844D723A0DA012FAD9D
KEY=$(dirname "$0")/../packaging/flatpak/repo-key.asc
URL=${PAGES_URL:?}/flatpak
REPO=$SITE/flatpak/repo
FLATHUB=https://dl.flathub.org/repo/flathub.flatpakrepo
# Pages serves at most 1 GB and git refuses files over 100 MB.
MAX_SITE_MB=${MAX_SITE_MB:-900} MAX_FILE_MB=95
BUNDLES=(x86_64:reiwand.flatpak aarch64:reiwand-aarch64.flatpak)

fail() { echo "flatpak-repo: $*" >&2; exit 1; }

GNUPGHOME=$(mktemp -d)
export GNUPGHOME
trap 'rm -rf --one-file-system "$GNUPGHOME"' EXIT
gpg --batch --quiet --import <<<"${FLATPAK_GPG_PRIVATE_KEY:?the signing key is missing}"
gpg --batch --with-colons --list-secret-keys | grep -q "^fpr:::::::::$FPR:" ||
  fail "the secret key is not the repository key $FPR"
gpg --batch --with-colons --show-keys "$KEY" | grep -q "^fpr:::::::::$FPR:" ||
  fail "$KEY is not the public key of $FPR"
gpg --batch --dearmor <"$KEY" >"$GNUPGHOME/repo-key.gpg"
SIGN=(--gpg-sign="$FPR" --gpg-homedir="$GNUPGHOME")

mkdir -p "$REPO" "$OUT"
[ -f "$REPO/config" ] || ostree init --mode=archive-z2 --repo="$REPO"
# Git keeps no empty folders, and OSTree fails without these in a repository checked out from Pages.
mkdir -p "$REPO"/{extensions,objects,state,tmp/cache,refs/heads,refs/mirrors,refs/remotes}
declare -A before
for pair in "${BUNDLES[@]}"; do
  arch=${pair%%:*}
  before[$arch]=$(ostree refs --repo="$REPO" | grep "^app/$APP/$arch/" | head -n1 || true)
  [ -z "${before[$arch]}" ] || before[$arch]=$(ostree rev-parse --repo="$REPO" "${before[$arch]}")
done
for pair in "${BUNDLES[@]}"; do
  bundle=${pair#*:}
  test -s "$IN/$bundle" || fail "$IN/$bundle is missing"
  flatpak build-import-bundle --no-update-summary "${SIGN[@]}" "$REPO" "$IN/$bundle"
done
# The bundles keep the branch the builder gave them; existing installations follow that branch.
for pair in "${BUNDLES[@]}"; do
  arch=${pair%%:*}
  mapfile -t refs < <(ostree refs --repo="$REPO" | grep "^app/$APP/$arch/")
  [ "${#refs[@]}" -eq 1 ] || fail "expected one branch of $APP for $arch, found: ${refs[*]:-none}"
  [ -z "${before[$arch]}" ] ||
    ostree static-delta generate --repo="$REPO" --from="${before[$arch]}" --to="${refs[0]}"
done
BRANCH=${refs[0]##*/}
flatpak build-update-repo "${SIGN[@]}" --title=Reiwand --default-branch="$BRANCH" \
  --prune --prune-depth=0 "$REPO"

GPGKEY=$(base64 -w0 <"$GNUPGHOME/repo-key.gpg")
cat >"$SITE/flatpak/reiwand.flatpakrepo" <<EOF
[Flatpak Repo]
Title=Reiwand
Url=$URL/repo/
Homepage=https://github.com/${GH_REPO:-reiwand/reiwand}
Comment=Animated, scene and web wallpapers for the desktop
DefaultBranch=$BRANCH
RuntimeRepo=$FLATHUB
GPGKey=$GPGKEY
EOF
cat >"$SITE/flatpak/$APP.flatpakref" <<EOF
[Flatpak Ref]
Name=$APP
Branch=$BRANCH
Title=Reiwand
Url=$URL/repo/
SuggestRemoteName=reiwand
RuntimeRepo=$FLATHUB
IsRuntime=false
GPGKey=$GPGKEY
EOF
touch "$SITE/.nojekyll"

for pair in "${BUNDLES[@]}"; do
  arch=${pair%%:*} bundle=${pair#*:}
  flatpak build-bundle --arch="$arch" --repo-url="$URL/repo/" --runtime-repo="$FLATHUB" \
    --gpg-keys="$GNUPGHOME/repo-key.gpg" "$REPO" "$OUT/$bundle" "$APP" "$BRANCH"
done

big=$(find "$SITE" -path "$SITE/.git" -prune -o -type f -size +"${MAX_FILE_MB}"M -print)
[ -z "$big" ] || fail "files over $MAX_FILE_MB MB cannot go to Pages: $big"
mb=$(du -sm --exclude=.git "$SITE" | cut -f1)
[ "$mb" -le "$MAX_SITE_MB" ] || fail "the Pages site would weigh $mb MB, over $MAX_SITE_MB MB"
echo "flatpak-repo: $BRANCH of $APP for ${#BUNDLES[@]} architectures, site $mb MB"
