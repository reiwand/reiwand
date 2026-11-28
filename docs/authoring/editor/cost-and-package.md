# Cost, the package and the image editor

On this page: costs, making and applying the package and the image editor.

## How to see what something costs

Cost is measured in “units of work”: the higher, the heavier. It can be seen in four places.

**An effect on a layer.** Where it is: the inspector, when you choose the layer's effect, under the **Cost** title.

1. Choose the level in the quality dropdown of that block (**Preview quality**). It is the same dropdown as the one in the level panel, so both change at once.
2. Look at the four rows: **Q3: 1.25 units of work**, or **Q0: left out of the scene at this level** if that level drops the effect.
3. The bold row is the level the daemon would really reach. With **On battery** checked and Q3 requested, it is Q2.

**All the effects at once**: the list in the **Preview level** panel, explained above.

**3D models and lights.** Where it is: the inspector's lights panel, **3D models** block (it only appears if the scene has models). For each level it gives the triangles, the 3D groups, the units of work and whether there are reflections. For each equipment class it says whether the model **fits**, **shows its poster** (a still image in its place) or is **too heavy**, with the texture memory. More detail in [3D](../editor-3d.md).

**Particles.** Where it is: the tools of a particle system, **Particle budget** block. It says how many particles are alive in the preview, where they are computed (graphics card or processor) and, with **Simulate on the processor and show the particles over the preview**, an estimate by the editor when no preview is running. **Restart systems** starts over. More detail in [particles and sound](../editor-particles-and-sound.md).

In the terminal: `reiwand-ctl scene ask DIR effect_cost`, `cost3d` and `budget`; `reiwand-ctl scene catalog DIR` lists all the effects with their cost. See [actions](../actions.md).

## How to create the package and apply it

Before publishing, the **How it will look** panel, in the **Wallpaper** layout, shows the package as a cover in the library: its thumbnail, title and kind and, when the panel is wide enough, another cover next to it at rest, dimmed as the covers you have not chosen are. **Update the thumbnail** takes it from the canvas, and **Record 6 s** (scenes and videos) records six seconds from the instant the preview is on as the animated preview.

Where it is: **Publish…** in the top bar, in a scene and in any other package. Next to it, **View as wallpaper** shows the scene on the desktop as a preview (a notice offers **Back to the editor**, **Keep it** and **Stop the preview**) and **Set as wallpaper** leaves it set (it asks on which screen when there is more than one and offers **Undo**). What the scene needs saved is saved first and the editor says so.

**Publish…** opens the **Publish “name”** page over the editor: on the left the thumbnail and the name of the package, on the right three numbered steps on the same page. A finished step folds into one line with **Change**, the next one opens by itself and **Enter** does its main action. **Back to the editor** closes the page without losing the step you were on.

1. **Review**: what the package will say to the world. It lists each problem, warning or note (with **Go to the problem**, which takes you to the field in the editor), where it plays, the **Version** and the **License** (with **Change**), and two questions you answer once: whether it has flashes and whether it has adult content. If the thumbnail is missing or older than the files, **Update the thumbnail** takes it again. **Go on** stays off, with the reason, while there are errors, unanswered questions or the license is missing. **More** checks the signature of an already built package.
2. **Your signature** the first time: with your author name and an optional password your key pair is created, and a notice offers **Save a copy…**. The following times the step is called **Sign it** and asks for the key's password if it has one; with a wrong password it offers **Manage keys…** and **Publish without signing**. A key without a password skips the step. **More** chooses another key or a key file, shows the fingerprint and saves a copy of the pair.
3. **Where?**: **Save the file** (the `.wpak` is left next to the folder with the id and the version as its name, or another path), **Upload to my server** (public or hidden; off if there is no connected server) or **Add to a repository in a folder** (a recent one, **Open another…** or **Create one here…**). It remembers the last destination. If something fails, the page stays and offers its way out: upload with the next version number, register your signature on the server, retry or save the file.

When it finishes, the **Ready** part says what to give the other person: the file with **Show in the folder**, the server link with **Copy the link** or the repository address with **Copy the address**. **Set as wallpaper** and **Back to the editor** close the page.

All this is explained in [package](../package.md).

In the terminal: `reiwand-ctl pack build DIR [-o FILE]` (sign first with `pack sign`) and `reiwand-ctl scene apply DIR [-o OUTPUT]`. The first refuses if there are errors (also a script that does not compile), and `scene apply` builds the scene into the library and shows it. To remove the wallpaper: `reiwand-ctl clear [-o OUTPUT]` and `reiwand-ctl restore`.

## The image editor

Library images have their own editor, different from the scene one. It changes the image without touching its file: it saves the edits separately.

Where it is: the card of a library image, the **Edit…** button. The window is titled **Edit <image>**.

Top bar: **Back**, **Save**, **Undo**, **Redo**, **Compare with the original** (a switch), **Show on the desktop**, **Export…** (saves the edited image as PNG, JPEG, WebP or AVIF) and **Reset all**.

Tools in the left column: **Crop and rotate**, **Screens**, **Light and color**, **Curves**, **Effects**, **Masks**, **Depth** and **Parallax**.

In the terminal: the `reiwand-ctl edit` group (`edit show`, `set`, `crop`, `curve`, `depth`, `parallax`, `undo`, `redo`, `export`…). See [cli](../cli.md).
