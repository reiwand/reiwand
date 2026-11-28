#!/usr/bin/env bash
set -euo pipefail
plugin_dir=$ROOT/docs/authoring/examples/provider-plugin
if ctl provider plugin install "$plugin_dir" </dev/null >/dev/null 2>"$WORK/plugin.err"; then
  fail "a provider plugin was installed without consent"
fi
grep -q "contacts: api.example.org, images.example.org" "$WORK/plugin.err" || fail "the hosts were not shown: $(cat "$WORK/plugin.err")"
[ "$(ctl provider plugin ls --json | jq_ "r['plugins']")" = "[]" ] || fail "a refused plugin was installed"
[ "$(ctl provider plugin install "$plugin_dir" --yes 2>/dev/null)" = "plugin:example-gallery" ] ||
  fail "the example provider plugin was not installed"
printf '\n' | ctl provider add gallery plugin:example-gallery query=lakes refresh=0s --secret-stdin >/dev/null
plugin_seen() { ctl provider ls --json | jq_ "[(p['kind'], p['hosts'], p['options']['per_page']) for p in r['providers'] if p['id'] == 'gallery']"; }
[ "$(plugin_seen)" = "[('plugin:example-gallery', ['api.example.org', 'images.example.org'], '30')]" ] ||
  fail "a provider of the plugin: $(plugin_seen)"
if ctl provider plugin rm example-gallery 2>"$WORK/plugin.err"; then fail "a plugin in use was removed"; fi
grep -q "is the provider of gallery" "$WORK/plugin.err" || fail "removing a plugin in use: $(cat "$WORK/plugin.err")"
ctl provider sync gallery --wait >/dev/null 2>&1 || true
plugin_error() { ctl provider ls --json | jq_ "[p['last_error'] for p in r['providers'] if p['id'] == 'gallery'][0]"; }
case "$(plugin_error)" in
  *"plugin host"*) fail "the plugin did not run in its own process: $(plugin_error)" ;;
  *"the plugin failed"* | *"refused"* | *"limit reached"*) ;;
  *) fail "a sync of the plugin did not run it: $(plugin_error)" ;;
esac
ctl provider rm gallery --purge
ctl provider plugin rm example-gallery
[ "$(ctl provider plugin ls --json | jq_ "r['plugins']")" = "[]" ] || fail "the plugin was not removed"
echo "provider plugin checks passed"
