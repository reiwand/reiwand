# Creating, saving and undoing

On this page: creating a scene, editing a copy, saving, recovering and undoing.

## How to create a scene

Where it is: the **Scenes** page, at the top.

Steps:

1. Type the **Title:**. It comes with **My scene**.
2. Look at the **Folder:**. It fills in by itself with the first free name inside the `reiwand` folder of your Documents, taken from the title (`my-scene`, and if that is taken `my-scene-2`, and so on). You can type another path. The folder has to be empty or not exist; otherwise the editor says “… is not an empty folder”.
3. Click the starting point you want. The scene is created and opens in the editor.

The package id comes from the folder (`org.example.` and the folder name without accents). You can change it later in the **Options** tab.

| Starting point | What it brings | Opens in |
|---|---|---|
| **Empty** | A canvas with no layers. | This editor |
| **Particles** | A sky with a gradient and falling snow, with a **Density** property to change how much falls. | This editor |
| **Layered parallax** | A sky, a sun and three hills that move differently with the pointer. | This editor |
| **Text clock** | A sky and two texts with the hours and minutes and the seconds, with a **Color** property. | This editor |
| **Resource pack** | A package without a scene that only holds parts to share (layers, particles, scripts, effects). It is edited like a scene. | This editor |
| **Web page** | A web page as the wallpaper. | The package page ([package](../package.md)) |
| **Preset** | The values of another package that another package takes as a starting point. | The package page |
| **Image** | A package of one image. | The package page |
| **Video** | A package of one video. | The package page |

The title's language is that of your desktop; if your language does not have the labels of the example properties, the English ones are copied.

In the terminal: `reiwand-ctl scene new DIR --from POINT [--title TITLE] [--id ID] [--locale LANGUAGE]`. The starting points are called `empty`, `particles`, `parallax`, `clock`, `assets`, `web`, `preset`, `image` and `video`. See [cli](../cli.md).

## How to edit a copy of an installed scene

An installed scene is never changed. A copy is made in a folder of yours and that one is edited.

Where it is:

- On the lit card of a scene in the library or in Explore (if it is installed): the **Edit a copy…** button. A right click or the menu key on the card offer it too.
- On the wallpaper's options page (**Options**), at the end.
- In **Create**, the **Learn from an example** tab: the examples come inside the app.

It always makes a copy as a new project, also if you made the scene yourself: the original does not change. To continue with a project of yours, open it from **Create** (recent row or **All my projects**).

Steps:

1. Choose the package. The **Edit a copy** dialog opens. It explains that the copy is unpacked into a folder of its own with an id of its own, that the installed package does not change, that it keeps its license and its credits and that it cites the original as the work it comes from.
2. Look at the **Package:** line (title, type and version).
3. In **Folder:** leave the one it proposes or click **Choose…**.
4. In **Id of the copy:** leave the one it proposes (the original's id with `.copy` after it) or type another.
5. If the license asks for something, the dialog says so: **The license … asks for: …**.
6. If the license does not allow derivative works, a notice appears: **The license … allows no derived works. Edit a copy only if you made this package or its author allows it.** Check **I made this package or its author allows a copy** only if it is true. Without the check, the button is not enabled.
7. Click **Edit a copy**. The editor opens with the copy.

The copy loses the original author's signature, because it is no longer their package.

In the terminal: `reiwand-ctl scene open DIR --from FILE.wpak [--id ID] [--mine]`. With `--facts` it only shows what the license allows and the id of the copy, without unpacking.

## How to save and recover changes

Where it is: **Save now** in the top bar, **File** › **Save now** and **Ctrl+S**.

Steps:

1. Click **Save**. The button is off while there are no changes or the editor is busy.
2. If any file in the folder changed since you opened it (for example, another program touched it), the editor does not overwrite it: the notice **Another program changed this folder while you were editing. Your changes are not lost: choose what to keep.** appears with three buttons: **Keep mine** (saves your version on top), **Take theirs** (discards yours and loads the folder as it is) and **Save elsewhere…** (saves yours aside and leaves the folder as the other program left it).

The editor looks at the folder every two seconds: if the outside change happens when you have nothing unsaved, it reloads the scene and the problems list by itself, without asking.

Shortcut: your system's save shortcut (see the [shortcut table](shortcuts.md#all-keyboard-shortcuts)).

What happens to what you do not save:

- Every 30 seconds, when you click **Back** and when you close the editor, the unsaved changes are set aside in an editor file, outside your folder, together with the undo history.
- When you open the folder again, if those changes are still there and start from the same files, the yellow notice **This folder has unsaved changes from an earlier session.** appears with two buttons: **Recover** brings them back (with their history) and **Discard** deletes them.
- That file disappears when you close the editor with everything saved.

In the terminal: there is no save button because each action saves immediately. The unsaved changes that the app set aside are recovered with `reiwand-ctl scene recover DIR`, or discarded with `--discard`.

## How to undo and redo

Where it is: **Undo** and **Redo** in the top bar and in **Edit**, which names the step. Each one is off when there is nothing to undo or redo.

Steps:

1. Click **Undo**. When you hover, the help says which step is about to be undone: **Undo: <step>**.
2. Click **Redo** to apply the step again. The help says **Redo: <step>**.

How steps are counted:

- Dragging a slider or a canvas handle is a single step, however long the drag.
- A typed value, moving a layer to another place, adding or removing something are one step each.
- If you undo and make a different change, the undone steps are lost.
- The history keeps at most 500 steps.
- A change that changes nothing (for example, setting the same value again) leaves no step.

Shortcut: your system's undo and redo shortcuts (see the [shortcut table](shortcuts.md#all-keyboard-shortcuts)).

History list: the editor has no on-screen list of steps. The closest thing is the help of the two buttons, which names the next step.

In the terminal: the `undo` and `redo` actions (`reiwand-ctl scene act DIR undo '{}'`) and the `reiwand-ctl scene history DIR` command, which lists the steps that undo and redo walk through and marks the undone ones. The history is the same in the app and in the terminal while there are unsaved changes. See [actions](../actions.md).
