# Scripts

A script is a JavaScript file that goes inside the scene. You write it and the scene runs it on
every frame. It is for what the properties of `scene.json` cannot reach: a calculation, a random
choice, a state that changes with time or with a click.

Before writing one, check whether what the scene already has is enough. Almost everything that
moves with time, sound or the pointer is declared in `scene.json` without code (`scene.md`,
`layers.md`, `properties.md`). The script is for logic.

Scripts, by topic:

- [Script basics](scripts/basics.md): what a script can do, the language and adding one.
- [Exports, events and inputs](scripts/exports-and-events.md): the functions a module exports and the events it gets.
- [The scene from a script](scripts/interface-scene.md): `engine`, `time`, `props`, `audio`, `input` and `scene`.
- [More of the script interface](scripts/interface-more.md): sounds, timelines, particles, storage, folders, media, shared data, timers and math.
- [Test and debug](scripts/testing.md): trying a script and finding what fails.
- [Complete examples](scripts/examples.md): whole scripts to start from.

## Next

- `package.md`: how to package, sign and publish the scene with its scripts.
- `cli.md`: the commands this page uses.
- `properties.md`: the properties that `props` reads.
- `actions.md`: what the editor can do on a scene, also from the command line.
