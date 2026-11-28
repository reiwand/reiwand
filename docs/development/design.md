# Design guide

How Reiwand looks and why. This page is for contributors who change the desktop app (QML), the
phone app (Android, Kotlin) or the pages the server renders. The token tables are in
[docs/specs/look.md](../specs/look.md) and the way the tones are derived is in
[docs/specs/palette.md](../specs/palette.md); this page does not repeat them. It gives the rules
and says which piece to use.

## The idea

The window is the wallpaper you have on. The app is a title screen: a few confident words and a
shelf of large covers over the living picture. The interface is always dark (the editor alone may
be light, see Workbench). Every tone has a name
and comes from the wallpaper in use, except four fixed ones. There is no blur, no glass and no
decorative gradient: separation is a veil in the picture's own deepest tone or an opaque panel in
that tone.

- Menus, titles and the main action in Red Hat Display; running text in Onest; icons are Phosphor.
- One lit thing per view. The accent is scarce. State is words plus icon plus colour.
- Flat tonal layers with soft dark shadows, not outlines around everything.
- The voice is the user's: words, not walls of options. The technical part sits behind
  "Details", "More" or "Advanced".

### The three surfaces

All three share the tone names, the two typefaces, the Phosphor icons, the 6 px radius, the
4/8/12/16 spacing rhythm, the one-lit-thing rule, the three state tones, the main button in the
accent and the Frost keyboard focus ring.

- **Desktop** (`apps/reiwand-app`). Three kinds of window, all full-bleed:
  - *Title screens* (`TitleFrame`: Library, Explore, Lists, Sources, Create): menu words on the
    left over a veil, the wallpaper's title large at the top right, and a lower band with the
    type filter, the search and the shelf of covers. The focused cover is larger and fully lit.
  - *Options screens* (`OptionsFrame`: Settings, options of a wallpaper, list, source, Account):
    the picture shrinks to the strip of the menu and an opaque Night panel holds `ListRow`s, with
    sections and the key hints at its foot.
  - *Work surface* (`WorkFrame`: editor, review, publish): the picture becomes the canvas, the
    sides are Night panels. Authors only, desktop only.
  - *Small surfaces* (`SmallPanel`: tray, menu of one control, question, confirmation) are never
    a page; they open next to the control that opens them.
- **Phone** (`android/app/src/main/kotlin`). User functions only, never authoring. The wallpaper
  is full screen, the title and controls sit in the upper third and the shelf sits above three
  tabs (`Tabs`: Library, Lists, Settings). Type follows the system text size, touch targets are
  48 dp or more, text is 14 sp or more, `Sheet`s replace small panels, and there are no keyboard
  hints and no hover.
- **Server** (`apps/reiwand-server`). Pages are plain HTML and one column (`--column`, 46 rem).
  There is no wallpaper in use: the head of the page (`.band`) takes the latest published cover
  under a veil, or the plain night ground (`.band.bare`, `f-none`) while the repository is empty.
  Sizes are in rem and fluid, nothing is icon-only, a confirmation is a state of the page and
  there are no keyboard hints.

## Colours

One deep family, one light family, one accent per wallpaper, four fixed colours. The default set
below is what the code uses when no wallpaper gives its own (`Look.qml`, `Tones.NONE` on the phone,
`:root` in `style.css`).

| Tone | Hex | Use |
|---|---|---|
| Night | `#0a1322` | Page ground, panels, veils, text on lit rows and on the accent |
| High night | `#121e33` | Buttons at rest, fields, notices, key caps, second-level panels |
| Night line | `#2a3a57` | 1 px lines, rail tracks, pressed quiet button |
| Mist | `#b9c4d6` | Second-level text, captions, resting menu words and icons |
| Low mist | `#8f9db4` | Muted and disabled text, always with its reason in words |
| Moon (accent) | `#9cc3f5` | Main button, mark of the current place, "In use" |
| Deep moon | `#6f9fdc` | Pressed main button |
| Veil | 0.62 to 0.96 | Strength of the veil under text; a brighter picture needs more |
| Frost | `#f4efe6` | Main text, focus ring, the lit row and the ring of the lit cover |
| Amber | `#f0c26b` | Warns of something that changes the experience |
| Coral | `#ff9d8a` | An error or a destructive action |
| Sprout | `#8fd6a8` | Done, healthy |
| Amber, Coral, Sprout deep | `#7a5200`, `#a8361f`, `#1d7040` | The three on Frost (6.0, 5.7, 5.3:1) |

