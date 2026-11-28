# Reaching and swinging

Each frame the pose is worked out in this order: the animation layers, the `rules`, the `ik`
chains in their order, what scripts set (see "Bones, blend shapes and attachments"), and the
`physics`. Rotations of bones are measured in degrees clockwise from their rest pose, as in clips.

A chain turns its bones so that its end (where the first child of its last bone starts, leaving
out nodes marked `extras.point`, or as far again along the last bone when it has none) reaches the
target; bodies of `physics` take the end of their bone, and ropes their links, the same way. Chains of two bones are solved
exactly, bending at their middle joint the way `bend` says; when `limits` stop one of their bones,
up to 10 rounds then turn each bone towards the target within its limits, so limits that only allow
the other side bend the middle joint that way. A longer chain takes at most 10 rounds that turn each
bone towards the target from the end back without letting any joint bend the other way, starting
from its pose, so the curl of its clips stays while it reaches. When that pose is straight or already
bends a joint the other way, the chain first curls into an even arc, every joint turning the way
`bend` says by the same angle, that reaches the target; when the rounds from the pose miss a target
within reach, the same frame tries again from that arc. A joint whose `limits` leave no room on the
`bend` side turns within its limits only, the other way, as in chains of two bones; a chain with
such a joint takes no arc. Each frame starts from its own pose, so a chain reaches the same way
live, after a seek or after a restart; a frame whose pose, limits and target are those of the frame
before takes its turns again without running any round. A target out of reach
stretches the chain towards it. `limits` keep each listed bone within `[min,
max]` degrees of its rest pose and `weight` blends the turns with the pose. `"cursor"` follows the
pointer only while the scene may read it (the pointer input, told to the user like any other use of
the pointer); without it the chain keeps its pose. `"layer:<id>"` follows the position of that layer.

Bodies of `physics` take one step per drawn frame, as long as the frame interval (1/120 s at
120 fps, 1/60 s at 60, 1/30 s at 30), on battery as well, so the motion matches at any of those
rates; a frame interval longer than 1/30 s is split into equal steps of at most 1/30 s, and a frame
catches up at most a quarter second, dropping longer gaps. Each step follows the pose of the
parent along the time it covers. A `spring` pulls the end of
its bone back to where the pose puts it (`stiffness`), a `rigid` bone swings like a pendulum held
loosely to its pose, and a `rope` links the bone and its first children (those without a body of
their own, at most 8 links) into a chain that hangs and sways. `inertia` is how much a body stays
behind when its parent moves, `damping` how fast it calms down, `gravity` pulls down (98 pixels per
second squared for each unit), `wind` is how much `canvas.wind` (in pixels a second, with gusts
taken at the time each step stands for, so they blow alike at every frame rate) pushes it, `rotation` bounds how far each link turns from its pose and `translation` lets the head of
the bone itself move that many pixels each way. A body that has not moved for half a second, while
its pose and the wind stay still, asks for no frames; at the lowest quality bodies keep the pose
of the animation and ask for nothing. Scripts can push bones with `impulse`.
