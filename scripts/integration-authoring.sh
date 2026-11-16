#!/usr/bin/env bash
set -euo pipefail
declare scene ROOT
craft=$scene/craft
ctl scene new "$craft" --id org.example.craft >/dev/null
ctl scene act "$craft" add_layer '{"id": "card", "kind": "solid"}' >/dev/null
ctl scene act "$craft" add_layer '{"id": "hole", "kind": "solid"}' >/dev/null
ctl scene act "$craft" effects '{"add": {"effect": "vignette", "layer": "card", "name": "edge", "params": {"intensity": 0.6}}}' || fail "no effect started the stack"
ctl scene act "$craft" effects '{"add": {"at": 0, "effect": "film_grain", "layer": "card"}}' || fail "no second effect was added"
ctl scene act "$craft" effects '{"set": {"enabled": true, "index": 1, "layer": "card", "params": {"radius": 1.5}}}' || fail "the effect was not set"
ctl scene act "$craft" effects '{"mask": {"channel": "luminance", "from": "hole", "index": 1, "invert": true, "layer": "card"}}' || fail "the effect was not masked"
ctl scene act "$craft" effects '{"move": {"index": 1, "layer": "card", "to": 0}}' || fail "the effect was not moved"
ctl scene act "$craft" effects '{"remove": {"index": 1, "layer": "card"}}' || fail "the effect was not removed"
ctl scene act "$craft" add '{"pointer": "/scene/layers/1/effects/-", "value": {"effect": "vignette"}}' || fail "the list of effects was not created"
ctl scene preview "$craft" -o preview:1 --time 0.5 || fail "a scene with masked effects was not previewed"
ctl scene act "$craft" add_layer '{"id": "dust", "kind": "particles"}' >/dev/null
ctl scene act "$craft" particles '{"create": {"on": "static", "parent": "dust", "system": "sparks"}}' || fail "no child system was created"
ctl scene act "$craft" particles '{"settings": {"capacity": 300, "prewarm": 1, "rate": 30, "seed": 4, "space": "world", "system": "dust"}}' || fail "the system was not set"
ctl scene act "$craft" particles '{"start": {"ranges": {"color": ["#ffffff", "#88ccff"], "life": [1, 3, null], "size": [2, 5, null], "velocity": [[-10, 0], [10, -40]]}, "system": "dust"}}' ||
  fail "the starting ranges were not set"
ctl scene act "$craft" particles '{"start": {"clear": ["velocity"], "system": "dust"}}' || fail "the velocity range was not cleared"
ctl scene act "$craft" particles '{"force": {"force": {"gravity": [0, 40]}, "system": "dust"}}' || fail "no gravity was added"
ctl scene act "$craft" particles '{"force": {"force": {"drag": 0.2}, "index": 0, "system": "dust"}}' || fail "the force was not replaced"
ctl scene act "$craft" particles '{"force": {"force": {"oscillate": {"freq": [1, 2], "scale": [0.5, 1], "what": "size"}}, "system": "dust"}}' || fail "no swing was added"
ctl scene act "$craft" particles '{"force": {"force": {"attractor": {"falloff": "linear", "strength": 30, "target": [40, 30]}}, "system": "dust"}}' || fail "no attractor was added"
ctl scene act "$craft" particles '{"force": {"force": {"turbulence": {"scale": 0.01, "strength": 10}}, "system": "dust"}}' || fail "no turbulence was added"
ctl scene act "$craft" particles '{"force": {"index": 1, "remove": true, "system": "dust"}}' || fail "the force was not removed"
ctl scene act "$craft" particles '{"force": {"force": {"turbulence": {"scale": 0.01, "strength": -20}}, "system": "dust"}}' || fail "a turbulence that pulls back was refused"
words=$(ctl scene act "$craft" particles '{"settings": {"prewarm": -5, "system": "dust"}}' 2>&1) && fail "a negative warm-up was kept"
case $words in *unexpected*) fail "a negative warm-up was not checked by the editor: $words" ;; esac
ctl scene act "$craft" particles '{"child": {"child": {"on": "death", "probability": 0.5, "system": "sparks"}, "system": "dust"}}' || fail "no child was added"
ctl scene act "$craft" particles '{"child": {"child": {"max": 3, "on": "spawn", "system": "sparks"}, "index": 1, "system": "dust"}}' || fail "the child was not replaced"
ctl scene act "$craft" particles '{"child": {"index": 0, "remove": true, "system": "dust"}}' || fail "the child was not removed"
ctl scene act "$craft" particles '{"over": {"size": [[0, 1], [0.5, 2], [1, 0]], "system": "dust"}}' || fail "the size over life was not set"
ctl scene act "$craft" particles '{"over": {"color": [[0, "#ffffff"], [1, "#ff000000"]], "system": "dust"}}' || fail "the color over life was not set"
ctl scene act "$craft" particles '{"render": {"brightness": 2.5, "fps": 8, "frames": "random", "mix": true, "stretch": {"max": 40, "min": 2, "time": 0.1}, "system": "dust"}}' ||
  fail "the look of the particles was not set"
