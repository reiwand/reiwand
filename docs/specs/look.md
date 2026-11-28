# Look of the desktop app

The app has a look of its own and is always dark. It does not follow the system's color scheme,
icon theme, font or control style: everything comes from the app.

## Tones

They live in the QML singleton `Look` (`apps/reiwand-app/qml/facade/Look.qml`). The first seven come
from the wallpaper in use: the window takes the poster of the wallpaper seen on its screen, draws
the tones with `reiwand-palette` (`Looks.derive`), stores them per wallpaper and applies them with
`Look.tone({...})`. Without a wallpaper in use, or if the poster cannot be read, the fixed "no
wallpaper" set applies. The window and its parts are described in `shell.md`.

| Property | Name | What for |
|---|---|---|
| `night` | Night | deep tone: surface of the window and the panels |
| `nightHigh` | High night | surface slightly lighter: buttons, fields, cards |
| `nightLine` | Night line | borders, separators, button with the pointer over it |
| `mist` | Mist | secondary text, icons of the words |
| `mistLow` | Low mist | muted text |
| `moon` | Moon | the accent: current place, main action, "In use" |
| `moonDeep` | Deep moon | the pressed accent |
| `veil` | — | opacity of the veil over the photo (`veilColor` is Night with that opacity) |
| `frost` | Frost | main text, focus ring (fixed) |
| `amber`, `coral`, `sprout` | Amber, Coral, Sprout | warnings, errors and done (fixed) |
| `amberDeep`, `coralDeep`, `sproutDeep` | — | backgrounds of those states (fixed) |

The application's palette is built with these tones and follows them when they change, so controls
without a style of their own and the Kirigami pieces (pages, dialogs, forms, titles, messages) also
take them. The Kirigami state colors of the window point to Amber, Coral and Sprout.

## Font

