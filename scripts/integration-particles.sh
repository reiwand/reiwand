#!/usr/bin/env bash
set -euo pipefail
declare draft
ctl scene act "$draft" add_layer '{"id": "dust", "kind": "particles"}' >/dev/null || fail "no particles layer was added"
ctl scene act "$draft" particles '{"point": {"angle": 15, "at": [40, 30], "follow": false, "point": 1, "system": "dust"}}' || fail "no control point was placed"
ctl scene act "$draft" particles '{"shape": {"emitter": 0, "shape": {"around": {"axis": [0, 0, 1], "at": "p1", "count": 12, "mode": "mirror", "radius": 20}}, "system": "dust"}}' ||
  fail "the emitter was not put around a point"
ctl scene act "$draft" particles '{"shape": {"emitter": 1, "shape": {"between": {"arc": {"amount": 0.2}, "count": 8, "from": "p0", "to": "p1"}}, "system": "dust"}}' ||
  fail "no emitter between two points was added"
ctl scene act "$draft" particles '{"layer": {"color": true, "emitter": 1, "id": "badge", "inherit": true, "refresh": "on_change", "system": "dust", "threshold": 0.4}}' ||
  fail "the emitter does not take a layer"
ctl scene act "$draft" particles '{"remap": {"remap": {"in": [0, 1], "input": "random", "out": [2, 6], "output": "size"}, "system": "dust"}}' ||
  fail "no remap was added"
ctl scene act "$draft" particles '{"remap": {"index": 0, "remap": {"in": [0, 100], "input": "distance:p1", "out": [1, 0], "output": "alpha"}, "system": "dust"}}' ||
  fail "the remap was not set"
ctl scene act "$draft" particles '{"remap": {"remap": {"in": [0, 1], "input": "daytime", "out": [0, 90], "output": "spin"}, "system": "dust"}}'
ctl scene act "$draft" particles '{"remap": {"index": 1, "remove": true, "system": "dust"}}' || fail "the remap was not removed"
ctl scene act "$draft" particles '{"cutout": {"range": [0.3, 0.7, 0.9], "system": "dust"}}' || fail "the alpha was not cut out"
ctl scene act "$draft" particles '{"orient": {"angle": 30, "orient": "fixed", "system": "dust"}}' || fail "the sprites were not turned"
ctl scene act "$draft" particles '{"perspective": {"focal": 600, "system": "dust"}}' || fail "no focal length was set"
ctl scene act "$draft" particles '{"start": {"ranges": {"depth": [0, 300, 2], "depth_speed": [-10, 10, null]}, "system": "dust"}}' || fail "no depth was given"
ctl scene act "$draft" set '{"pointer": "/scene/particles/spark", "value": {"capacity": 10, "emitters": [{"shape": {"point": [0, 0]}}]}}'
ctl scene act "$draft" set '{"pointer": "/scene/particles/dust/children", "value": [{"on": "follow", "system": "spark"}]}'
ctl scene act "$draft" particles '{"inherit": {"child": 0, "follow": true, "inherit": ["color", "size"], "system": "dust"}}' || fail "the child took nothing from its parent"
ctl scene act "$draft" particles '{"shader": {"system": "dust"}}' || fail "the shader was not cleared"
ctl scene act "$draft" particles '{"turbulent": {"system": "dust", "turbulent": {"audio": "bass", "kind": "relative", "unit": "db", "forward": 90, "scale": 0.5, "speed": [20, 40]}}}' || fail "no turbulent start was set"
ctl scene act "$draft" particles '{"turbulent": {"quiet": true, "system": "dust", "turbulent": {}}}' || fail "the turbulent start still follows the sound"
ctl scene act "$draft" particles '{"offset": {"offset": {"distance": 12, "octaves": 3, "sign": [0, 1]}, "system": "dust"}}' || fail "no fractal offset was set"
scene_doc "$draft" /manifest/requires/features | grep particle_depth >/dev/null || fail "depth left particle_depth out"
ctl scene preview "$draft" -o preview:1 --time 0.5 || fail "a scene with newer particles was not previewed"
ctl scene act "$draft" particles '{"perspective": {"off": true, "system": "dust"}}' || fail "the depth was not removed"
ctl scene act "$draft" particles '{"collider": {"collider": {"bounce": 0.5, "bounds": {"box": {"from": [0, 0], "inside": true, "to": [400, 300]}}}, "system": "dust"}}' ||
  fail "no box collider was added"
