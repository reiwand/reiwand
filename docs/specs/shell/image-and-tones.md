# Image and tones, Menu, Skeletons, Keyboard, Choosing files and folders, Notices, Size and What a page declares

On this page: image and tones, menu, skeletons, keyboard, choosing files and folders, notices, size, what a page declares.

## Image and tones

- The image is the poster of the wallpaper seen on the window's screen, in the thumbnail that covers
  the width of that screen times its scale (2048 or 4096 px, never larger than the wallpaper itself;
  a packaged scene is drawn at that size as `library.md` describes, and a web page or a packaged
  preset stays on its cover, of at most 1024 px); if that screen has no wallpaper, that of the first
  one that has.
- When the wallpaper changes the new image enters with a fade; if the system asks for less motion
  (animation durations at zero) it changes at once.
- Without a wallpaper in use an example image of a bundled scene is seen, more muted, with the
  caption "Example picture: "Snowfall", one of the bundled scenes".
- With the window hidden or minimized nothing is requested or derived. The tones of each wallpaper
  stay in memory: going back to a wallpaper already seen does not repeat the computation.

## Menu

Library, Explore, Lists, Sources and Create; apart, further down, Settings and Account. The word of
the current place goes in Frost and bold with the accent mark. Layouts (formerly profiles) live
inside Settings, in the Screens tab. The menu is a single block for the keyboard.

## Skeletons

| Skeleton | Pages | Parts |
|---|---|---|
| title | Library, Explore, Lists, Sources, Create | left column with logo and menu under its veil; "now" place at the top right (see "Now block"); band veil at the bottom; the page to the right of the menu or, if it asks for it, in the low band; a place for notices; shortcut line at the bottom right |
| options | Settings, Account | the image as a strip with its veil; the menu; an opaque panel in Night with the page title, its actions, section words and the path with "Esc back"; the page; notices at the top right, under the actions; shortcuts always at the foot |
| work | wallpaper editor, scene editor, packager | top bar in Night with the way back ("‹ Library", "‹ Create"…), the title and the actions; the page takes the rest; notices at the top right of the canvas |

The size steps of `look.md` (compact up to 800 px of height, wide from 1800 × 1000) apply by
themselves.

## Keyboard

- Arrows: move inside a block (the menu, a row, a list).
- Tab: moves to the next block (menu, "now", actions, page).
- Enter: does the main thing of the block (in the menu, opens the chosen word).
- Esc: goes back a level. In title and options it takes focus to the menu, or goes up a step of the
  path if the page has one. In the work skeleton Esc does not leave: only the back word does, so as
  not to leave an editor by accident.
- Focus is seen with the Frost ring.
- The shortcut line only shows arrows, Enter, Esc and Tab.

## Choosing files and folders

Every "Open…", "Add…", "Choose a folder…" and "Save as…" opens the desktop's own file picker,
requested from the portal (`org.freedesktop.portal.FileChooser`). On startup the Qt platform theme
is used for the portal, which wraps the desktop's, unless the user has chosen another theme in
`QT_QPA_PLATFORMTHEME` (that of Plasma, `kde`, does not give its picker to this app and is
replaced). Pages do not open file dialogs of their own: all go through the `Chooser` component (one
file, several, a folder or save, with filters and a starting folder), and a test of the app fails if
a page opens another. When the picker closes, with Esc or on choosing, focus returns to the button
that opened it. Without a portal or platform theme (a bare compositor) Qt draws its own picker.
Dragging files and typing paths by hand stay the same.

## Notices

A single place per skeleton and one notice at a time: first the service error, then the account
notice and last the passing notice (`showPassiveNotification`), which goes away by itself after 7 s
or with its close button. All three are drawn with the same `Toast` piece (see
`settings-pieces.md`): the service error in Coral and with a cross (hidden until it changes), the
passing one with its action ("Undo", for example) and its tone. While the pointer or focus is on the
notice it does not go away. Notices go in the top right corner, the same as the progress of tasks,
aligned to the right edge and stacked one under another: in title skeletons, below the "now" block
(and the page's action bar, if it has one); in options ones, below the title and the actions of the
panel; in work ones, to the right of the canvas, below its bar. If the text and the button do not
fit in one row, the button drops to a second row so that the text uses the full width.

## Size

The first time the window opens at 90 % of the free area of the screen, up to 1600 × 1000. After
that it remembers its size (`~/.config/reiwand/window.conf`). When an editor opens it grows to
1440 × 900 (or 90 % of the screen, whichever is smaller); on screens smaller than 1280 × 800 it
maximizes.

## What a page declares

- `title`: the title the skeleton shows.
- `actions`: the page's actions; the skeleton puts them in its bar.
- `topBar` (optional, only in the work skeleton): a component that takes the top bar after the
  title, instead of the actions; used by a page whose bar has an order of its own (state, groups).
- `keyHints` (optional): list `[{keys: ["left", "right"], text: "…"}]` for the shortcut line; the
  valid keys are `left`, `right`, `up`, `down`, `Enter`, `Esc` and `Tab`. Without it the line shows
  "Tab Next part" and "Esc Menu", except while a pop-up menu has focus: then it shows only "Esc
  Close".
- `route` and `routeBack()` (optional, options pages): the steps of the path and what Esc does.
- `full` (optional, title skeleton): the page takes the whole height; the "now" block and its veils
  are hidden, and an even veil in Night covers the image (the Library in "See all").
- `now` (optional, title skeleton): a `Component` that takes the place of the time and the state, at
  the top right; the page puts its large title and its summary there.
- Notices: `applicationWindow().showPassiveNotification(text, time, action, function)`. Instead of
  the text it may be `{text, tone}` (`tone`: `""`, `"amber"`, `"coral"` or `"sprout"`) to give it its
  tone; the function is what the action does.

The skeleton comes from the place: Settings and Account use the options one; the editors, the work
one; the rest, the title one. A title page can ask for another skeleton while it shows a level of its
own with `skeleton` ("options"): the Library uses it for the options of a wallpaper and Sources for
adding a source and for its details.
