# The `reiwand-ctl` authoring commands

`reiwand-ctl` is Reiwand's command-line program. These pages cover the commands used by people who
make scenes and packages: create, import, view, capture, check, package, sign, publish and run a
repository. Each one comes with a line that works and what it prints.

Each command explains its options with `--help` (`reiwand-ctl scene render --help`), in the
desktop's language. A command that fails exits with a code other than 0 and writes `Error: …`.

**With the daemon and without it.** Almost everything on these pages works without the daemon. Only
`scene preview`, `scene apply`, `scene thumbnail --capture`, `lib import`, `remote add`, `clear`,
`restore` and the `pkg` and `input` commands need it. The app starts it; with the app closed, the
`reiwandd` user service starts it.

**Folders.** A scene is a folder with `manifest.json`, `scene.json` and its files. Almost all the
`scene` commands start with `reiwand-ctl scene COMMAND DIR`.

In the examples, `my-scene` is a scene folder, `DP-1` the name of an output (screen) and
`org.example.my-scene` the id of a package.

The commands, by topic:

- [Create, change and import](cli/create.md): making a scene and changing it with commands.
- [Viewing the scene](cli/view.md): previews from the terminal.
- [Capture](cli/capture.md): images and recordings of a scene.
- [Check, package and publish](cli/check-and-publish.md): checks, packages, signatures and repositories.
- [The editor's actions from the terminal](cli/actions.md): `scene act`, `scene actions`, `scene ask` and other useful commands.