The first eight change with the wallpaper (`reiwand-palette` derives them); Frost and the state
tones never change.

The same names are used on every surface: `Look.night`, `Tones.night` on the phone, `--night` on
the server. The server spells the accent `--accent` and `--accent-deep`, and carries the veil as
`--veil` plus `--night-rgb` (phone: `Tones.veiled(alpha)`).

On the server the tones come from a generated stylesheet per cover (`band/sheet.rs`, `css`) and
the class `f-none` (`frame.rs`, `NO_WALLPAPER`) is the fixed set. A veil or a shadow is always
`rgb(var(--night-rgb) / alpha)`; never a loose black on the server.

### Rules

**The Own-Tones Rule.** Every tone comes from the wallpaper in use and has a name. Do not add a
colour to a page, a component or a Kotlin screen. If one is needed, it is a change to
`reiwand-palette`. The phone and the server have no colours of their own.

**The One-Lit-Thing Rule.** One thing is lit per view: the focused cover (Frost ring, no dimming),
the focused row (Frost ground, Night bold text) or the current tool. Everything else rests at Mist
over Night. When two screens exist, the chosen one is marked, not lit.

**The Scarce-Accent Rule.** Moon appears in three places only: the mark of the current place, the
main action and "In use". One main button per view (per state on the server). If the accent
lands on a fourth thing, the first three lose their meaning.

**The State-Tones Rule.** Amber, Coral and Sprout are for state only, always with an icon and
words beside them. They tint the icon, not large areas; the text stays Frost or Mist. Amber warns,
Coral fails or destroys, Sprout confirms. On a lit row (Frost ground) use the deep variants: in
QML ask `Tone.of(tone, lit, rest)` (`qml/Tone.qml`) instead of choosing the colour, and
`Tone.icon(tone)` for the icon. In the server CSS the classes `.good`, `.notice` and `.bad` on
`.status`, `.status-line`, `.note` and `.field.bad` do it.

**The Coral-Confirm Rule.** Only an irreversible action is confirmed, and its button is coral.
What can be undone is not confirmed; it gets a notice with "Undo".
- Desktop: the main `Button` of a deletion carries `palette.highlight: Look.coral`;
  `SmallPanel { destructive: true }` does it for a confirmation.
- Phone: `Kind.Remove` in `Buttons.kt` (filled Coral, Night text, with its icon), the only main
  button of that sheet.
- Server: `.button.danger` (coral text on High night), inside `.confirm` in the lit row. A coral
  fill would not read on Frost.
- A destructive action that is only offered (it still leads to the confirmation) is coral words,
  never filled.

**The Text-On-Deep Rule.** Body text is at least 4.5:1. Text over a picture sits only inside a
block with a veil: `BandVeil` and `TitleVeils` on the desktop, the title veil on the phone and
`.band::before` on the server. The palette crate guarantees the ratios of the tones
(`palette.md`); you guarantee that text is on Night or on a veil.

**The No-Blur Rule.** No blur, no `backdrop-filter`, no glass, no decorative gradient. Veils are
gradients of Night with alpha, nothing else, and never over the whole picture on a title screen.

**The Dim-Until-Lit Rule.** Resting covers are dimmed (`Look.coverVeil`, 0.42); the lit cover loses
the dimming and gains the ring and the big shadow.

## Typography

Red Hat Display (variable, `Look.titleFont`) carries menus, titles, buttons and the names of covers
and rows: confident, tightly tracked (-0.02em to -0.03em at large sizes). Onest (`Look.textFont`)
carries explanations, captions and values, with tabular figures. Both are self-hosted
under the SIL OFL.

### Desktop scale (`Look.t*`, pixels; compact / normal / roomy where it steps)

