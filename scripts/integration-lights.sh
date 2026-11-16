#!/usr/bin/env bash
set -euo pipefail
declare draft ROOT
ctl scene act "$draft" add_layer '{"id": "wall", "kind": "image", "source": "leaf"}' >/dev/null || fail "no image layer was added"
ctl scene act "$draft" light '{"add": {"at": [40, 30], "id": "lamp", "light": "spot"}}' || fail "no spot light was added"
ctl scene act "$draft" light '{"set": {"aim": [80, 60], "color": "#ffcc88", "cone": [20, 40], "intensity": 3, "layer": "lamp", "radius": 200, "tilt": 30}}' ||
  fail "the spot light was not set"
ctl scene act "$draft" light '{"add": {"id": "tube", "light": "tube"}}' || fail "no tube light was added"
ctl scene act "$draft" light '{"set": {"at": [60, 40], "layer": "tube", "length": 120, "z": 30}}' || fail "the tube light was not set"
scene_doc "$draft" /scene/layers | grep '"tilt": \[' >/dev/null || fail "the tilt of the spot light was not kept"
ctl scene act "$draft" light '{"mute": {"layer": "tube", "muted": true}}' || fail "the tube light was not muted"
ctl scene act "$draft" light '{"solo": {"layer": "lamp"}}' || fail "the spot light was not lit alone"
ctl scene act "$draft" light '{"solo": {}}' || fail "the solo light was not let go"
ctl scene act "$draft" light '{"mute": {"layer": "tube"}}' || fail "the tube light was not brought back"
ctl scene act "$draft" normals '{"asset": "leaf", "id": "leaf_normals", "layer": "wall", "strength": 3}' ||
  fail "no normal map was made"
scene_doc "$draft" /scene/layers | grep '"leaf_normals"' >/dev/null || fail "the layer does not take the normal map"
ctl scene act "$draft" lighting '{"emissive": 0.5, "layer": "wall", "reflection": 0.2, "roughness": 0.3, "strength": 0.9}' ||
  fail "the lighting of the layer was not set"
ctl scene act "$draft" lighting '{"emissive_map": "leaf", "layer": "wall"}' || fail "the glow map was not picked"
scene_doc "$draft" /scene/assets | grep '"leaf_linear"' >/dev/null || fail "the picked map was not read as linear"
ctl scene act "$draft" lighting '{"emissive_map": "", "layer": "wall"}' || fail "the glow map was not cleared"
ctl scene act "$draft" paint_map '{"layer": "wall", "map": "roughness", "points": [[0.2, 0.5], [0.8, 0.5]], "radius": 0.1, "value": 0.2}' ||
  fail "no roughness was painted"
ctl scene act "$draft" paint_map '{"layer": "wall", "map": "emissive", "points": [[0.5, 0.5]], "radius": 0.2}' || fail "no glow was painted"
ctl scene act "$draft" ambient '{"ambient": "#202020", "sky": "#6080ff"}' || fail "the ambient light was not set"
ctl scene act "$draft" light '{"add": {"at": [80, 20], "id": "beam", "light": "spot"}}' || fail "no second spot light was added"
ctl scene act "$draft" light '{"volumetric": {"density": 0.4, "layer": "beam", "samples": 24}}' || fail "the spot light did not light the air"
ctl scene act "$draft" light '{"volumetric": {"layer": "beam", "shafts": 0.6}}' || fail "the air of the spot light took no shafts"
ctl scene act "$draft" light '{"projection": {"asset": "leaf", "layer": "beam"}}' || fail "the spot light did not project a picture"
ctl scene import "$draft" "$ROOT/tests/fixtures/magenta-64x32.webm" --id film >/dev/null || fail "no video was imported"
ctl scene act "$draft" light '{"projection": {"asset": "film", "layer": "beam"}}' || fail "the spot light did not project a video"
ctl scene act "$draft" light '{"projection": {"asset": "leaf", "layer": "beam"}}' || fail "the spot light did not go back to the picture"
ctl scene act "$draft" remove '{"pointer": "/scene/assets/film"}' || fail "the video was not removed"
ctl scene prune "$draft" | grep magenta >/dev/null || fail "the video was not pruned"
ctl scene act "$draft" particles '{"lit": {"lit": true, "normal": "leaf_normals", "system": "dust"}}' || fail "the particles do not take the light"
ctl scene act "$draft" lighting '{"emissive": 2, "layer": "dust", "strength": 0.6}' || fail "the particles layer took no strength or glow"
ctl scene act "$draft" particles '{"refract": {"amount": 0.3, "normal": "leaf_normals", "system": "dust"}}' || fail "the particles do not refract"
ctl pack check "$draft" >/dev/null || fail "the scene with lit air and particles is not valid"
for f in volumetric_lights light_textures particle_lighting; do
  scene_doc "$draft" /manifest/requires/features | grep "$f" >/dev/null || fail "the scene left $f out"
done
ctl scene preview "$draft" -o preview:1 --time 0.5 || fail "lit air and particles were not previewed"
ctl scene act "$draft" particles '{"refract": {"off": true, "system": "dust"}}' && ctl scene act "$draft" particles '{"lit": {"lit": false, "normal": "", "system": "dust"}}'
ctl scene act "$draft" lighting '{"layer": "dust", "off": true}' || fail "the particles layer kept its lighting"
ctl scene act "$draft" light '{"projection": {"layer": "beam", "off": true}}' && ctl scene act "$draft" light '{"volumetric": {"layer": "beam", "off": true}}'
ctl scene act "$draft" light '{"remove": {"layer": "beam"}}' || fail "the second spot light was not removed"
ctl pack check "$draft" >/dev/null || fail "the lit scene is not valid"
scene_doc "$draft" /manifest/requires/features | grep lighting2d >/dev/null || fail "lighting left lighting2d out"
ctl scene preview "$draft" -o preview:1 --time 0.5 || fail "a lit scene was not previewed"
ctl scene act "$draft" light '{"remove": {"layer": "tube"}}' || fail "the tube light was not removed"
ctl scene act "$draft" light '{"remove": {"layer": "lamp"}}' || fail "the spot light was not removed"
ctl scene act "$draft" ambient '{"clear": true}' || fail "the ambient light was not cleared"
ctl scene act "$draft" remove_layer '{"id": "wall"}' || fail "the lit layer was not removed"
ctl pack check "$draft" >/dev/null || fail "removing the lights left the scene invalid"
