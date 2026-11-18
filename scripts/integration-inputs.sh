#!/usr/bin/env bash
set -euo pipefail
declare scene
draft=$scene/draft
looks_at() { local copy; copy=$(mktemp -d "$WORK/look.XXXXXX"); cp -r "$1/." "$copy"; complete "$copy"; shift; ctl scene preview "$copy" "$@"; }
ctl scene new "$draft" >/dev/null
ctl scene act "$draft" set '{"pointer": "/scene/canvas/background", "value": "#00ff00"}'
read -r _ _ _ s15 < <(ctl stream preview:1 160 90)
expect "list(s['outputs'])" "[]"
looks_at "$draft" -o preview:1 --time 0
shown "$s15" 0 255 0
ctl scene act "$draft" add_layer '{"kind": "solid"}' >/dev/null
ctl scene act "$draft" set '{"pointer": "/scene/layers/0/color", "value": "#ff0000"}'
looks_at "$draft" -o preview:1 --time 0
shown "$s15" 255 0 0
ctl scene act "$draft" solo '{"id": "solid"}' && ctl scene act "$draft" hide '{"hidden": true, "id": "solid"}'
looks_at "$draft" -o preview:1 --time 2
shown "$s15" 0 255 0
if ctl scene act "$draft" set '{"pointer": "/scene/canvas/size/0", "value": 0}' && looks_at "$draft" -o preview:1 2>/dev/null; then
  fail "an invalid scene was previewed"
