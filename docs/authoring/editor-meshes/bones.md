# Meshes and bones

On this page: what a mesh is, making one, bones, weights, depth and reaching a target.

## What a mesh is

A mesh is a layer that can be deformed with bones. All the tools on this page are in the mesh field of the inspector (it is called "Mesh") and draw over the preview. A mesh layer can have up to 64 pieces. The tools read a single image from the file: if the `.glb` has several, their texture-coordinate views use the first one. The bones are the joints; each bone moves the vertices according to their weights.

## How to make a mesh from an image

**Where it is:** **Layers** tab > **Add a layer** > **Mesh from a picture…** (it is off until there is an image in the assets).

**Steps:**

1. Import the image in **Assets** > **Import…**.
2. Choose **Mesh from a picture…**. The window of the same name opens.
3. Pick the image in the list.
4. Check **Follow the outline of the picture** if you want the mesh to follow the edge of what is drawn and leave out what is transparent. Unchecked, it is cut in a grid.
5. Set **Cells along the longer side** (from 2 to 64; default 16) and accept the dialog.
6. The layer appears with the image as its texture. Now place the bones, as the window says: **Then place its bones in the inspector and drag them on the preview.**

If you already have a model file (`.glb`) in the assets, **Add a layer** > **Mesh** creates a mesh layer with it and it is edited the same way. A **3D model** layer does not have bone placement: its bones come from its file, are seen in the inspector and are posed by its clips (see [editor-3d](../editor-3d.md)).

**In the terminal:** `mesh_from_picture` (`image`, `outline`, `cells`, `id`).

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","outline":true,"cells":12,"id":"doll"}'
```

## How to place, move, rename and remove bones

**Where it is:** the mesh layer's inspector, **Bones** list.

**Steps:**

1. Type the name in **Name of a new bone**, choose its parent in the list (**No parent** or a bone) and click **Add**. The bone starts at the center of the mesh and is left chosen.
2. Click **Place bones on the preview** and drag the head of each bone. Lines join each bone with its parent. Click a bone (in the list or on the preview) to choose it.
3. To change the name, double-click the name or click **Rename**.
4. To change the parent, drag a row's handle onto another bone. For it to hang from none, drop it on **Drop a bone here to hang it from no other bone**.
5. **Remove** deletes the bone. If the weights were already painted, those of that bone pass to its parent.

When you add, move or remove a bone, the weights are recomputed by themselves as long as you have not painted them. After painting them they are kept: a bone you add later starts with no weight.

A mesh can have up to 128 bones with this tool.

**In the terminal:** `bone` (`layer`, `name`, and `at`, `parent`, `to` to rename, `remove`). `at` is a position in fractions of the mesh (0.5 and 0.5 is the center). The `rig` (bones, points, parts and clips of the layer) and `handles` queries give the state.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","cells":12,"id":"doll"}'
reiwand-ctl scene act my-scene bone '{"layer":"doll","name":"body","at":[0.5,0.7]}'
reiwand-ctl scene act my-scene bone '{"layer":"doll","name":"arm","parent":"body","at":[0.8,0.4]}'
reiwand-ctl scene act my-scene bone '{"layer":"doll","name":"arm","to":"hand"}'
reiwand-ctl scene ask my-scene rig '{"layer":"doll"}'
```

## How to paint the bone weights

Weights say how much each bone moves each vertex.

**Where it is:** the mesh's inspector, **Paint weights on the preview** (at least one bone is needed).

**Steps:**

1. Click **Paint weights on the preview**. A bone is chosen if there was none.
2. Choose the bone in **Bone**, or click one on the preview or in the list.
3. Choose the **Brush**: **Add**, **Subtract**, **Smooth** or **Normalise**. Adjust **Strength**, **Brush size** and **Soft edge**.
4. In **Show** choose **Weight of the bone** (the mesh is tinted from blue, no weight, to red, all the weight) or **Bone that leads each part** (each triangle with the color of the bone that weighs most).
5. In **Locked bones** check the bones whose weight must not change while you paint.
6. Drag over the mesh. Each stroke is a step.
7. **Auto weights** recomputes all the weights from the position of the bones. The text next to it says whether the weights follow the bones or are kept.

