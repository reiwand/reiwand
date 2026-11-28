# Pieces for what is adjusted and for what each screen says

Shared QML pieces of the desktop app (`apps/reiwand-app/qml/`). Each screen uses them instead of
drawing its own. They only receive data and notify with signals: no color or size is passed from
the screen; everything comes from `Look` (see `look.md`). All are used with the keyboard alone and
have a name for accessibility tools.

## Rows of options

**`ListRows`**: the column of rows. Its width leaves room for the explanation at the side.

**`ListRow`**: one row per setting, with its value on the right. With focus it is the bar lit in Frost
and its explanation appears at the side.
- Data: `text` (label), `note` (small text below) and `noteTone`, `value` (the value in words),
  `number`, `explain`, `status` and `statusTone` (a status line under the explanation), `off`
  (disabled: what the computer does not have; it still receives focus and shows its reason).
- `kind` chooses the type of value:
  - `text` (words only);
  - `switch` (`checked`, Space or Enter changes it, signal `toggled`);
  - `choice` (‹ value ›: `options` and `current`, left and right arrows, signal `chosen(index)`);
  - `slider` (`from`, `to`, `stepSize`, `at`, arrows, signal `moved(value)`);
  - `field` (`fieldText`, `placeholder`; Enter enters the field, Enter or Esc return to the row,
    signal `edited(text)`);
  - `capture` (`keys`, Enter listens for the combination, Esc cancels, signal `captured(keys)`);
  - `page` (opens a second level, signal `opened`);
  - `expand` (opens or closes the sublist below: `open`, signal `opened`).
- Up and down arrows move from row to row.
- Do not use it for a form with several fields inside a dialog: there the loose controls go.

**`Section`**: the small title that groups rows.

**`Sublist`**: a list under a row (the library folders), with "Remove" on each item and a row to
add.
- Data: `model` (list of `{name, where, problem}`), `addText` and `removeText`.
- Signals: `removed(index)` (also with Delete) and `added`.
- `problem` is written in Coral (deep on the lit item).

## What each screen says

**`StatusLine`** and **`Note`**: a status line in words with its icon, and its small version.
- `text`, `tone` (`amber` warns, `coral` fails, `sprout` went well; empty is neutral), `icon` (one of
  the tone's own if not given).
- `lit`: it is over something lit and uses the deep tones.
- `Note.fitWidth`: maximum width; text that does not fit ends in "…" (the note under a cover).
- `Note.wrap`: with `fitWidth`, the text goes on to the next line instead of being clipped; used by
  notices that must be read whole (the keyring in Sources, no connection).
- Color never goes alone: there are always words.

**`Empty`**: what to do when there is nothing. `title`, `text`, `action` and `actionIcon` (main
button, signal `acted`), `small` (inside a list, usually without an action).

**`Failure`**: the problem in the person's words and the button that fixes it. `title`, `text`,
`action` (signal `acted`), `details` (technical text behind the "Details" fold), `detailsOpen`,
`big` (the whole screen failed).

**`Details`**: the fold for technical text. `text` (default "Details"), `details`, `open`.

**`Toast`**: a passive notice with a single action and the cross to close (`text`, `tone`,
`action`, signals `acted` and `closed`). For what can be undone: `action: qsTr("Undo")`, and the
screen undoes on `acted`. What can be undone is not confirmed.

## Small surfaces

**`SmallPanel`**: the opaque panel of every small surface, placed next to the control that opens it
(below, or above if it does not fit).
- `opener` (the control), `cover` (header photo), `title`, `text`, own content inside.
- Footer: `quiet` (quiet button, signal `rejected`), `primary` and `primaryIcon` (the single main
  action, signal `accepted`), `foot` (more pieces to the left of the footer).
- `destructive`: confirm something that cannot be undone; the main button turns Coral.
- Esc or a click outside close it and focus returns to the control that opened it.
- Grouped question: a `SmallPanel` with several **`Request`** (`icon`, `text`, `reason`).

**`OptionMenu`** with **`OptionItem`**: the menu that a control opens.
- `options` (list of `{text, icon, note, tone, off}` or `{separator: true}`), `current`.
- Arrows, Enter or Space choose; it skips separators and disabled options. Signal `chosen(index)`.

**`Progress`**: progress in words with its rail. `text`, `count` ("2 of 3"), `value` (0 to 1),
`indeterminate`.

## What they do not do

- They do not paint the window or the screens: the screens do that with these pieces.
- Buttons, the loose switch, the text field and the loose rail are controls of the own style; the
  pieces use them and do not draw them again.
- The lit row, the small panel (also the menu of a control) and the notice carry their shadow
  (`Shadow`) only when the window draws with the graphics card; with software drawing they are seen
  without it. The header photo of the small panel (`Photo`) rounds its top corners with the same
  radius as the panel; with software drawing they stay square.