fi
ctl scene act "$draft" undo
ctl scene act "$draft" set '{"pointer": "/scene/layers/0/blend", "value": "linear_burn"}'
looks_at "$draft" -o preview:1 --time 2 || fail "a scene with a newer blend mode was not previewed"
ctl scene act "$draft" undo
ctl scene act "$draft" set '{"pointer": "/scene/camera3d", "value": {"fov": 60, "near": 0.1, "far": 1000, "eye": [0, 0, 5], "center": [0, 0, 0], "up": [0, 1, 0], "order": "sequential", "fade": null, "paths": [{"id": "a", "ease": null, "loop": false, "keys": [{"center": [0, 0, 0], "eye": [0, 0, 5], "t": 0, "up": null, "fov": null}, {"center": [0, 0, 0], "eye": [1, 0, 5], "t": 1, "up": null, "fov": null}]}]}}'
looks_at "$draft" -o preview:1 --time 0 || fail "a scene with camera paths was not previewed"
ctl scene act "$draft" undo
ctl scene act "$draft" set '{"pointer": "/scene/layers/0", "value": { "blend_space": "canvas", "id": "title", "text": {"media": "title"}, "type": "text"}}'
looks_at "$draft" -o preview:1 --time 0 || fail "a scene showing media titles was not previewed"
ctl scene act "$draft" undo
ctl scene act "$draft" set '{"pointer": "/scene/layers/0/transform/tilt", "value": [10, 0]}'
ctl scene act "$draft" set '{"pointer": "/scene/canvas/camera", "value": {"rotation": 10, "zoom": 1.5}}'
ctl scene act "$draft" set '{"pointer": "/scene/canvas/bloom", "value": {"tint": "#ffd0a0"}}'
looks_at "$draft" -o preview:1 --time 2 || fail "a scene with a camera, bloom and tilted layers was not previewed"
ctl scene act "$draft" undo && ctl scene act "$draft" undo && ctl scene act "$draft" undo
ctl scene act "$draft" set '{"pointer": "/scene/layers/0/solid", "value": {"alpha": 0.5}}'
looks_at "$draft" -o preview:1 --time 2 || fail "a scene with solid layers was not previewed"
ctl scene act "$draft" undo
ctl scene act "$draft" set '{"pointer": "/scene/layers/0/effects", "value": [{"effect": "glitter", "params": {"speed": 0}}, {"effect": "fluid", "params": {"emitters": [{"from": [0.3, 0.9], "strength": 8}, {"from": [0.2, 1], "strength": 4, "to": [0.8, 1]}]}}, {"effect": "god_rays", "params": {"mode": 2}}]}'
looks_at "$draft" -o preview:1 --time 2 || fail "a scene with glitter, fluid and a star of rays was not previewed"
ctl scene act "$draft" keyframe '{"pointer": "/scene/layers/0/effects/1/params/emitters", "time": 0}' && ctl scene act "$draft" keyframe '{"pointer": "/scene/layers/0/effects/1/params/emitters", "time": 3, "value": [{"angle": 0, "from": [0.7, 0.5], "strength": 2}, {"from": [0.2, 0.9], "strength": 6, "to": [0.8, 0.9]}]}'
grep -q '"keys"' "$draft/scene.json" || fail "fluid emitters were not keyframed"
looks_at "$draft" -o preview:1 --time 1.5 || fail "a scene with keyframed fluid emitters was not previewed"
ctl scene act "$draft" undo && ctl scene act "$draft" undo && ctl scene act "$draft" undo
ctl scene act "$draft" set '{"pointer": "/scene/layers/0/effects", "value": [{"effect": "blend_gradient"}]}'
ctl scene act "$draft" paint_mask '{"channel": 1, "pointer": "/scene/layers/0/effects/0/params/map", "points": [[0.4, 0.5], [0.6, 0.5]], "radius": 0.05}' || fail "a mask was not painted"
looks_at "$draft" -o preview:1 --time 2 || fail "a scene with a painted mask was not previewed"
ctl scene act "$draft" undo
ctl scene act "$draft" undo
ctl scene act "$draft" set '{"pointer": "/scene/layers/0/effects", "value": [{"effect": "light_shafts", "params": {"speed": 0}}, {"effect": "tint", "mask": {"shape": {"ellipse": {"size": [40, 40]}}}}]}'
looks_at "$draft" -o preview:1 --time 2 || fail "a scene with the newer built-in effects was not previewed"
ctl scene act "$draft" undo
ctl scene act "$draft" set '{"pointer": "/scene/clock", "value": {"seed": null}}'
looks_at "$draft" -o preview:1 --time 2 || fail "a scene with a fresh seed was not previewed"
ctl scene act "$draft" undo
steps_done() {
  ctl scene ask "$draft" history | python3 -c 'import json, sys; print(len(json.load(sys.stdin)["done"]))'
}
steps=$(steps_done)
ctl scene import "$draft" "$ROOT/tests/fixtures/green-64x32.png" --id leaf >/dev/null
ctl scene act "$draft" mesh_from_picture '{"cells": 8, "id": "puppet", "image": "leaf"}' >/dev/null || fail "no mesh was cut from a picture"
ctl scene act "$draft" bone '{"at": [0.2, 0.5], "layer": "puppet", "name": "root"}' || fail "the mesh took no root bone"
ctl scene act "$draft" bone '{"at": [0.6, 0.5], "layer": "puppet", "name": "arm", "parent": "root"}' || fail "the mesh took no arm"
ctl scene ask "$draft" rig --layer puppet | grep '"arm"' >/dev/null || fail "the bones of the mesh were not listed"
ctl scene ask "$draft" outline --layer puppet | grep '\[' >/dev/null || fail "the outlines of the parts were not listed"
ctl scene act "$draft" set '{"pointer": "/scene/layers/1/clips", "value": [{"bones": {"arm": {"rotation": [[0, 0], [1, 40]]}}, "id": "wave"}]}'
ctl scene act "$draft" set '{"pointer": "/scene/layers/1/animations", "value": [{"clip": "wave"}]}'
looks_at "$draft" -o preview:1 --time 0.5 || fail "a scene with a bent mesh was not previewed"
ctl scene act "$draft" set '{"pointer": "/scene/layers/1/ik", "value": [{"chain": ["root", "arm"], "limits": {"arm": [-60, 60]}, "target": "cursor"}]}'
ctl scene act "$draft" set '{"pointer": "/scene/layers/1/physics", "value": [{"bone": "arm", "gravity": 1, "mode": "rope", "wind": 0.5}]}'
ctl scene act "$draft" set '{"pointer": "/scene/layers/1/depth", "value": {"cull": true, "scale": 1}}'
ctl scene act "$draft" mesh_depth '{"by": 30, "layer": "puppet", "points": [[0.4, 0.5], [0.6, 0.5]], "radius": 0.2}' || fail "the depth of the mesh was not painted"
ctl scene act "$draft" set '{"pointer": "/scene/layers/1/hit", "value": ["arm"]}'
ctl scene act "$draft" set '{"pointer": "/scene/canvas/wind", "value": [40, 0]}'
looks_at "$draft" -o preview:1 --time 0.5 || fail "a scene with reaching and swinging bones was not previewed"
ctl scene act "$draft" bone '{"at": [0.9, 0.5], "layer": "puppet", "name": "hand", "parent": "arm"}' || fail "the mesh took no hand"
ctl scene act "$draft" weights '{"bone": "arm", "by": 0.5, "layer": "puppet", "lock": ["root"], "mode": "add", "points": [[0.5, 0.5], [0.6, 0.5]], "radius": 0.2}' || fail "no weights were painted"
ctl scene ask "$draft" weights '"arm"' --layer puppet | grep '"arm"' >/dev/null || fail "the weights of the vertices were not listed"
ctl scene act "$draft" bone '{"layer": "puppet", "name": "hand", "to": "paw"}' || fail "the hand was not renamed"
ctl scene act "$draft" bone '{"layer": "puppet", "name": "paw", "parent": ""}' || fail "the paw was not rehung"
ctl scene act "$draft" auto_weights '{"layer": "puppet"}' || fail "the weights were not bound to the bones again"
ctl scene act "$draft" clip '{"add": true, "id": "swing", "layer": "puppet", "length": 2, "loop": "pingpong"}' || fail "no clip was added"
ctl scene act "$draft" key '{"bone": "arm", "clip": "swing", "ease": "in_out_cubic", "layer": "puppet", "position": [4, 0], "rotation": 30, "scale": [1.2, 1.2], "time": 1}' || fail "the arm was not keyed"
ctl scene act "$draft" key '{"bone": "arm", "clip": "swing", "copy": true, "from": 1, "layer": "puppet", "time": 1.5}' || fail "the key was not copied"
ctl scene act "$draft" key '{"clip": "swing", "layer": "puppet", "order": "body", "time": 0, "value": 3}' || fail "the order of a part was not keyed"
ctl scene ask "$draft" keys '"swing"' --layer puppet | grep '"in_out_cubic"' >/dev/null || fail "the keys of the clip were not listed"
ctl scene act "$draft" key '{"bone": "arm", "clip": "swing", "layer": "puppet", "out_handle": [0.5, 10], "rotation": 0, "time": 0}' || fail "a key of a clip took no Bézier handle"
ctl scene ask "$draft" keys '"swing"' --layer puppet | grep '"out"' >/dev/null || fail "the handle of the key was not listed"
ctl scene act "$draft" key '{"bone": "arm", "clip": "swing", "ease": [0.25, 0.1, 0.25, 1], "layer": "puppet", "rotation": 0, "time": 0}' || fail "a key of a clip took no cubic Bézier easing"
if ctl scene act "$draft" key '{"bone": "arm", "clip": "swing", "ease": "linear", "in_handle": [0.1, 1], "layer": "puppet", "rotation": 0, "time": 0}' 2>/dev/null; then fail "a key took handles and an easing together"; fi
ctl scene act "$draft" clip '{"id": "swing", "layer": "puppet", "to": "sway"}' || fail "the clip was not renamed"
ctl scene act "$draft" set '{"pointer": "/scene/layers/1/animations", "value": [{"clip": "sway"}]}'
looks_at "$draft" -o preview:1 --time 0.5 || fail "a scene with a keyed and painted mesh was not previewed"
ctl scene act "$draft" ik '{"bend": "counterclockwise", "layer": "puppet", "limits": {"arm": [-45, 45]}, "root": "root", "tip": "arm"}' || fail "no chain was added"
ctl scene act "$draft" ik '{"index": 1, "layer": "puppet", "target": [30, 20], "weight": 0.8}' || fail "the chain was not aimed"
ctl scene act "$draft" ik '{"index": 0, "layer": "puppet", "remove": true}' || fail "the chain was not removed"
ctl scene act "$draft" physics '{"bones": ["arm"], "gravity": -2, "layer": "puppet", "mode": "spring", "stiffness": 0.3}' || fail "the arm does not swing"
ctl scene act "$draft" physics '{"bones": ["tail"], "layer": "puppet", "parent": "arm", "rope": [[0.6, 0.6], [0.7, 0.7], [0.8, 0.8]], "wind": 0.5}' || fail "no rope was drawn"
ctl scene act "$draft" physics '{"bones": ["arm"], "layer": "puppet", "rotation": [-20, 20], "translation": [-4, 4]}' || fail "the swing of the arm was not bounded"
ctl scene act "$draft" physics '{"bones": ["arm"], "free": ["rotation"], "layer": "puppet"}' || fail "the bound of the arm was not lifted"
ctl scene act "$draft" physics '{"bones": ["arm"], "layer": "puppet", "remove": true}' || fail "the arm still swings"
ctl scene act "$draft" wind '{"angle": 30, "gusts": 0.5, "strength": 60}' || fail "the wind was not set"
ctl scene act "$draft" add_layer '{"id": "badge", "kind": "solid"}' >/dev/null
ctl scene act "$draft" attach '{"bone": "arm", "inherit": ["position", "rotation"], "layer": "badge", "to": "puppet"}' || fail "the layer was not attached"
ctl scene act "$draft" attach '{"layer": "badge", "point": [0.5, 0.4]}' || fail "the layer was not pinned to a new point"
ctl scene act "$draft" attach '{"layer": "badge", "nudge": [4, -2]}' || fail "the layer was not moved along its point"
ctl scene act "$draft" ik '{"layer": "puppet", "root": "root", "tip": "arm"}' || fail "no chain without limits was added"
ctl scene act "$draft" point '{"at": [0.5, 0.3], "layer": "puppet", "name": "pin"}' || fail "no point was placed"
ctl scene act "$draft" point '{"layer": "puppet", "name": "pin", "parent": "root", "to": "peg"}' || fail "the point was not renamed and rehung"
ctl scene ask "$draft" rig --layer puppet | grep '"peg"' >/dev/null || fail "the point was not listed"
ctl scene act "$draft" point '{"layer": "puppet", "name": "peg", "remove": true}' || fail "the point was not removed"
ctl pack check "$draft" >/dev/null || fail "removing a point left the scene invalid"
ctl scene act "$draft" parts '{"hide": ["body"], "layer": "puppet", "order": {"body": 2}}' || fail "the part was not ordered"
ctl scene act "$draft" parts '{"layer": "puppet", "show": ["body"]}' || fail "the part was not shown"
ctl scene act "$draft" parts '{"depth": {"cull": true, "scale": 2, "sort": false}, "layer": "puppet"}' || fail "the depth of the mesh was not set"
if ctl scene act "$draft" parts '{"depth": {"scale": 20}, "layer": "puppet"}' 2>/dev/null; then fail "a depth scale of 20 was kept"; fi
ctl scene act "$draft" parts '{"depth": {"off": true}, "layer": "puppet"}' || fail "the depth of the mesh was not turned off"
looks_at "$draft" -o preview:1 --time 0.5 || fail "a scene with chains, a rope, wind and attachments was not previewed"
ctl scene act "$draft" attach '{"detach": true, "layer": "badge"}' || fail "the layer was not let go"
source "$(dirname "$0")"/integration-particles.sh
source "$(dirname "$0")"/integration-lights.sh
source "$(dirname "$0")"/integration-models.sh
source "$(dirname "$0")"/integration-shared.sh
source "$(dirname "$0")"/integration-authoring.sh
source "$(dirname "$0")"/integration-seeding.sh
source "$(dirname "$0")"/integration-timing.sh
ctl scene act "$draft" bone '{"at": [0.9, 0.5], "layer": "puppet", "name": "tip", "parent": "arm"}' || fail "the mesh took no tip"
ctl scene act "$draft" ik '{"layer": "puppet", "limits": {"tip": [-30, 30]}, "root": "arm", "tip": "tip"}' || fail "no chain to the tip was added"
ctl scene act "$draft" physics '{"bones": ["tip"], "layer": "puppet"}' || fail "the tip does not swing"
ctl scene act "$draft" key '{"bone": "tip", "clip": "sway", "layer": "puppet", "rotation": 10, "time": 0.5}' || fail "the tip was not keyed"
ctl scene act "$draft" bone '{"layer": "puppet", "name": "tip", "remove": true}' || fail "the tip was not removed"
ctl pack check "$draft" >/dev/null || fail "removing a bone left the scene invalid"
ctl scene act "$draft" undo
ctl scene act "$draft" bone '{"layer": "puppet", "name": "arm", "parent": ""}' || fail "the arm was not rehung"
ctl pack check "$draft" >/dev/null || fail "rehanging a bone left the scene invalid"
ctl scene act "$draft" undo
ctl scene act "$draft" bone '{"layer": "puppet", "name": "arm", "parent": "paw", "to": "limb"}' || fail "the arm was not renamed and rehung"
ctl pack check "$draft" >/dev/null || fail "renaming and rehanging a bone left the scene invalid"
ctl scene import "$draft" "$ROOT/tests/fixtures/parts.glb" --id body >/dev/null
ctl scene act "$draft" add '{"pointer": "/scene/layers/-", "value": { "blend_space": "canvas", "id": "body", "mesh": "body", "size": [100, 100], "type": "mesh"}}'
ctl scene act "$draft" bone '{"at": [0.8, 0.8], "layer": "body", "name": "tail", "parent": "hip"}' || fail "a mesh of several parts took no bone"
ctl scene act "$draft" expression '{"layer": "body", "name": "happy", "shapes": {"smile": 0.8}}' || fail "no expression was made"
ctl scene act "$draft" expression '{"layer": "body", "name": "happy", "to": "glad"}' || fail "the expression was not renamed"
ctl scene act "$draft" clip '{"add": true, "id": "grin", "layer": "body"}' || fail "no clip was added to the mesh of several parts"
ctl scene act "$draft" key '{"clip": "grin", "layer": "body", "shape": "smile", "time": 0.5, "weight": 0.5}' || fail "the blend shape was not keyed"
ctl scene act "$draft" parts '{"clip": {"chest": "legs"}, "layer": "body", "order": {"legs": -1}}' || fail "the part was not cut to another"
looks_at "$draft" -o preview:1 --time 0.5 || fail "a scene with a mesh of several parts was not previewed"
ctl pack build "$draft" -o "$WORK/meshes.wpak" >/dev/null || fail "the scene with meshes was not built"
packed=$(python3 -c "import sys, zipfile; print(sum(n.startswith('meshes/') for n in zipfile.ZipFile(sys.argv[1]).namelist()))" "$WORK/meshes.wpak")
[ "$packed" = 2 ] || fail "the package carries $packed meshes instead of the two in use"
while [ "$(steps_done)" -gt "$steps" ]; do ctl scene act "$draft" undo; done
"$BIN/probe" "$s15" 4 --hold >"$WORK/edit-frames.out" 2>&1 & counter=$!; pids+=("$counter")
sleep 1
ctl scene act "$draft" set '{"pointer": "/scene/canvas/background", "value": "#0000ff"}'
looks_at "$draft" -o preview:1 --time 2
wait "$counter" || fail "no frames while editing: $(cat "$WORK/edit-frames.out")"
frames=$(sed -n 's/^frames //p' "$WORK/edit-frames.out")
[ "$frames" -lt 15 ] || fail "one edit of a held preview rendered $frames frames"
ctl scene act "$draft" solo && ctl scene act "$draft" hide '{"hidden": false, "id": "solid"}'
ctl scene act "$draft" keyframe '{"pointer": "/scene/layers/0/color", "time": 0, "value": "#ff0000"}'
ctl scene act "$draft" keyframe '{"pointer": "/scene/layers/0/color", "time": 16, "value": "#0000ff"}'
"$BIN/probe" "$s15" 10 --hold >"$WORK/play-frames.out" 2>&1 & player=$!; pids+=("$player")
looks_at "$draft" -o preview:1
sleep 6
ctl scene act "$draft" set '{"pointer": "/scene/canvas/background", "value": "#00ff00"}'
looks_at "$draft" -o preview:1
read -r b g r _ < <("$BIN/probe" "$s15" 3 | sed 's/.*center bgra \[\(.*\)\]/\1/; s/,/ /g')
[ "$b" -gt 85 ] || fail "an edit restarted the playing preview: bgra $b $g $r"
wait "$player" || fail "the playing preview stopped: $(cat "$WORK/play-frames.out")"
ctl scene act "$draft" translate '{"language": "es", "pointer": "/manifest/title", "text": "Borrador"}'
ctl scene act "$draft" set '{"pointer": "/manifest/content", "value": {"descriptors": ["gore"], "rating": "mature"}}'
looks_at "$draft" -o preview:1 --time 0
shown "$s15" 255 0 0
ctl scene act "$draft" remove '{"pointer": "/manifest/content"}' && ctl scene act "$draft" remove '{"pointer": "/manifest/origin"}'
ctl scene act "$draft" hide '{"hidden": true, "id": "solid"}'
looks_at "$draft" -o preview:1 --time 0 || fail "incomplete package details stopped the preview"
shown "$s15" 0 255 0
ctl scene act "$draft" hide '{"hidden": false, "id": "solid"}'
[ "$(ctl scene act "$draft" add_property '{"id": "look", "kind": "combo"}')" = look ] || fail "add-property printed no id"
ctl scene act "$draft" set '{"pointer": "/manifest/properties/look/options", "value": [{"label": "Red", "value": "red"}, {"label": "Blue", "value": "blue"}]}'
ctl scene act "$draft" set '{"pointer": "/manifest/properties/look/default", "value": "blue"}'
ctl scene act "$draft" set '{"pointer": "/scene/layers/0/color", "value": {"map": {"blue": "#0000ff", "red": "#ff0000"}, "prop": "look"}}'
ctl scene act "$draft" add_property '{"id": "flash", "kind": "button"}' >/dev/null
if ctl scene act "$draft" condition '{"id": "flash", "text": "look > 2"}' 2>/dev/null; then fail "a condition that compares a text with a number was kept"; fi
ctl scene act "$draft" condition '{"enabled": true, "id": "flash", "text": "look == '\''red'\''"}'
ctl scene act "$draft" rename_property '{"id": "look", "to": "tone"}'
looks_at "$draft" -o preview:1 --time 0
shown "$s15" 0 0 255
ctl scene act "$draft" set '{"pointer": "/manifest/properties/tone/default", "value": "red"}'
looks_at "$draft" -o preview:1 --time 0
shown "$s15" 255 0 0
scene_doc "$draft" /manifest/requires/features | grep properties2 >/dev/null || fail "a button left properties2 out of the features"
if ctl scene act "$draft" remove_property '{"id": "tone"}' 2>/dev/null; then fail "a property that a binding follows was removed"; fi
ctl scene act "$draft" remove_property '{"id": "flash"}'
if scene_doc "$draft" /manifest/properties | grep flash >/dev/null; then fail "remove-property kept the button"; fi
looks_at "$draft" -o preview:1 --stop
ctl release preview:1 && expect "list(s['outputs'])" "[]"
echo "scene editor preview checks passed"
source "$(dirname "$0")"/integration-grants.sh