Wallpaper title `tCover` 52 / 66 / 80 bold. Screen title and empty state `tPage` 42 / 50 / 58 bold.
Large heading `tLarge` 26 / 34 / 38. Menu and title of the lit cover `tMenu`, `tFocus` 22 / 27 / 30
(bold when current). Row label `tRow` 20 / 22 / 24. `tSection` 20, `tAside` 16 or 18, `tFilter` 18.
All of these are Red Hat Display. In Onest: `tExplain` 17, `tName` 16, `tBody` 15 (running text),
`tFoot` 13.5, `tKey` 12.5. The button is `tButton` 16.5, bold, Red Hat Display.

`Look.tier` steps by window height: `short` up to 680, `compact` up to 800, `roomy` from
1800 x 1000, else `normal`. The window sets it and every measure that depends on it follows. A
page never reads the window size to pick a font size.

The editor's workbench is a tool surface and runs denser: while it is in view `Look.desk` is on
and every type size follows (`tBody`, `tName`, `tButton`, `tRow`, `tSection`, `tFilter` 13, `tFoot`,
`tAside`, `tExplain` 12, `tTool` 12.5, and the large ones capped at 13), with tool buttons 24 px,
fields 26 px, controls and buttons 28 to 30 px and menu options 30 px. Its frame, tab strips, rows
and blueprints take their measures from `DeskLook` (rows of 30 px, 13 px names, 12 px kinds).

### Phone (`Type` in `Type.kt`, sp, follows the system text size)

`titleL` 50, `titleM` 32, `titleS` 21, `empty` 26, `row` 17, `button` 16, `word` 16, `fold` 15.5,
`group` 14, `tab` 14, `text` 15.5, `foot` 14, `digits` 44 (the pairing code). Leading is 1.45 for
text and 1.3 for titles. Never go below 14 sp, and check the screen at the largest system text
size.

### Server (`style.css`, rem, fluid)

`--t-cover` 3.5 (2.5 on phone widths), `--t-section` 2 (1.625), `--t-row` 1.3125 (1.1875),
`--t-large` 1.1875, `--t-text` 1.0625, `--t-foot` 0.9375, `--t-small` 0.8125. Body line height is
1.55. Text is limited to `--measure` (68ch).

### Rules

**The Line-Height Rule.** Headings use `text-wrap: balance` and paragraphs `pretty`. Do not force
`<br>`, and leave no lonely last word. Put a non-breaking space between a number and its unit
(`2&nbsp;GB`).

**The Words-Not-Numbers Rule.** Give values in words ("Soft", "Medium", "Slow computer"), with the
number inside the selector. Quality numbers, fingerprints, hashes, paths and variables never
appear in the main view. They go behind "Details".

## Layout and spacing

Spacing is a scale and nothing else.

- **Desktop**: `Look.e1` to `e8` = 4, 8, 12, 16, 24, 32, 48, 64. `margin` 56, `menuWidth` 312,
  covers `cover` 256, `coverFocus` 420, `coverGrid` 232, `shelfGap` 28, always 16:9. `rowHeight`
  74, `rowsWidth` 660 with an `explainWidth` column of 300. `controlHeight` 40, `buttonHeight`
  42, `toolHeight` 32. The shelf aligns the bottoms of the covers; do not align them by hand.
- **Phone**: `Measure.e1` to `e10` = 4, 8, 12, 16, 20, 24, 32, 40 dp. Margin 20, touch 48, bar 64,
  row 64, option 56, field 52. Text over the picture sits only in the bands the title veil
  covers; the brightest point of a picture never sits under text.
- **Server**: `--e-1` to `--e-9` (0.25 to 6 rem). One column of `--column`, section gap `--e-8`
  (`--e-7` on phone widths), head `--band` 23 rem (17 on phone widths), control height 3 rem.
  Forms are 32 rem wide with the label above and the help and error below; the reason a button is
  disabled sits beside it (`.why`).

