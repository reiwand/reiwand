#!/bin/sh
set -eu
root=$(cd "$(dirname "$0")/../.." && pwd)
out=$root/target/flatpak-src
rm -rf "$out" "$out.list" "$out.tar"
mkdir -p "$out"
git -c safe.directory="$root" -C "$root" ls-files -z > "$out.list"
tar -C "$root" --null --no-recursion --ignore-failed-read -T "$out.list" -cf "$out.tar"
tar -C "$out" -xf "$out.tar"
rm "$out.list" "$out.tar"
if [ -n "${REIWAND_VERSION:-}" ]; then
  printf '%s\n' "$REIWAND_VERSION" > "$out/VERSION"
  sed -i "s/\"Version\": \"[^\"]*\"/\"Version\": \"${REIWAND_VERSION%%+*}\"/" "$out"/integrations/*/metadata.json
fi
version=${REIWAND_VERSION:-$(sed -n 's/^version = "\(.*\)"$/\1/p' "$root/Cargo.toml" | head -n 1)}
"$root/packaging/flatpak/releases.sh" "$out/packaging/flatpak/io.github.reiwand.Reiwand.metainfo.xml" "$version" "$root"
"$root/packaging/flatpak/cargo-sources.py"
echo "$out"
