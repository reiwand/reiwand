#!/usr/bin/env bash
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
MODE=${1:-}
OUT=$(realpath -m "${2:-$ROOT/integrations}")
TMP=$(mktemp -d)
trap 'rm -rf --one-file-system "$TMP"' EXIT
case $MODE in
  update | check | build) ;;
  *) echo "usage: $0 update [language...] | check | build [output-dir]" >&2 && exit 2 ;;
esac
cd "$ROOT"
if [ "$MODE" = check ]; then "$ROOT/scripts/translations-lint.sh"; fi
shopt -s nullglob
LANGS=(po/*/)
LANGS=("${LANGS[@]#po/}")
LANGS=("${LANGS[@]%/}")
if [ "$MODE" = update ]; then LANGS+=("${@:2}"); fi

catalog() {
  local domain=$1 locale=$2 pot=$TMP/$1.pot
  sed -i -e '/^#\( \|$\)/d' -e 's/charset=[^\\]*/charset=UTF-8/' "$pot"
  for lang in "${LANGS[@]}"; do
    po=po/$lang/$domain.po
    case $MODE in
      update)
        if [ -f "$po" ]; then
          msgmerge -q -U --backup=none --add-location=file "$po" "$pot"
        else
          mkdir -p "po/$lang"
          msginit -i "$pot" -o "$po" -l "$lang" --no-translator
          sed -i 's/^"\(Last-Translator\|Language-Team\): .*/"\1: Reiwand contributors\\n"/' "$po"
        fi
        msgattrib --no-obsolete -o "$po" "$po"
        sed -i -e 's/^"\(POT-Creation-Date\|PO-Revision-Date\): \([0-9-]*\) .*/"\1: \2 00:00+0000\\n"/' \
          -e '/^"X-Generator: /d' "$po"
        ;;
      check)
        msgfmt --check -o /dev/null "$po"
        msgcmp -N "$po" "$pot"
        if { msgattrib --untranslated --no-obsolete "$po" && msgattrib --only-fuzzy --no-obsolete "$po"; } | grep '^msgid ' >/dev/null; then
          echo "$po has untranslated or fuzzy strings" >&2 && exit 1
        fi
        ;;
      build) [ -z "$locale" ] || { mkdir -p "$locale/$lang/LC_MESSAGES" && msgfmt --check -o "$locale/$lang/LC_MESSAGES/$domain.mo" "$po"; } ;;
    esac
  done
}

for pkg in integrations/*/; do
  pkg=${pkg%/}
  kind=$(sed -n 's|.*"KPackageStructure": "Plasma/\(.*\)".*|\1|p' "$pkg/metadata.json")
  [ -n "$kind" ] || continue
  domain=plasma_${kind,,}_$(sed -n 's|.*"Id": "\(.*\)".*|\1|p' "$pkg/metadata.json")
  mapfile -t sources < <(find "$pkg/contents" -name '*.qml' | LC_ALL=C sort)
  xgettext --c++ --kde --from-code=UTF-8 -ci18n -k{,x}i18n:1 -k{,x}i18nc:1c,2 -k{,x}i18np:1,2 \
    -k{,x}i18ncp:1c,2,3 -ktr:1 --add-location=file --force-po --package-name=Reiwand -o "$TMP/$domain.pot" "${sources[@]}"
  catalog "$domain" "$OUT/${pkg#integrations/}/contents/locale"
done

mapfile -t sources < <(find apps/reiwand-app/qml apps/reiwand-app/src -name '*.qml' -o -name '*.rs' | LC_ALL=C sort)
xgettext --c++ --from-code=UTF-8 -k -ktr:1 -ktrc:1c,2 --add-location=file --force-po --package-name=Reiwand \
  -o "$TMP/reiwand-app.pot" "${sources[@]}"
catalog reiwand-app ""
xgettext --c++ --from-code=UTF-8 -k -ktr:1 --add-location=file --force-po --package-name=Reiwand \
  -o "$TMP/reiwandd.pot" apps/reiwandd/src/words.rs apps/reiwandd/src/words/ask.rs
catalog reiwandd ""

mapfile -t sources < <(find crates/reiwand-scene/src crates/reiwand-format/src crates/reiwand-editor/src crates/reiwand-depth/src crates/reiwand-library/src crates/reiwand-core/src/edit -name '*.rs' | LC_ALL=C sort)
for f in "${sources[@]}"; do
  mkdir -p "$TMP/rs/$(dirname "$f")"
  sed -E "s/'([a-z_]+)([^a-z_'])/\2/g" "$f" >"$TMP/rs/$f"
done
sed -i -E 's/^(\s*)\("[a-z_.]+", .*, ("[^"]*")\),?$/\1w(\2);/' "$TMP"/rs/crates/reiwand-format/src/{check,validate,web}.rs "$TMP"/rs/crates/reiwand-format/src/check/*.rs
sed -i -E 's/^(\s*)\("[a-z_]+", ("[^"]*")\),?$/\1w(\2);/' "$TMP"/rs/crates/reiwand-format/src/check.rs "$TMP"/rs/crates/reiwand-format/src/check/*.rs
sed -i -E 's/^(\s*)\("[a-z0-9_.]+", ("[^"]*"), \[[A-Za-z, ]+\]\),?$/\1w(\2);/' "$TMP"/rs/crates/reiwand-format/src/{features,scenery}.rs
(cd "$TMP/rs" && xgettext --c++ --from-code=UTF-8 -k -ksaid:2 -kw:1 -kknown:2 -kpointed:2 -krequire:3 -kwithin:1 \
  -kUnsupported:2 -kSignature:1 --add-location=file --force-po --package-name=Reiwand -o "$TMP/reiwand-messages.pot" "${sources[@]}")
catalog reiwand-messages ""

mapfile -t sources < <(find apps/reiwand-ctl/src -name '*.rs' | LC_ALL=C sort)
helps=$(grep -hoE '(help|about|after_long_help) = ([a-z_]+::)*[A-Z][A-Z_]+' "${sources[@]}" | sed -E 's/.*[ :]([A-Z][A-Z_]+)$/\1/' | sort -u | paste -sd'|')
for f in "${sources[@]}"; do
  mkdir -p "$TMP/ctl/$(dirname "$f")"
  perl -0pe 's/\\\n\s*//g; s/(const [A-Z_]+: &str =)\s*\n\s*/$1 /g' "$f" | sed -E -e 's/^(\s*)#!?\[/\1[/' \
    -e "s/(^|[^A-Za-z])'([a-z_]+)([^a-z_'])/\1\3/g" -e 's/(help|about|long_about) = ("([^"\\]|\\.)*")/w(\2)/g' \
    -e "s/^(pub(\(crate\)|\(super\))? )?const ($helps): &str = (\"([^\"\\\\]|\\\\.)*\");/w(\4);/" >"$TMP/ctl/$f"
done
(cd "$TMP/ctl" && xgettext --c++ --from-code=UTF-8 -k -kw:1 -ktold:1 --add-location=file --force-po --package-name=Reiwand \
  -o "$TMP/reiwand-ctl.pot" "${sources[@]}")
catalog reiwand-ctl ""