**The Size-From-Tokens Rule.** Use the tokens for tone, size and space: `Look`, `Measure` and
`Type`, or the `--` variables. A loose hex or pixel value in a page is a bug. If a value has no
token and will be used twice, add the token. The only singletons beside `Look` are `PreviewLook`
(measures of the switch-with-preview screen: reel of covers, badges, overlay timings) and
`PublishLook` (the numbered steps and the width of the publish screen) and `DeskLook` (the frame
of the editor's workbench); keep one-screen measures there.

**Notices have one place per surface.** They show one at a time: above the band on title screens,
at the foot of the panel above the keys on options screens, top left of the work area on the work
surface. Pages never position them (`OptionsFrame.noticeAt`, `WorkFrame.noticeAt`).

## Depth, shapes and icons

Depth is tonal layering: picture, veil, Night panel, High night element, Frost lit thing. Shadows
are soft and dark, never coloured, never hard-offset.

- Use the `Shadow` piece (`qml/Shadow.qml`) as a child of an opaque background. Its `kind` is
  `cover` (every cover), `focus` (the lit cover), `row` (the lit row) or `panel` (small panel,
  menu, notice). It exists only when the window draws with the graphics card, so the surface
  must look right without it. `Photo` rounds covers (`corner`, `lowCorner`).
- On the server use `--shadow` (covers, notes, images) and `--shadow-row` (the lit row).

Shapes are quiet and rectangular. `Look.radius` and `--radius` (6 px) for covers, buttons, rows,
panels, notices and fields; `radiusSmall` (4) for small icons, close buttons and thumbnails;
`keyRadius` (5) for key caps; `panelRadius` (10) for small panels and menus. The phone adds
`rNotice` (10 dp) and `rSheet` (22 dp). Round shapes are rare: the logo moon, radio buttons and
the selection badge of a chosen cover.

A mark of place is a small bar (`markWidth` x `markHeight`, 16 x 4, 1 px radius) beside or under
the current word; on the phone it sits on the top edge of the current tab. A focused thing gets a
2 px Frost outline with a 3 px gap (`ring`, `ringGap`), never a border. Lines are 1 px Night line.

Icons are Phosphor, in the `reiwand` icon theme: regular at rest, `-fill` when active or current,
`-bold` in key hints. Every icon travels with a word, except "close" (give it an accessible name)
and keys. A name the set lacks is not drawn, and the app tests fail on it
(`look.md`, "Icons").

Motion is `Look.pace` (180 ms) with `Look.paceCurve` on colour, background, shadow and position.
On the server transitions are off under `prefers-reduced-motion: reduce`.

## Components

Use the piece, not a copy of its look. The properties named here are the real ones.

### Buttons

- **Main**: `Button { highlighted: true }`. Moon ground, Night text, Red Hat Display bold, 42 px.
  Pressed goes Deep moon and drops 1 px. Only one per view, with its `Enter` key hint beside it
  on the desktop. Phone: `Action` (`Kind.Main`) pinned at the bottom of a sheet or page. Server:
  `.button.primary`.
- **Quiet**: the default `Button`, High night with a 1 px Night line border, bold title font.
  Phone `QuietAction`; server `.button`.
- **Word**: `Button { flat: true }`, words only, underline on hover. Phone `WordAction`; server
  `.button.link`.
- **Remove**: see the Coral-Confirm Rule.
- **Disabled**: always with its reason in words. Hardware that was not found stays visible, grey,
  with "not detected".
- **Focus**: keyboard focus is the Frost ring of the style, or `:focus-visible` /
  `.keyboard-focus` on the server. Never draw an outline of your own. Inside a clipped container
  leave `Look.ringRoom` so the ring is not cut.

### Menu words and filters

`MenuWord`: Red Hat Display, Mist at rest, Frost and bold when current, with the accent mark on
its left; Settings and Account sit apart. `Filter`: words with a small count, the current one with
the mark under it. Phone `Tabs`: three tabs, the current one with the mark and a filled icon.
Server `.nav`: the same words, with the mark under `.current`; "Sign out" is a form button.

### Cover

`Cover` (`Cover.qml`): 16:9 `Photo`, High night ground, dimmed at rest, name in Red Hat Display.
`lit: true` makes it larger, undimmed, ringed in Frost, with `Shadow { kind: "focus" }`, and the
title, caption and "Set as wallpaper" (the main button) below it. "In use" is the small accent
label inside the cover (`inUse`). `chosen` shows a round check badge. There is never a banner, a
featured cover, a rating or a popularity badge. Phone: `Cover` and `Thumb` in `Cover.kt`.

### Rows

`ListRow` (`ListRow.qml`) is one setting. Set `kind` for the value (words, switch, field, key
capture, rail, arrows, a caret for a second level), `explain` for the column that opens beside the
lit row, `off` for a disabled row, `status` and `statusTone` or `note` and `noteTone` for state.
Resting: Night, Mist label. Hover: High night. Lit (`lit`, from `activeFocus`): Frost ground,
Night bold text, deep state tones, `Shadow { kind: "row" }`. A disabled row is Low mist and
still shows why. All rows share `rowHeight` and one midline; `reiwand-app --check` fails otherwise.

Phone: `Setting`, `Group`, `Fold`, `Flip` and `Seam` in `Rows.kt`. Server `.rows > .row` with
`.row-name`, `.row-value`, `.row-action`: the name, the state in words (`.status`), a detail after
a "·", and the button that fixes it beside it. `.row.lit` or `:focus-within` lights it and every
child turns Night, except `.button.danger` (stays coral) and `pre.fact` (stays Mist).

### Panels and small surfaces

Opaque Night, 1 px line, `Shadow { kind: "panel" }`. `SmallPanel` has a cover header (`cover`),
`title`, `text`, `primary` and `quiet` buttons and a `foot`. Menus and drop-downs use the style's
`Menu` and `ComboBox` (38 px options, Night line, radius 10). On the phone use `Sheet` (handle,
title, note, `SheetOption` with a mark when chosen) and `Question` for a question; there are no
centred dialogs. On the server a confirmation is `.confirm` inside the lit row, with a line above.

### Workbench

The editor is a workbench of groups (`SceneGroup`): each holds panels as tabs on a strip, a panel
moves by dragging its tab or from its menu, and the tabs that do not fit go to the strip's "⋯".
Six factory layouts (Scene, Particles, Scripts, Simulation, Animation, Wallpaper; Ctrl+1 to
Ctrl+6) hold only the panels of their kind of work, four to seven, and keep the rest closed:
Window opens any of them where it usually sits (package pages that are not scenes start with
every panel they have). The person's changes are kept per layout. Each layout is drawn as its
**blueprint** (`SceneBlueprint`, from the same tree that builds it) beside its name in the top bar,
in Window and in the list of layouts, and the zone a dragged tab will take is painted in the same
language: no other picture names a layout. Below `DeskLook.folds` the
left side folds into a rail of icons whose panel unfolds over the canvas; F11 enlarges the panel
in focus and a panel can go to a window of its own.

- **Tones**: while the editor is in view (a scene's or another package's) its tones come from the
  thumbnail of the package being edited, the app's own set when it has none, with the same names
  (`SceneEditorTones`, `Look.tone`); leaving it gives back the tones the app had. **Light editor**
  is an editor-only preference (`EditorLight`, `editor.conf`, `[look] light`), in the editor's
  View menu and in Settings › Wallpapers › Making wallpapers: the same hues as paper surfaces and
  ink text (`reiwand_palette::paper`, `Look.light`), every text tone at 4.5:1 or more on every
  surface; lit rows become ink on paper and the focus ring takes the text tone. The rest of the
  app stays dark. Build editor pieces from the tone names only, so they read in both sets;
  `SceneTonesCheck` turns the light set on and off.

- **Value rows** (`SceneInput`, `SceneValueRow`): the name on the left, the editor beside it, the
  reset and keyframe buttons after it. On the workbench the names make one column
  (`DeskLook.label`, less the indent of a group), so editors start on one line; a row too narrow
  for it takes room from the name, never from the editor's floor.
- **Language rows** (Texts and languages): the language is a column of its own and sits, like the
  remove button, on the first line of the field, whatever its height; a field keeps its width when
  the remove button is hidden.
- **Buttons in a row** keep their place when they hide (opacity, not `visible`), so the eyes,
  locks and solos of the layers stay in columns.
- **Nothing sets the panel's width**: a row never asks for more than a narrow panel has (wrap the
  words, fold a button to its icon) and nothing reads a width it sets itself, which loops on a
  fractional scale. Content keeps clear of an overlaid scroll bar (`ScrollColumn`).
