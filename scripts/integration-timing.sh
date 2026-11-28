#!/usr/bin/env bash
set -euo pipefail
declare scene ROOT WORK
timed=$scene/timed
ctl scene new "$timed" --id org.example.timed >/dev/null
ctl scene act "$timed" add_layer '{"id": "card", "kind": "solid"}' >/dev/null
ctl scene act "$timed" add_layer '{"file": "'"$ROOT"'/tests/fixtures/parts.glb", "id": "body", "kind": "mesh"}' >/dev/null || fail "no mesh layer came from a .glb file"
ctl scene act "$timed" clip '{"add": true, "id": "wave", "layer": "body", "play": true}' || fail "no clip was added that plays"
[ "$(scene_doc "$timed" /scene/layers/1/animations/0/clip)" = '"wave"' ] || fail "the new clip does not play"
ctl scene act "$timed" timeline '{"add": true, "duration": 4, "id": "intro", "loop": "repeat"}' || fail "no timeline was added"
ctl scene act "$timed" timeline '{"add_events": [{"name": "boom", "t": 1.5}, {"name": "spark", "t": 3.0}], "id": "intro"}' || fail "the timeline took no events"
ctl scene act "$timed" timeline '{"add_events": [{"name": "boom", "t": 2.0}], "drop_events": ["boom@1.5"], "id": "intro"}' || fail "an event was not moved"
[ "$(scene_doc "$timed" /scene/timelines/0/events/0/t)" = 2.0 ] || fail "the moved event is not at 2 s"
ctl scene act "$timed" keyframe '{"pointer": "/scene/layers/0/opacity", "time": 0, "timeline": "intro", "value": 1}' || fail "no keyframe on the timeline"
ctl scene act "$timed" keyframe '{"base": 0.75, "pointer": "/scene/layers/0/opacity", "time": 3, "value": -0.5}' || fail "the keyframes took no base"
ctl scene act "$timed" keyframe '{"out_handle": [1, 0], "pointer": "/scene/layers/0/opacity", "time": 0}' || fail "the keyframe took no handle"
ctl scene act "$timed" keyframe '{"in_handle": [1, 0], "pointer": "/scene/layers/0/opacity", "time": 3}' || fail "the keyframe took no handle into it"
ctl scene act "$timed" keyframe '{"pointer": "/scene/layers/0/transform/position", "time": 0, "value": [0, 0]}' >/dev/null || fail "the position took no keyframe"
ctl scene act "$timed" keyframe '{"pointer": "/scene/layers/0/transform/position", "time": 2, "value": [100, 50]}' >/dev/null || fail "the position took no second keyframe"
ctl scene act "$timed" keyframe '{"out_handle": [1, 0.5], "pointer": "/scene/layers/0/transform/position", "time": 0}' || fail "a position keyframe took no handle"
ctl pack check "$timed" >/dev/null || fail "keyframes with handles left the scene invalid"
ctl pack check "$timed" >/dev/null || fail "refused keyframe edits left the scene invalid"
ctl scene act "$timed" timeline '{"id": "intro", "speed": 2, "to": "opening"}' || fail "the timeline was not renamed"
[ "$(scene_doc "$timed" /scene/layers/0/opacity/timeline)" = '"opening"' ] || fail "the keyframes lost their timeline"
ctl scene preview "$timed" -o preview:1 --time 0.5 || fail "a scene with relative and curved keyframes was not previewed"
ctl scene act "$timed" timeline '{"id": "opening", "remove": true}' || fail "the timeline was not removed"
ctl scene act "$timed" keyframe '{"absolute": true, "pointer": "/scene/layers/0/opacity", "time": 0}' || fail "the base was not dropped"
printf 'import { scene } from "reiwand";\nexport function update() { scene.parallax = { amount: 2 }; }\n' >"$WORK/tilt.js"
ctl scene act "$timed" import_script '{"file": "'"$WORK"'/tilt.js"}' >/dev/null || fail "the camera script was not imported"
scene_doc "$timed" /scene/scripts | grep '"camera"' >/dev/null || fail "a script that moves the parallax does not declare camera"
