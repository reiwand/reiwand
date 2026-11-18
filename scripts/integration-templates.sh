#!/usr/bin/env bash
set -euo pipefail
declare scene
made=$scene/authoring
ctl scene new "$made" >/dev/null
ctl scene act "$made" add_layer '{"id": "fill", "kind": "solid"}' >/dev/null
ctl scene act "$made" duplicate_layer '{"id": "fill", "template": true}' || fail "no template was made"
ctl scene act "$made" duplicate_layer '{"id": "fill", "template": true}' || fail "no second template was made"
grep -q '"fill_template_2"' "$made/scene.json" || fail "the second template did not get a free name"
ctl scene import "$made" "$ROOT/tests/fixtures/magenta-64x32.webm" --id clip >/dev/null || fail "the video was not imported"
ctl scene act "$made" set '{"pointer": "/scene/layers/0", "value": { "blend_space": "canvas", "id": "film", "playback": {"own_clock": true, "rate": 2, "start": 0.5}, "type": "video", "video": "clip"}}'
ctl scene act "$made" add_layer '{"id": "flakes", "kind": "particles"}' >/dev/null
printf '%s' '{"id": "throb", "name": {"en": "Pulse"}, "cost": {"taps": 1}, "passes": [{"entry": "fs_throb"}],
  "params": [{"id": "gain", "type": "float", "default": 1, "min": 0, "max": 4}, {"id": "speed", "type": "float", "default": 0}]}' >"$WORK/throb.json"
"$BIN/reiwand-upgrade" effects "$WORK/throb.json" >/dev/null
printf '%s' '@fragment fn fs_throb(in: VertexOut) -> @location(0) vec4<f32> { return tap(in.uv) * param(0u).x; }' >"$WORK/throb.wgsl"
ctl scene act "$made" import_effect '{"descriptor": "'"$WORK"'/throb.json"}' >/dev/null || fail "the package effect was not imported"
ctl scene act "$made" effect_rule '{"effect": "throb", "param": "gain", "uses": "time", "waits_for": "speed"}' || fail "the wake rule was not changed"
ctl scene act "$made" effect_rule '{"effect": "throb", "param": "gain", "uses": "", "waits_for": ""}' || fail "the wake rule was not cleared"
ctl scene --inside fill_template act "$made" group '{"id": "box", "ids": ["fill"]}' || fail "the template layer was not grouped"
ctl scene --inside fill_template act "$made" add_layer '{"id": "dot", "into": "box", "kind": "solid"}' >/dev/null || fail "no layer was added inside the template"
scene_doc "$made" /scene/templates/fill_template/layers/1/id | grep '"dot"' >/dev/null ||
  fail "the layer did not land in the template"
ctl scene open "$made" --json --inside fill_template | grep -F '"pointer":"/scene/templates/fill_template/layers/1"' >/dev/null ||
  fail "the outline of the template did not list its layers"
ctl scene act "$made" add_property '{"id": "amount", "kind": "slider"}' >/dev/null
ctl scene act "$made" add_layer '{"id": "other", "kind": "solid"}' >/dev/null
other=$(ctl scene open "$made" --json | grep -o '"id":"other"[^}]*"pointer":"[^"]*"' | grep -o '/scene/layers/[0-9]*' | head -1)
ctl scene act "$made" set '{"pointer": "'"$other"'/opacity", "value": {"prop": "amount"}}'
ctl scene --inside fill_template act "$made" rename_property '{"id": "amount", "to": "level"}' || fail "the property was not renamed from inside a template"
scene_doc "$made" "$other/opacity" | grep '"level"' >/dev/null || fail "a binding of the scene kept the old property id"
ctl pack check "$made" >/dev/null || fail "a rename inside a template broke the scene"
