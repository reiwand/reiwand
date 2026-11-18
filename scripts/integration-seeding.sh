#!/usr/bin/env bash
set -euo pipefail
declare scene WORK
craft=$scene/craft
ctl scene act "$craft" particles '{"emit": {"emitter": 0, "system": "dust", "values": {"audio_gate": true, "bursts": [{"count": 20, "every": 2}], "delay": 0.5, "follow": "cursor", "inherit": 0.3, "periodic": {"off": [0.5, 1], "on": [1, 2]}}}}' ||
  fail "the emitter was not set"
ctl scene act "$craft" particles '{"emit": {"clear": ["bursts"], "emitter": 0, "system": "dust", "values": {"audio_gate": false}}}' || fail "the emitter values were not removed"
[ "$(scene_doc "$craft" /scene/particles/dust/emitters/0/follow)" = '"cursor"' ] || fail "the emitter does not follow the pointer"
ctl scene act "$craft" particles '{"start": {"ranges": {"hsv": {"hue": [180, 240], "sat": [0.5, 1], "steps": 6}}, "system": "dust"}}' || fail "the particles took no hues"
ctl scene act "$craft" particles '{"start": {"ranges": {"life": [1.0, 3.0, 2.0]}, "system": "dust"}}' || fail "the life took no curve"
ctl scene act "$craft" particles '{"start": {"ranges": {"life": [1.0, 4.0, null]}, "system": "dust"}}' || fail "the ends of the life were not set"
[ "$(scene_doc "$craft" /scene/particles/dust/init/life | tr -d " \n")" = '[1.0,4.0,2.0]' ] || fail "setting the ends dropped the curve"
ctl scene act "$craft" particles '{"start": {"ranges": {"palette": {"colors": ["#ff0000", "#00ff00"], "noise": [0.1, 0, 0]}}, "system": "dust"}}' || fail "the particles took no palette"
ctl scene act "$craft" particles '{"start": {"ranges": {"gradient": [[0, "#000000"], [1, "#ffffff"]]}, "system": "dust"}}' || fail "the particles took no gradient"
if scene_doc "$craft" /scene/particles/dust/init/palette >/dev/null 2>&1; then fail "the palette stayed beside the gradient"; fi
ctl scene act "$craft" particles '{"start": {"clear": ["gradient"], "system": "dust"}}' || fail "the gradient was not removed"
ctl scene act "$craft" particles '{"window": {"index": 0, "system": "dust", "window": [0.0, 0.2, 0.8, 1.0]}}' || fail "the force took no window of the life"
ctl scene act "$craft" particles '{"force": {"force": {"gravity": [0, 200]}, "index": 0, "system": "dust"}}' || fail "the force was not replaced"
[ "$(scene_doc "$craft" /scene/particles/dust/forces/0/window | tr -d " \n")" = '[0.0,0.2,0.8,1.0]' ] || fail "replacing the force dropped its window"
ctl scene act "$craft" particles '{"window": {"index": 0, "system": "dust"}}' || fail "the window was not removed"
ctl scene act "$craft" particles '{"settings": {"lock": ["size", "speed"], "system": "dust"}}' || fail "nothing was locked"
ctl scene act "$craft" particles '{"render": {"cycles": 2, "frames": "life", "order": "oldest", "system": "dust"}}' || fail "the cycles and order were not set"
ctl scene act "$craft" particles '{"force": {"force": {"turbulence": {"mask": [0.2, 0.8], "scale": 0.01, "strength": 5, "time_scale": 2}}, "system": "dust"}}' || fail "the turbulence took no mask"
for force in '{"noise": 30}' '{"cap": 200}' '{"torque": -90}' '{"spin_drag": 0.5}' \
  '{"orbit": {"center": "p0", "radius": 200, "strength": 40}}' \
  '{"vortex": {"axis": [0, 0, 1], "center": "cursor", "inner": 5, "ring": {"pull": 2, "radius": 80, "width": 20}, "strength": -60}}' \
  '{"slow_near": {"factor": 0.5, "inner": 10, "outer": 60, "target": "p0"}}'; do
  ctl scene act "$craft" particles '{"force": {"force": '"$force"', "system": "dust"}}' || fail "no $force force was added"
done
ctl scene act "$craft" layer '{"instance": {"layer": "dust", "values": {"points": [[10.0, 20.0], [30.0, -40.0]]}}}' || fail "the instance took no control points"
ctl scene act "$craft" layer '{"instance": {"layer": "dust", "values": {"points": {"0": null, "2": [5.0, 6.0]}}}}' || fail "one control point of the instance was not moved"
scene_doc "$craft" /scene/layers | tr -d ' \n' | grep -q '"points":\[null,\[30.0,-40.0\],\[5.0,6.0\]\]' || fail "moving one point of the instance changed the others"
printf 'const system = "sparks";\nexport function update() { return system; }\n' >"$WORK/sparks.js"
ctl scene act "$craft" import_script '{"file": "'"$WORK"'/sparks.js"}' >/dev/null || fail "the script that names the system was not imported"
ctl scene act "$craft" particles '{"rename": {"system": "sparks", "to": "embers"}}' 2>"$WORK/renamed.err" || fail "the system was not renamed"
grep -q 'scripts/sparks.js:1 names "sparks"; scripts are not rewritten' "$WORK/renamed.err" || fail "renaming did not warn about the line of the script that names the system"
grep -q 'scene act DIR particles' "$WORK/renamed.err" || fail "renaming did not offer to rewrite the names"
[ "$(scene_doc "$craft" /scene/particles/dust/children/0/system)" = '"embers"' ] || fail "the child did not follow the new name"
looks_at "$craft" -o preview:1 --time 0.5 || fail "a scene with every emitter and color field was not previewed"