ctl scene act "$craft" particles '{"create": {"system": "lone"}}' || fail "no lone system was created"
ctl scene act "$craft" particles '{"delete": {"system": "lone"}}' || fail "a lone system was not deleted"
ctl scene preview "$craft" -o preview:1 --time 0.5 || fail "a scene with every particle field was not previewed"
ctl scene act "$craft" layer '{"transform": {"layer": "card", "position": [100, -20], "rotation": -15, "scale": [2, 2], "size": [300, 200]}}' || fail "the layer was not placed"
ctl scene act "$craft" layer '{"blend": {"blend": "screen", "layer": "card"}}' || fail "the blend mode was not set"
ctl scene act "$craft" layer '{"transform": {"layer": "card", "tilt": [20, -10], "z": 120}}' || fail "the layer was not tilted"
ctl scene act "$craft" keyframe '{"pointer": "/scene/layers/0/transform/scale", "time": 0, "value": [1, 1]}' >/dev/null || fail "the scale took no keyframe"
ctl scene act "$craft" layer '{"transform": {"at": 1, "layer": "card", "scale": [2, 2]}}' || fail "the keyed scale took no keyframe at a second"
python3 -c "import json, sys; k = json.load(open(sys.argv[1]))['layers'][0]['transform']['scale']['keys']; sys.exit(0 if [1, [2, 2]] in k else 1)" "$craft/scene.json" ||
  fail "the keyed scale has no keyframe at 1 s"