- **Type**: the window's title (15 px) is the largest; a panel's tab is 13 px and nothing inside
  a panel is larger: section heads 13 px semibold (`PanelHeading`), names, values and body 13 px,
  secondary words 12 px. Controls with no size of their own follow it (the window's font while the
  workbench is in view); `SceneTypeCheck` fails on larger words in any panel of any layout.
- **Rows of names and controls** in a panel use `SceneGrid` (two columns) or `SceneRowFit` (a row):
  their names wrap and their check boxes and drop-down lists narrow, down to a floor, before a
  control is pushed past the panel's edge.
- **Bars** give up words in turn when short of room: the top bar's Undo and Redo, then the layout
  names, then the words of its buttons; the canvas bar's Test the pointer and Capture.
- **Focus**: the item chosen is lit only in the group that has the keyboard, or had
  it last (`SceneFocus.group`); the other groups keep it marked in High night with a line
  (`marked` of the style's `ItemDelegate`, lit through `SceneFocus.lit`). The accent under the
  current tab is drawn only on that group. F6 and Shift+F6 go from group to group, Ctrl+Tab and
  Ctrl+Shift+Tab from tab to tab; on an edge the arrows move it 4 px, 40 px with Shift.

Check each panel at 1600 x 900 and 1366 x 768, at scale 1 and 1.25 (1280 and 1093 wide), in
every layout and package page.

### Notices, state, empty and error

- `Toast` (`Toast.qml`): High night ground with a line, an icon in the state tone (`tone`), one
  line, an optional `action`, a close button. One visible at a time, never blocking. Phone:
  `NoticeAsk` in `Notice.kt`. Server `.note` with `.good`, `.notice`, `.bad`.
- `StatusLine`: icon plus words (`tone`, `lit`). Server `.status` and `.status-line`.
- `Note`, `SmallNote`: a quiet line under a row or a panel.
- `Empty`: one big sentence and the first step as an action. Server `.empty`.
- `Failure` and `Details`: what happened, what to do, one button that fixes it, the technical
  part behind "Details". Server `.more` (`.more.fix` for a fixing step).
- `Progress`: words, a rail, and "Stop" when it can be stopped.

### Keys and fields

`KeyCap` and `Hints` (pairs of keys and words) sit at the foot of the band or the panel. Only
arrows, Enter, Esc and Tab; at most two hints on a title screen; Enter appears once, beside the
main button.

Desktop fields are the style's controls (`look.md`, "Controls"); a test fails if a screen uses a
control the style does not draw. Server forms: `form.form`, `.field` (label above, `.help`,
`.field-error`, `.field.bad`), `.checkbox`, `.choices > .choice`, `.file` (the native input is
never in view; a button plus `.file-name`), `.fact`, `.pairs` and `.copy` (a datum that selects
whole with one click, and a "Copy" button). The only script the server serves is
`assets/copy.js`; the page works without it.

## Do

- Take every tone from the wallpaper in use, through `Look`, `Tones` or the CSS variables.
- Light one thing per view; give the accent to the current place, the main action and "In use".
- Keep one main button per view, and a disabled control with its reason in words beside it.
- Write state in words with an icon and its tone; use `Tone.of` on lit rows.
- Confirm only what cannot be undone, with the coral button; offer "Undo" for the rest.
- Say what happened, put the button that fixes it beside the message and fold the technical part
  behind "Details".
- Keep touch targets at 48 dp and text at 14 sp or more on the phone.
- Use the user's vocabulary: "Set as wallpaper", "In use", "Slow computer / Fast computer".
  "Repository" stays "repository"; a source is a place that gives pictures. "Author key" is the
  secret that signs and is never just "key"; "signing" is the act and its result. The server also
  speaks of People (who enters, invitations, accounts) and Review (what people upload);
  Administration is only the health of the server.

## Don't

- Do not make it look like a store: no banners, featured covers, ratings, prices, download counts
  or autoplay carousels. The shelf is the person's own covers.
- Do not make it look like a technical panel: no ids, paths, fingerprints, hashes or `REIWAND_*`
  names in the main view.
- Do not wrap everything in a bordered card, or use loose colours or sizes in a page.
- Do not put an icon alone (except "close" and keys), or draw your own focus outline.
- Do not use keyboard hints or hover on the phone, or add authoring tools to it.
- Do not add an eyebrow or small-caps label above titles.

## Checking

Run `target/debug/reiwand-app --check` (it loads every QML file and checks row heights and focus
rings). Look at the app with a bright wallpaper, a dark one and the "no wallpaper" set; test the
phone at the largest system text size; load server pages with and without a cover, at phone width
and with the keyboard only.