**In the terminal:** `weights` (`layer`, `bone`, `mode`, `by`, `radius`, `falloff`, `lock`, `points`) and `auto_weights`. The `weights` query with the bone's name gives the heat map.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","cells":12,"id":"doll"}'
reiwand-ctl scene act my-scene bone '{"layer":"doll","name":"body","at":[0.5,0.7]}'
reiwand-ctl scene act my-scene bone '{"layer":"doll","name":"arm","parent":"body","at":[0.8,0.4]}'
reiwand-ctl scene act my-scene weights '{"layer":"doll","bone":"arm","mode":"add","by":0.5,"radius":0.15,"points":[[0.7,0.4],[0.8,0.4]]}'
reiwand-ctl scene act my-scene auto_weights '{"layer":"doll"}'
```

## How to give depth to a mesh

Depth makes the nearer vertices cover the farther ones.

**Where it is:** the mesh's inspector, below the weights.

**Steps:**

1. Check **Depth: nearer vertices cover farther ones**.
2. Move **Perspective of the depth** (from 0 to 10). It can also be dragged from the preview: a line at the bottom has a square with the value above it.
3. Check **Hide faces seen from behind** if you want to hide what faces backward.
4. **Draw the parts from back to front, so soft edges show what lies behind** comes checked.
5. To bring vertices nearer or move them away, click **Paint depth on the preview**, set **Pixels towards the viewer** (from -1000 to 1000; negative moves away) and **Brush size**, and drag over the mesh.

**In the terminal:** `mesh_depth` to paint (`layer`, `by`, `radius`, `points`) and `parts` with `depth` (`scale`, `cull`, `sort`, `off`) for the settings.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","cells":12,"id":"doll"}'
reiwand-ctl scene act my-scene parts '{"layer":"doll","depth":{"scale":2}}'
reiwand-ctl scene act my-scene mesh_depth '{"layer":"doll","by":20,"radius":0.15,"points":[[0.5,0.5]]}'
```

## How to make a chain of bones reach a target

A chain (inverse kinematics, "ik") bends several bones so that the last one reaches a target.

**Where it is:** the mesh's inspector, **Chains that reach**.

**Steps:**

1. Click **Make a chain on the preview** (two or more bones are needed).
2. Click the bone where the chain starts. Then, the bone where it ends, below the first.
3. The chain appears in the list, with its bones separated by arrows. Choose what it reaches: **Reaches for A point**, **Reaches for The pointer**, a bone or point of the mesh, or another layer.
4. **Weight** blends the effect with the normal pose (from 0 to 1).
5. **Bend the other way** changes the direction it bends. On the preview the round button next to the chain does it too: **Bends clockwise; click to bend the other way**.
6. To limit a bone's rotation, choose it in the chain's list and click **Limit**; **Free** removes the limit. Each limit is an arc around the bone with a point at each end: drag them to change the lesser and the greater angle.
7. A dashed line joins the chain's end with its target; if the target is a point, drag the ring to move it.

If the chain follows the pointer, the scene needs the pointer permission and warns the user.

**In the terminal:** `ik` (`layer`, `root`, `tip`, `target`, `index`, `bend`, `weight`, `limits`, `unlimit`, `remove`). `target` is `"cursor"` (the pointer), `"bone:NAME"` (a bone or point of the mesh), `"layer:ID"` (another layer) or a point `[x, y]` in canvas pixels.

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene mesh_from_picture '{"image":"picture","cells":12,"id":"doll"}'
reiwand-ctl scene act my-scene bone '{"layer":"doll","name":"body","at":[0.5,0.7]}'
reiwand-ctl scene act my-scene bone '{"layer":"doll","name":"arm","parent":"body","at":[0.8,0.4]}'
reiwand-ctl scene act my-scene ik '{"layer":"doll","root":"body","tip":"arm","target":[900,500]}'
reiwand-ctl scene act my-scene ik '{"layer":"doll","index":0,"bend":"counterclockwise","weight":0.8,"limits":{"arm":[-45,45]}}'
```
