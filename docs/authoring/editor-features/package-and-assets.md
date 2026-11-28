# The package, properties and assets

- **Package**: *Package details* brings the manifest into the inspector, which is also generated
  from its schema (`reiwand-ctl scene schema --manifest`): title, description and notes of this
  version in each language (*Translate* adds one from the main language's text without touching
  those already there, and `@key` texts are written in `strings`), version, id, authors (up to 64),
  license (an SPDX expression of up to 256 characters), rating (for all or for adults, with its
  reasons), whether it flashes, origin and the works it derives from, credits per file, thumbnail
  and animated preview (*Choose…* copies the image into `assets/` and shrinks the thumbnail if it
  exceeds 1024 pixels or 1 MiB), links, tags, default language and the inputs it can ask for. The
  inputs the scene uses are marked by themselves and cannot be removed, and `requires.features` is
  computed on every change as in `reiwand-ctl pack build`: it is never written by hand; the inputs
  that the editor added go away when nothing uses them. Like `reiwand-ctl pack check`, *Problems*
  says whether the desktop and the phone refuse it or play it without some feature. A new scene
  takes the desktop's language as its main language.
- **Properties and presets**: *User properties* lists them in the order of the form, each group
  with its members below, and continues with the presets in the same list. Properties of any type
  can be added (slider, switch, color, selection, text, file, folder, button, group, label or
  separator) with no text in any language, so they show their id until you give them a label; they
  are raised and lowered, put into a group or taken out (groups do not nest), renamed (the links,
  triggers, groups, conditions and presets that name them change with them; scripts do not, and you
  are warned about those that name it) and removed together with their values in the presets and in
  the map conditions, except while a link, a trigger or an expression names them: then it says
  where. The inspector edits each field of its type: step, unit, decimals, checkbox for the number
  and logarithmic scale of a slider, values, labels and icons of the options, color with
  transparency, secret texts, collapsed groups, files it accepts and default asset, folders with
  their order and interval, icon and order, and shows the `"@key"` texts with those of `strings`.
  `visible_if` and `enabled_if` are written as an expression of the conditions language or as a
  JSON map and are checked while you type; only a valid condition is saved, and a problem with a
  condition or a preset leads to its field. *Presets* lists the package's presets and *Add a preset*
  creates another, with a row for each property they can set. Only properties with a value can be
  linked, and a timeline's trigger is chosen among `start`, `beat`, the buttons and those
  properties. A value that follows a selection or a switch (a layer's image, a mask, the texture of
  some particles, a video or a color) shows the property that drives it and a row per option to
  choose its variant. `requires.features` follows the new types and fields that are used.
- **Assets**: it imports images, videos, sounds, fonts and `.cube` tables within the format's
  limits and deletes the files the scene no longer uses. *Import an effect…* (`reiwand-ctl scene
  import-effect`) copies an effect of your own, its descriptor and its WGSL shader (next to it or
  chosen separately, as `--wgsl`), to `effects/`, checks it as when reading the package and says
  what to change if it does not pass; afterwards it is added to layers like the built-in ones, with
  its name in your language, and its parameters appear in the inspector with their label and help
  in your language, their limits, the images or layers they can read and their variants, linkable to
  properties, keyframes or scripts. *Import a script…* (`reiwand-ctl scene act DIR import_script`)
  copies a `.js` module into `scripts/` and proposes its declaration: the events it exports or
  handles with `on` and the parts of `reiwand` it imports or calls; it is worth reviewing before
  publishing. When the same module is imported again, it keeps its `params` and says what changed in
  the declaration. *Convert to KTX2…*, on each still image, converts it to a KTX2 texture with Basis
  Universal (UASTC, sharper, or ETC1S, smaller) with a quality control: it shows the file size
  before and after, the memory it takes on each card class and a crop of how each one draws it (BC7
  on desktop, ASTC on phones, ETC2 on old phones and uncompressed on the others). It is a step that
  can be undone; `reiwand-ctl scene asset DIR ID` shows the same and
  `--ktx2 [--codec uastc|etc1s] [--quality N]` converts, or only reports with
  `--dry-run [--looks DIR]`. When loading, the scene passes each KTX2 texture to the compressed
  format the card reads and leaves it that way in memory, with the file's levels as mipmaps; the
  image keeps its exact size even if a side is not a multiple of 4 pixels, because the texture is
  padded up to the next block and only its real pixels are drawn, also when the image is a mask, a
  light map, a reflection or a projection. An animated image (GIF or WebP) becomes an array of
  frames that keeps the time of each frame; an HDR image (`.hdr` or `.exr`) a HDR texture that keeps
  its light above white; and a panorama, or six square images chosen one per face, a cube map
  (`--cube` or `--faces` in `reiwand-ctl scene asset`). Each card class that keeps an uncompressed
  form says so. HDR KTX2 files are also read (BC6H on desktop, ASTC HDR where the card reads it and
  float halves on the others), which gain range on `hdr` canvases; frame arrays, which are seen as
  sprite sheets and animated with `animated`; and cube maps, which are seen as equirectangular
  panoramas. The assets panel says on each KTX2 whether it is HDR, how many frames it has or whether
  it is a cube, and `reiwand-ctl scene asset` says so too. The crops for each card class are drawn
  by the graphics card itself when it reads that format and decoded by the processor when it does
  not, and each crop says which it was. The phone does not convert to KTX2: it has no scene editor
  and only carries the Rust transcoder, not Basis Universal's C++ encoder; packages made on the
  desktop look the same on it.
