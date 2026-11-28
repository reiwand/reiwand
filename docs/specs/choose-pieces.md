# Pieces for choosing wallpapers

Shared QML pieces of the screens where a person chooses (library, explore, lists, sources). They
live in `apps/reiwand-app/qml/` inside the app's module and only use the tokens of `Look` (see
`look.md`). Their properties are data and signals: never colors or sizes. All are operated with the
keyboard and have a name for accessibility tools.

## Filter

Filter words with their count; the current one carries the accent mark.

- `words`: list of `{ "text": …, "count": … }` (the count is optional).
- `current`: index of the current word. `sections`: larger words, for the tabs of an options screen.
  `label`: name of the group for accessibility.
- Signal `picked(index)`: the screen changes `current`.
- Keyboard: Tab enters the group, left and right arrows move the focus ring, Enter or Space choose.
- Do not use for actions (that is a button in words) or for more than one line of options.

## SearchBox

Search field with the magnifier.

- `text`, `placeholder`; signal `accepted()` with Enter.
- Do not use to type values of a setting: that is the text field of an options row.

## Cover

A cover: 16:9 frame with the image, name and note. The note is the `Note` piece, clipped to the
width of the cover; the focus ring is the common ring (`FocusHalo`).

- `picture` (URL), `name`, `note`, `noteIcon` (icon name), `noteTone` (`""`, `"amber"`, `"coral"` or
  `"sprout"`).
- States: at rest it is dimmed; `lit` enlarges it (except with `even`) and puts the focus ring on
  it, which is seen only while `Keyboard.used` (the last input was a key, not a click or a touch);
  the pointer over it lightens the veil; `off` turns it off (it must carry its reason in `note`);
  `inUse` puts the in-use mark; `choosing` shows the choose mark and `chosen` fills it.
- `even`: fixed size given by whoever contains it (grid); when lit it does not grow and shows the
  name in bold.
- What is put in as a child appears under the lit, enlarged cover (the card).
- Signals `tapped(modifiers)` and `doubleTapped()`.
- The image loads off the main thread.
- `moving` (URL): animated preview that is drawn over the image while it is not empty; it stops
  with the window hidden. Whoever creates it decides when to give it.
- Normally it is not used alone: `CoverShelf` and `CoverGrid` create it.

## Card and FixedCard

`Card`: title, footer and actions of the lit cover, always under it. `FixedCard`: the same on a
fixed line at the bottom of "See all", with the actions on the right and a line above.

- `title`, `caption`; the children are the actions (buttons and the Enter key next to the main one).
- In `FixedCard` without `title` the actions take the line: that is how a `Selection` is put in.
- A single main action per card.

## InUse

The mark of a cover in use or of the running list. Only the cover puts it. In use it is a small
round icon, without text: `screen` gives the accessible name and the hint ("In use" or "On …"),
which appears with `pointed` (the pointer over the mark) only while the mark is seen and carries no
list; it sits next to the icon and goes off when the pointer leaves. With `list` it shows the list
icon and the name of the list.

## ChooseMark and Selection

`ChooseMark` is the round mark of a cover when several are chosen (`chosen`). `Selection` is the line
that takes the place of the card: "N chosen" and its actions.

- If it does not fit on one line, the actions go to two or more lines in columns.
- `Selection.count`; the children are the actions (the main one, the quiet ones, "Stop choosing" in
  words and the Esc key).
- Esc to stop choosing is handled by the screen.

## CoverShelf

The first row of a collection: covers at rest, one lit and enlarged with its card below. It leaves
through both edges of the window: the screen puts it in the page margin and gives it the width up to
the right edge; on the left the covers keep being drawn `Look.margin` beyond their left side, so
when scrolling they go under the window edge and are not cut at the margin. The lit cover does not
go past the page margin.

- `model`: any list model (or an array of objects). The names of the roles are chosen with
  `pictureRole` (`picture`), `nameRole` (`name`), `captionRole` (`caption`), `noteRole`,
  `noteIconRole`, `noteToneRole`, `inUseRole` (`inUse`) and `offRole` (`off`). It does not know the
  library.
- `currentIndex`: the lit cover. `currentName` and `currentCaption` describe it.
- `actions`: component with the actions of the card. `selection`: component that replaces the card
  while choosing and something is chosen.
- `choosing`, `chosen` (list of indexes); `label` for accessibility.
- Signals `activated(index)` (Enter or double click) and `toggled(index)` (Space or click while
  choosing, or Ctrl+click). The screen updates `chosen`.
- Animated view: `movingOf` (a function that receives the index and returns the URL or `""`) and
  the signal `wanted(index)`, which notifies when the pointer enters a cover or the lit cover has
  focus; only that cover receives `movingOf(index)` and stops moving when it loses it. The screen
  asks for the view in `wanted` and decides in `movingOf` whether it moves (battery, flashes).
- Keyboard: left and right arrows move the lit cover and the shelf follows it.
- It only creates the covers that are seen plus one in reserve, and reuses those that leave the
  view.

## CoverGrid

"See all": the whole collection in covers of the same size, as many columns as fit.

- The same properties and signals as `CoverShelf`, except `actions` and `selection`: the card of "See
  all" is a `FixedCard` that the screen puts in, with `currentName` and `currentCaption`.
- Arrows in the four directions.
- The screen gives it height; it only creates the visible rows plus one.

## Limits

- The image of a cover (`Photo`) has rounded corners only when the window draws with the graphics
  card; with software drawing they stay square.
- A disabled cover is dimmed more; it does not turn gray.
- The shadow of covers (`Shadow`) is seen only when the window draws with the graphics card.
- The buttons of the card show their focus ring when reached with Tab.