ctl scene act "$draft" particles '{"collider": {"collider": {"bounds": {"segment": {"from": [0, 200], "to": [400, 200]}}}, "system": "dust"}}' ||
  fail "no segment collider was added"
ctl scene act "$draft" particles '{"response": {"index": 1, "response": "stick", "system": "dust"}}' || fail "particles do not stick to the segment"
ctl scene act "$draft" particles '{"response": {"bounce": 0.7, "friction": 0.1, "index": 0, "response": "bounce", "system": "dust"}}' || fail "the bounce was not set"
ctl scene act "$draft" particles '{"collider": {"collider": {"bounds": {"layer": {"id": "badge"}}, "kill": true}, "index": 1, "system": "dust"}}' ||
  fail "the layer collider was not set"
ctl scene act "$draft" particles '{"collider": {"index": 1, "remove": true, "system": "dust"}}' || fail "the collider was not removed"
ctl scene act "$draft" particles '{"force": {"force": {"keep_distance": {"distance": 30, "strength": 5, "target": "p1"}}, "system": "dust"}}' || fail "no distance is kept"
ctl scene act "$draft" particles '{"force": {"force": {"keep_between": {"from": "p0", "strength": 2, "to": [40, 30]}}, "system": "dust"}}' || fail "particles are not kept between two points"
ctl scene act "$draft" particles '{"force": {"force": {"keep_distance": {"distance": 80, "strength": 4, "target": "cursor"}}, "index": 1, "system": "dust"}}' || fail "the distance was not replaced"
ctl scene act "$draft" particles '{"force": {"index": 2, "remove": true, "system": "dust"}}' || fail "the force was not removed"
ctl scene act "$draft" particles '{"boids": {"boids": {"alignment": 2, "max_speed": 120, "radius": 30}, "system": "dust"}}' || fail "the particles do not flock"
ctl scene act "$draft" particles '{"force": {"force": {"remap": {"in": [0, 3], "input": "age", "out": [1, 0], "output": "alpha"}}, "system": "dust"}}' ||
  fail "no remap operator was added"
ctl scene act "$draft" particles '{"force": {"force": {"remap": {"in": [0, 2], "input": "age", "op": "mul", "out": [4, 1], "output": "size"}}, "index": 3, "system": "dust"}}' ||
  fail "the remap operator was not set"
ctl scene act "$draft" particles '{"strand": {"settings": {"subdivide": 3, "uv_scroll": 0.5, "width": 5}, "strand": "rope", "system": "dust"}}' ||
  fail "no rope was drawn"
ctl scene act "$draft" particles '{"pace": {"max_per_step": 1, "system": "dust"}}' || fail "the emitters were not paced"
scene_doc "$draft" /scene/particles/dust/emitters/1/max_per_step | grep -x 1 >/dev/null || fail "an emitter was not paced"
ctl scene act "$draft" particles '{"strand": {"settings": {"length": 48, "segments": 8, "subdivide": 2, "textured": true}, "strand": "trail", "system": "dust"}}' || fail "no long trail was drawn"
ctl scene act "$draft" particles '{"texture": {"system": "dust"}}' || fail "the texture was not cleared"
ctl scene preview "$draft" -o preview:1 --time 0.5 --quality q1 --restart || fail "the particles were not previewed at a lower quality"
ctl scene preview "$draft" -o preview:1 --time 0.5 --quality q1 --restart || fail "the particles were not restarted"
ctl scene act "$draft" particles '{"turbulent": {"off": true, "system": "dust", "turbulent": {}}}' && ctl scene act "$draft" particles '{"offset": {"off": true, "offset": {}, "system": "dust"}}'
ctl scene act "$draft" particles '{"force": {"index": 3, "remove": true, "system": "dust"}}' && ctl scene act "$draft" particles '{"boids": {"boids": {}, "off": true, "system": "dust"}}'
ctl scene act "$draft" particles '{"strand": {"off": true, "strand": "rope", "system": "dust"}}' || fail "the rope was not removed"
scene_doc "$draft" /manifest/requires/features | grep particle_ropes >/dev/null || fail "long trails left particle_ropes out"
