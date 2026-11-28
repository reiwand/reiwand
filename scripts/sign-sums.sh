#!/usr/bin/env bash
# Writes <SHA256SUMS>.asc, a detached OpenPGP signature made with the release key (the one that also
# signs the Flatpak repository), which `reiwand --update` checks before installing a new AppImage.
#   FLATPAK_GPG_PRIVATE_KEY=<armored key> scripts/sign-sums.sh <SHA256SUMS>
set -euo pipefail
SUMS=${1:?usage: $0 <SHA256SUMS>}
FPR=FFF21B32D7D030B3840D6844D723A0DA012FAD9D
GNUPGHOME=$(mktemp -d)
export GNUPGHOME
trap 'rm -rf --one-file-system "$GNUPGHOME"' EXIT
gpg --batch --quiet --import <<<"${FLATPAK_GPG_PRIVATE_KEY:?the release key is missing}"
gpg --batch --with-colons --list-secret-keys | grep -q "^fpr:::::::::$FPR:" || {
  echo "sign-sums: the secret key is not the release key $FPR" >&2
  exit 1
}
gpg --batch --yes --armor --local-user "$FPR" --detach-sign --output "$SUMS.asc" "$SUMS"
gpg --batch --verify "$SUMS.asc" "$SUMS" 2>/dev/null