Two fonts go inside the app: `textFont` ("Onest", the application font) and `titleFont` ("Red
Hat Display", menus, titles and the main action). The scale is `tCover`, `tPage`, `tLarge`,
`tMenu`, `tFocus`, `tRow`, `tSection`, `tAside`, `tFilter`, `tExplain`, `tButton`, `tName`,
`tBody` (15 px, the running text), `tFoot` and `tKey`, in pixels.

## Measures

- Spacing: `e1` to `e8` (4, 8, 12, 16, 24, 32, 48, 64); `margin`, `marginTop`, `marginBottom`.
- Radii `radius` and `radiusSmall`; thicknesses `line`, `ring` and `ringGap` (gap of the focus
  ring).
- Sizes: `menuWidth`, `menuHeight`, `asideHeight`, `cover`, `coverFocus`, `coverGrid`, `shelfGap`,
  `controlHeight`, `buttonHeight`, `rowHeight`, `fieldHeight`, `rowsWidth`, `explainWidth`, `shortcutsHeight`,
  `switchWidth`, `switchHeight`, `switchDot`.
- Control measures: `toolHeight` (32, tool and small button), `optionHeight` (38, option of a menu),
  `panelRadius` (10, small panel), `railHeight` and `railDot` (4 px rail and its 14 dot), `mark`
  (18, checkbox and round option), `icon` (18, icon inside a control), `bar` (6, scroll bar) and
  the font `tTool` (14).
- Rows of buttons: every `Button` measures `controlHeight` (40), whether main, flat or quiet, so that a
  row has a single height and a single midline; tool buttons measure `toolHeight`. The top bar of the
  editor uses `topButtonHeight` on all its buttons.
- Ring room: `ringRoom` (`ringGap` + `ring`, 5 px) is how far the focus ring sticks out of
  the control. Every container that clips (lists, scrollables, editor panels) leaves that margin
  inside so that the ring is never cut, and the control with focus is drawn above its neighbors.
  The `RowsCheck` check of `reiwand-app --check` goes through each page and fails on rows of uneven
  height or center, icons shifted from their text and cut or covered rings; the exceptions are
  declared in that check with their reason.
- Duration `pace` (180 ms) with the curve `paceCurve`.
- Step: `tier` is `compact` up to 800 px of height, `roomy` from 1800 × 1000 and `normal` in the
  rest; the window sets it by itself and the measures that change with it change by themselves.

## Shadows

`Shadow` puts the soft shadow of a surface: it is placed as a child of the surface's background,
which must be opaque, and is drawn under it (`z: -1`) with the same `corner`. `kind` chooses the
shadow: `cover` (every cover), `focus` (the lit cover), `row` (the lit row) and `panel` (small
panel, menu of a control and notice). Example: `Rectangle { radius: Look.radius; color: Look.night;
Shadow { kind: "panel" } }`. It only exists when the window draws with the graphics card; with
software drawing it is not created and the surface stays as if without it. It does not work per
frame: it is drawn once and drawn again only if the surface changes.

## Rounded photos

`Photo` is an `Image` (the same properties, with `PreserveAspectCrop` and asynchronous loading by
default) that rounds its corners with `corner` (`Look.radius` by default); `lowCorner` changes only
the bottom two (the header of the small panel uses `corner: Look.panelRadius` and `lowCorner: 0`).
Example: `Photo { anchors.fill: parent; source: model.picture; sourceSize.width: Look.coverFocus }`. It
rounds only when the window draws with the graphics card and the image has already loaded; with
software drawing the photo is seen with square corners, like an `Image`.

## Icons

Icons are a set of our own (Phosphor, MIT license) inside the app, as the icon theme `reiwand`. An
icon is requested by its Phosphor name and weight: `house` (normal, at rest), `house-fill` (active)
or `house-bold` (shortcuts). The desktop names that the app already used are resolved by the table
`apps/reiwand-app/look/icons/names.txt`. No name falls back to the system theme: what the set does
not have is not drawn, and a test of the app fails if a screen asks for a name the set does not
have. Icons take the color of the text around them.

## Controls

The control style is our own (`io.github.reiwand.style`) and is always chosen on startup. It draws
every control that the app uses; a test of the app (`styled`) fails if a screen uses a control that
the style does not draw. All have rest, pointer over, pressed, disabled and the keyboard focus ring
in Frost. `reiwand-app --check` creates once each file of the style and fails naming the file that
does not load or does not create its control; it also compiles each QML file of the app and the
style and prints the error of each component that does not load (file, line and message).

- Button: quiet by default; `highlighted: true` is the main action (in Moon, title font); `flat:
  true` is a word button, without a surface. The main one of a deletion carries `palette.highlight:
  Look.coral`. Disabled, pressed, with the pointer over and with keyboard focus (ring in Frost) are
  drawn by themselves.
- Switch: `Switch`, 40 × 22 px; on in Frost with the dot in Night.
- Checkbox (`CheckBox`, `CheckDelegate`) and round option (`RadioButton`): of the switch's family;
  off in Night line, checked in Frost with the mark in Night.
- Text field (`TextField`, `TextArea`, also search and password): High night with a line, Frost
  font, help text in Mist; the ring appears when typing.
- Number (`SpinBox`): a field with the arrows ‹ › at the sides.
- Dropdown list (`ComboBox`) and menu (`Menu`, `MenuItem`, `MenuSeparator`): the small panel (Night,
  line, radius 10) with 38 px options; the option with the pointer over it in High night and the one
  chosen with the keyboard in Frost with Night font. The open list is as wide as its longest option,
  up to the edge of the window; a name that does not fit is cut in the middle (the beginning and the
  end are read) and the whole name comes out when leaving the pointer over it. The closed control
  cuts the chosen name the same way and shows it whole in the same help.
- Tool (`ToolButton`): word or icon in Mist without a surface; the one left chosen (`checked` or
  `highlighted`) in Frost with Night font.
- Tabs (`TabBar`, `TabButton`): filter words in title font; the current one in Frost with its mark
  in Moon.
- Rail (`Slider`, `RangeSlider`, `ProgressBar`, `Dial`): 4 px rail in Night line, filled in Frost,
  14 px dot.
- Thin scroll bars in Night line, which appear only when moving.
- Pop-up notice (`ToolTip`), pop-up window (`Popup`) and dialog (`Dialog`): panel in Night with a
  line; in the button row of a dialog, OK or Yes is the main action.
- Bar (`ToolBar`): High night with a line toward the content.

Licenses of the fonts and icons: `apps/reiwand-app/look/fonts/` and `apps/reiwand-app/look/icons/`.

## The Plasma tray

The tray panel and the app share the same font, the same tone names, the same measures and the same
icons, and take them from a single source: `scripts/plasma-assets.sh` copies the fonts, derives
`contents/look/Look.qml` from `apps/reiwand-app/qml/facade/Look.qml` and copies only the icons that
the panel names. `scripts/package.sh` and the Flatpak recipe run it; the `contents/look` folder is
not stored in the repository.

- The tones of each screen are published by the daemon in `GetState` (`outputs.<screen>.tones`, see
  `dbus.md`); the panel dresses with those of the first screen or, in the view of one screen, with
  those of that one. Without a wallpaper to derive them from it uses the fixed tones.
- With one screen, its cover is the header of the menu. With more than one there is a row per screen
  (thumbnail, wallpaper name, *Next*, *Choose…* and *⋯*), and *⋯* opens the view of that screen:
  *Favorite*, *Fit*, *Filter*, clicks and taps, *Remove the wallpaper of this screen*, *Back to the
  Plasma wallpaper* and *Recover the previous one*.
- *Change with preview…* calls `Playlists1.Browse` and shows on the right the key of
  `shortcuts.next` if `shortcuts.enabled` is on.
- *More* gathers the four switches for sound and flashes and *Pause on battery* (Never, Always or
  below 20 %).
- The row of a screen does not say why a wallpaper is still: the reason is read only in the terminal
  (`reiwand-ctl status`). A scene too heavy for the computer keeps its notice.
- The profile is no longer chosen in the tray: it is done in the app. The thumbnail of an adult
  package is no longer blurred per row.
- The panel cannot be seen without a Plasma session with the plugin; `scripts/plasma-features.sh`
  runs `tests/tray-row.qml`, `tests/tray-left-out.qml` and `tests/tray-consent.qml` (library previews
  never ask for permissions, nor mark the tray as needing attention, nor open it) there.
