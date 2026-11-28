#!/usr/bin/env bash
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
PREFIX=$(realpath -m "${1:?usage: $0 <prefix>}")
KDE=(
  extra-cmake-modules=f694cada1ca4cba20bbf3848b2b142ec7f6a5de5
  kirigami=8dc2b409e13c4988105fb692b68e191cb52266f7
  kcoreaddons=19b95cd96ca7d81e392661b463dbf55b3c7a9e79
  ki18n=b067e5b149e26c03af28076d01df4165366da50a
  kpipewire=f30b39c53092673d136660c8deaf24d3935723eb
)
WORK=$(mktemp -d)
trap 'rm -rf --one-file-system "$WORK"' EXIT

[ ! -e "$PREFIX" ] || { echo "$PREFIX already exists" >&2; exit 1; }
MAMBA=$("$ROOT/scripts/tools.sh" micromamba "$WORK/tools")
export MAMBA_ROOT_PREFIX=$WORK/mamba
"$MAMBA" create -y -q -p "$PREFIX" --file "$ROOT/packaging/appimage/qt.lock"
find "$MAMBA_ROOT_PREFIX/pkgs" -path '*/info/index.json' -printf '%h\n' | while read -r info; do
  name=$(python3 -c 'import json, sys; print(json.load(open(sys.argv[1]))["name"])' "$info/index.json")
  mkdir -p "$PREFIX/share/licenses/$name"
  [ ! -d "$info/licenses" ] || cp -r "$info/licenses/." "$PREFIX/share/licenses/$name/"
done

fetch() {
  git init -q "$WORK/$1"
  git -C "$WORK/$1" fetch -q --depth 1 "https://github.com/KDE/$1" "$2"
  git -C "$WORK/$1" checkout -q FETCH_HEAD
  test "$(git -C "$WORK/$1" rev-parse HEAD)" = "$2"
}
quiet() { "$@" >"$WORK/step.log" 2>&1 || { tail -n 40 "$WORK/step.log" >&2; return 1; }; }
build() {
  quiet "$PREFIX/bin/cmake" -S "$WORK/$1" -B "$WORK/$1-build" -G Ninja -DCMAKE_MAKE_PROGRAM="$PREFIX/bin/ninja" \
    -DCMAKE_BUILD_TYPE=Release -DBUILD_TESTING=OFF -DBUILD_PYTHON_BINDINGS=OFF -DBUILD_QCH=OFF \
    -DCMAKE_INSTALL_PREFIX="$PREFIX" -DCMAKE_INSTALL_LIBDIR=lib -DCMAKE_PREFIX_PATH="$PREFIX" -DKDE_INSTALL_USE_QT_SYS_PATHS=ON \
    -DQT_MAJOR_VERSION=6 -DBUILD_EXAMPLES=OFF -DCMAKE_SKIP_INSTALL_RPATH=ON
  quiet "$PREFIX/bin/cmake" --build "$WORK/$1-build"
  quiet "$PREFIX/bin/cmake" --install "$WORK/$1-build"
}
for part in "${KDE[@]}"; do
  name=${part%%=*}
  fetch "$name" "${part#*=}"
  build "$name"
  [ ! -d "$WORK/$name/LICENSES" ] || { mkdir -p "$PREFIX/share/licenses/$name" && cp -r "$WORK/$name/LICENSES/." "$PREFIX/share/licenses/$name/"; }
done
echo "$PREFIX"
