#!/usr/bin/env bash
set -euo pipefail
name=${1:?usage: $0 actionlint|appimage-runtime|cosign|micromamba|syft <dir>}
dir=${2:?usage: $0 actionlint|appimage-runtime|cosign|micromamba|syft <dir>}
releases=https://github.com
case $name in
  actionlint) v=1.7.12 sum=8aca8db96f1b94770f1b0d72b6dddcb1ebb8123cb3712530b08cc387b349a3d8
    url=$releases/rhysd/actionlint/releases/download/v$v/actionlint_${v}_linux_amd64.tar.gz ;;
  appimage-runtime) v=20251108 sum=2fca8b443c92510f1483a883f60061ad09b46b978b2631c807cd873a47ec260d
    url=$releases/AppImage/type2-runtime/releases/download/$v/runtime-x86_64 ;;
  cosign) v=3.1.3 sum=4629c757b7618056f8ddd7e2625ae9fdd94c0372a65049520bc7d9df9efc7f71
    url=$releases/sigstore/cosign/releases/download/v$v/cosign-linux-amd64 ;;
  micromamba) v=2.10.0-0 sum=701f39c06f8e6a670a0e186c8479b9e5c0502a2f0cda6cef65d83bdd2875e8bd
    url=$releases/mamba-org/micromamba-releases/releases/download/$v/micromamba-linux-64 ;;
  syft) v=1.54.1 sum=c069905b391cc4c20a5ba65ad5c10be2a7ba074f8ea6ad203e24d14e303dad47
    url=$releases/anchore/syft/releases/download/v$v/syft_${v}_linux_amd64.tar.gz ;;
  *) echo "unknown tool $name" >&2 && exit 2 ;;
esac
mkdir -p "$dir"
file=$dir/${url##*/}
curl -fsSL --retry 3 -o "$file" "$url"
echo "$sum  $file" | sha256sum -c - >&2
if [[ $file == *.tar.gz ]]; then tar -xzf "$file" -C "$dir" "$name"; else mv "$file" "$dir/$name"; fi
rm -f "$file"
chmod +x "$dir/$name"
echo "$dir/$name"