ctl scene act "$craft" layer '{"transform": {"layer": "card", "replace": true, "scale": [3, 3]}}' || fail "the keyframes were not replaced"
ctl scene act "$craft" set '{"pointer": "/scene/particles/dust/emitters/0/rate", "value": {"keys": [[0, 10], [1, 30]]}}' >/dev/null
ctl scene act "$craft" particles '{"settings": {"rate": 20, "replace": true, "system": "dust"}}' || fail "the keyframed rate was not replaced"
ctl scene act "$craft" layer '{"instance": {"layer": "dust", "values": {"alpha": 0.5, "color": "#ffcc88", "count": 0.25}}}' || fail "the instance was not set"
ctl scene act "$craft" layer '{"instance": {"layer": "dust", "values": {"count": null}}}' || fail "the instance value was not unset"
ctl scene act "$craft" manifest '{"license": {"license": "CC-BY-4.0"}}' || fail "the license was not set"
ctl scene act "$craft" manifest '{"authors": {"authors": ["Ana", "Luis"]}}' || fail "the authors were not set"
ctl scene act "$craft" manifest '{"tags": {"tags": ["sea", "night"]}}' || fail "the tags were not set"
ctl scene act "$craft" manifest '{"tags": {"remove": true, "tags": ["night"]}}' || fail "the tag was not removed"
ctl scene act "$craft" manifest '{"origin": {"derived_from": [{"author": "Ana"}], "kind": "derived"}}' || fail "the origin was not set"
ctl scene act "$craft" manifest '{"display": {"values": {"accent": "#ff8800", "span": "all-outputs"}}}' || fail "the display was not set"
ctl scene act "$craft" add_property '{"id": "speed", "kind": "slider"}' >/dev/null
ctl scene act "$craft" property '{"default": 4, "id": "speed", "max": 10, "min": -1, "order": 0, "step": 0.5}' || fail "the property range was not set"
ctl scene act "$craft" canvas '{"bloom": {"radius": 24, "strength": 2}}' || fail "the canvas does not glow"
ctl scene act "$craft" canvas '{"parallax": {"amount": 1.5, "gyro": true}}' || fail "the canvas has no parallax"
ctl scene import "$craft" "$ROOT/tests/fixtures/green-64x32.png" --id sheet --atlas 2,4 >/dev/null || fail "no sprite sheet was imported"
ctl scene import "$craft" "$ROOT/tests/fixtures/green-64x32.png" --id bumps --linear >/dev/null || fail "no linear picture was imported"
ctl scene preview "$craft" -o preview:1 --time 0.5 || fail "a scene with authored layers, canvas and manifest was not previewed"
ctl scene act "$craft" add_layer '{"kind": "solid", "id": "generic"}' >/dev/null || fail "scene act added no layer"
ctl scene act "$craft" set '{"pointer": "/scene/canvas/background", "value": "#204060"}' || fail "scene act set no value"
[ "$(scene_doc "$craft" /scene/canvas/background)" = '"#204060"' ] || fail "scene act did not change the background"
echo '[{"hide": {"id": "generic", "hidden": true}}, {"lock": {"id": "generic", "locked": true}}]' | ctl scene act "$craft" batch - || fail "scene act did not apply a list from the standard input"
scene_doc "$craft" /scene/ext/editor | grep -q generic || fail "the batch did not lock the layer"
ctl scene act "$craft" undo || fail "a batch was not undone"
if scene_doc "$craft" /scene/ext/editor 2>/dev/null | grep -q generic; then fail "a batch was not undone as one step"; fi
ctl scene actions add_layer | grep -q "kind: string (required)" || fail "scene actions did not describe add_layer"
ctl scene preview "$craft" -o preview:1 --time 0.5 --quality q1 --battery --calm || fail "a preview on battery with reduced motion was refused"
ctl scene preview "$craft" -o preview:1 --time 0.5 --without-bloom || fail "a preview without the glow was refused"
ctl scene preview "$craft" -o preview:1 --stop
shots=$scene/shots
mkdir -p "$shots"
ctl scene render "$craft" --out "$shots/full.png" --size 96 --time 0.5 >/dev/null || fail "the scene was not rendered to a picture"
ctl scene render "$craft" --out "$shots/low.png" --size 96 --time 0.5 --quality q0 --battery >/dev/null || fail "the scene was not rendered at the lowest level on battery"
for shot in full low; do [ -s "$shots/$shot.png" ] || fail "the picture $shot is empty"; done
ctl scene render "$craft" --out "$shots/clip.gif" --size 64 --frames 4 --fps 10 --pointer 0.2,0.5:0.8,0.5 --click >/dev/null || fail "the clip was not rendered"
[ "$(head -c 3 "$shots/clip.gif")" = GIF ] || fail "the clip is no animated picture"
ctl scene render "$craft" --out "$shots/seq.png" --size 64 --frames 3 >/dev/null || fail "the image sequence was not rendered"
for n in 0001 0003; do [ -s "$shots/seq-$n.png" ] || fail "the image sequence is missing frame $n"; done
ctl scene render "$craft" --out "$shots/versus.png" --size 96 --time 0.5 --against q0 --without-bloom --split 0.25 >/dev/null || fail "the comparison was not rendered"
[ -s "$shots/versus.png" ] || fail "the comparison picture is empty"
if ctl scene render "$craft" --out "$shots/bad.png" --pointer 2,0.5 2>/dev/null; then fail "a pointer outside the canvas was taken"; fi
if ctl scene render "$craft" --out "$shots/bad.txt" 2>/dev/null; then fail "a picture was written with an unknown extension"; fi
lic=$scene/licensed
ctl scene new "$lic" --id org.example.licensed >/dev/null
ctl scene act "$lic" set '{"pointer": "/manifest/license", "value": "CC-BY-SA-4.0"}' >/dev/null
mkdir -p "$scene/author-keys"
(cd "$scene/author-keys" && ctl repo keygen ana --no-password >/dev/null && ctl repo keygen bo --no-password >/dev/null)
if (cd "$scene/author-keys" && ctl repo keygen ana --no-password 2>/dev/null); then fail "a key pair was replaced unasked"; fi
ctl pack sign "$lic" --key "$scene/author-keys/ana.key" >/dev/null || fail "the scene was not signed"
ctl pack build "$lic" -o "$scene/licensed.wpak" >/dev/null || fail "the signed scene was not built"
ctl pack validate "$lic" --key "$scene/author-keys/ana.pub" | grep -q "^ok scene org.example.licensed" || fail "the signature of the folder was not accepted"
ctl pack validate "$scene/licensed.wpak" --key "$scene/author-keys/ana.pub" | grep -q "signer=" || fail "the signature of the package was not accepted"
if ctl pack validate "$scene/licensed.wpak" --key "$scene/author-keys/bo.pub" 2>/dev/null; then fail "another key vouched for the package"; fi
ctl pack check "$lic" --json | grep -q '"warnings"' || fail "pack check of the folder gave no report"
ctl scene open "$scene/copy" --from "$scene/licensed.wpak" --facts | grep -q '"new_id":"org.example.licensed.copy"' || fail "the facts of the copy were not printed"
ctl scene open "$scene/copy" --from "$scene/licensed.wpak" --id org.example.mycopy >/dev/null || fail "a copy was not unpacked"
[ "$(scene_doc "$scene/copy" /manifest/id)" = '"org.example.mycopy"' ] || fail "the copy kept the id of the original"
[ "$(scene_doc "$scene/copy" /manifest/license)" = '"CC-BY-SA-4.0"' ] || fail "the copy lost the license of the original"
[ "$(scene_doc "$scene/copy" /manifest/origin/kind)" = '"derived"' ] || fail "the copy does not credit the original"
if ctl scene open "$scene/same" --from "$scene/licensed.wpak" --id org.example.licensed 2>/dev/null; then fail "a copy took the id of the original"; fi
ctl scene act "$lic" set '{"pointer": "/manifest/license", "value": "CC-BY-ND-4.0"}' >/dev/null
rm -f "$lic/manifest.json.minisig"
ctl pack build "$lic" -o "$scene/nd.wpak" >/dev/null || fail "the scene without derivatives was not built"
if ctl scene open "$scene/nd-copy" --from "$scene/nd.wpak" 2>/dev/null; then fail "a copy of a package without derivatives was unpacked"; fi
[ ! -e "$scene/nd-copy/manifest.json" ] || fail "a refused copy left files"
ctl scene open "$scene/nd-copy" --from "$scene/nd.wpak" --mine >/dev/null || fail "the author could not edit a copy of their own package"
echo "package review, key and copy checks passed"
