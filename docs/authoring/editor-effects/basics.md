# Adding effects

On this page: what each thing is, adding effects and arranging them.

## What each thing is

- An **effect** changes how a layer is drawn: it blurs it, tints it, warps it, moves it. A layer can have several; each one works on the result of the previous one.
- A **parameter** is a setting of an effect (a radius, a color, an image, a point).
- A **mask** limits where the effect is seen, or where the whole layer is seen.
- An **adjustment layer** draws nothing of its own: it applies its effects to everything below it.
- A **package effect** is one that you write yourself, with a descriptor (the list of its parameters) and a shader in WGSL code. It travels inside the scene.

## How to add an effect to a layer

**Where it is:** the inspector of a layer (any but a sound layer), in the button bar at the top.

**Steps:**

1. In the **Layers** tab, choose the layer.
2. In the inspector, click **Add an effect**.
3. The **Add an effect** dialog opens. Type in the search box to find an effect by its name or its description, or go through the list, grouped by category.
4. Click the effect. It is added at the end of the layer's effect list and the dialog closes. **Cancel** closes the dialog without adding anything.

The **Add** button in the **Effects** row of the inspector does the same: it opens this dialog and creates nothing until you choose an effect.

Each entry in the list shows the effect's name, its description (in Spanish if the system is in Spanish) and its cost, for example “(cost 2.2)”. The cost is the work it asks of the graphics card; see [How to see what an effect costs](adjust-and-cost.md#how-to-see-what-an-effect-costs).

The effects you write in the package appear in the same list, with their category and their description.

**In the terminal:** the `effects` action with `add`; also `reiwand-ctl scene act DIR effects`. The catalog list comes from `reiwand-ctl scene catalog DIR [--category CATEGORY] [--json]`. See [actions](../actions.md).

## How to see the effects of a layer, reorder them and remove them

**Where it is:** in the layer's inspector, the **Effects** field.

**Steps:**

1. Choose the layer. The **Effects** field says how many items it has (“2 items”).
2. Open the **Edit…** dropdown and choose an **Item** (1, 2, 3…). The inspector switches to that effect.
3. With the effect chosen, the buttons of the inspector bar are:
   - **Up**: goes back to the layer.
   - **Raise** and **Lower**: change the effect's place in the list. Changing the order changes the result, because each effect works on what the previous one left.
   - **Remove** (the trash icon): removes the effect.
4. The fields of the chosen effect are **Effect** (switches to another effect), **Enabled** (a switch to turn it off without removing it), **Name** (a name of your own, optional), **Optional** and **Mask**, and below them its parameters.

**Optional:** if the scene is too heavy for the computer, the program first skips the effects marked **Optional**. See [How to see what an effect costs](adjust-and-cost.md#how-to-see-what-an-effect-costs).

**In the terminal:** the `effects` action with `add`, `set` (parameters, name, **Enabled**, **Optional**), `remove` and `move`; also `reiwand-ctl scene act DIR effects`. See [actions](../actions.md).
