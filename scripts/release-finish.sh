#!/usr/bin/env bash
# Finishes the draft that scripts/release.sh made: adds the Flatpak bundles to it when they were
# built, signs SHA256SUMS once (cosign and the release key) and publishes the version. Updaters only
# see published versions, so none ever finds checksums without their signature or the other way round.
#   VERSION=<semver> scripts/release-finish.sh [<bundles-dir>]
set -euo pipefail
IN=${1:-}
TAG=v${VERSION:?usage: VERSION=<semver> $0 [<bundles-dir>]}
GH_REPO=${GH_REPO:?}
OUT=$(mktemp -d)
trap 'rm -rf --one-file-system "$OUT"' EXIT
gh release view "$TAG" >/dev/null || { echo "$TAG was not made, so there is nothing to publish"; exit 0; }
mkdir "$OUT/files"
gh release download "$TAG" --pattern SHA256SUMS --dir "$OUT/files"
upload=()
if [ -n "$IN" ]; then
  bundles=(reiwand.flatpak reiwand-aarch64.flatpak)
  for bundle in "${bundles[@]}"; do
    test -s "$IN/$bundle"
    cp "$IN/$bundle" "$OUT/files/$bundle"
  done
  sed -i '/[ *]reiwand\(-aarch64\)\?\.flatpak$/d' "$OUT/files/SHA256SUMS"
  (cd "$OUT/files" && sha256sum -- "${bundles[@]}" >> SHA256SUMS && sha256sum -c --ignore-missing SHA256SUMS)
  upload+=("$OUT/files/SHA256SUMS" "${bundles[@]/#/$OUT/files/}")
else
  echo "No Flatpak bundles: $TAG is published without them"
fi
# What gets signed must be what the release holds: every file of the draft is listed in SHA256SUMS
# with the SHA-256 that GitHub computed on upload (bundles about to be added are checked above).
own=$(gh api "repos/$GH_REPO/releases" --paginate --jq ".[] | select(.tag_name == \"$TAG\") | .assets[] |
  select(.name | test(\"^SHA256SUMS\") | not) | \"\(.digest | ltrimstr(\"sha256:\"))  \(.name)\"")
while read -r sum name; do
  [ -n "$name" ] || continue
  if [ -n "$IN" ] && [[ " ${bundles[*]} " == *" $name "* ]]; then continue; fi
  grep -qxF "$sum  $name" "$OUT/files/SHA256SUMS" || { echo "$name of $TAG does not match SHA256SUMS: nothing signed"; exit 1; }
done <<<"$own"
listed=$(awk '{print $2}' "$OUT/files/SHA256SUMS" | sed 's/^\*//' | sort)
held=$( { awk '{print $2}' <<<"$own"; [ -z "$IN" ] || printf '%s\n' "${bundles[@]}"; } | sort -u)
[ "$listed" = "$held" ] || { echo "SHA256SUMS of $TAG lists other files than the release holds: nothing signed"; exit 1; }
if [ "${SIGN:-}" = true ]; then
  "$("$(dirname "$0")"/tools.sh cosign "$OUT/tools")" sign-blob --yes --bundle "$OUT/files/SHA256SUMS.sigstore.json" "$OUT/files/SHA256SUMS"
  "$(dirname "$0")"/sign-sums.sh "$OUT/files/SHA256SUMS"
  upload+=("$OUT/files/SHA256SUMS.sigstore.json" "$OUT/files/SHA256SUMS.asc")
fi
[ "${#upload[@]}" -eq 0 ] || gh release upload "$TAG" "${upload[@]}" --clobber
gh release edit "$TAG" --draft=false --latest
