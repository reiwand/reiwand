# Newer particle fields

Systems may also use these fields. Most name `particles2`, which also covers scenes beyond the
older limits: up to 32 systems, 8 emitters, 16 forces and 8 colliders per system and 30 s of
`prewarm`. Control points are named `p0` to `p7`: `p0` is the origin of the system unless `points`
sets it, and every other name must be one of `points`. Targets and centers of forces, `around`,
`between` and `init.inherit` take a point, `"cursor"` or a control point.

The newer particle fields, part by part:

- [The newer fields](particles-newer-particle-fields/fields.md): every newer field with its meaning and feature.
- [Space, forces and children](particles-newer-particle-fields/space-forces-children.md): world space, control points, shapes, starting values, forces, emitters and children.
- [Looks and births](particles-newer-particle-fields/look-and-births.md): how particles look and where they are born.
- [Shaders, colliders and limits](particles-newer-particle-fields/shaders-colliders-limits.md): particle shaders, keeping distance, flocks, colliders, ropes, long trails, limits and instances.
