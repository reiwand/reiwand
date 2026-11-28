# Conditions, presets and translations

On this page: conditions, icons, values by quality, presets, translations and what the user sees.

## Conditions

`visible_if` and `enabled_if` are a map `{ "<other property>": value }` (all of them must be met)
or a short expression, never code, with property ids, numbers, texts in quotes `'...'` or `"..."`,
`true`, `false`, `==`, `!=`, `<`, `<=`, `>`, `>=`, `&&`, `||`, `!` and parentheses, such as
`"style == 'night' || show_gallery"`. An expression has up to 256 characters and 8 levels. It only
compares a property with a value of the same type (`<` and its kin, numbers only). The validator
rejects unknown ids, properties with no value, conditions that are neither true nor false and
conditions that depend on themselves.

```
reiwand-ctl scene act MY-FOLDER condition '{"id":"open","text":"style == '\''night'\'' || show_gallery"}'
reiwand-ctl scene act MY-FOLDER condition '{"id":"folder","text":"show_gallery","enabled":true}'
```

## Icons

An `icon` is one of `appointment-soon`, `audio-volume-high`, `audio-volume-muted`,
`audio-x-generic`, `camera-photo`, `dialog-information`, `document-open`, `edit-find`,
`emblem-favorite`, `folder`, `go-home`, `image-x-generic`, `input-keyboard`, `input-mouse`,
`media-playback-start`, `media-playlist-repeat`, `media-playlist-shuffle`,
`preferences-desktop-theme`, `preferences-system`, `text-x-generic`, `video-display`,
`video-x-generic`, `view-refresh`, `weather-clear`, `weather-clear-night`, `weather-few-clouds`,
`weather-overcast`, `weather-showers`, `weather-snow`, `weather-storm`, `x-office-calendar` or
`zoom-in`: they are icons of the desktop theme. Other names made of `[a-z0-9-]` are accepted
without being shown. Packages do not bring icons of their own.

## Values by quality

`quality` carries values for the levels `q0` to `q3` (`q3` the highest). At that level and the
lower ones, the value is at most that in a slider and exactly that in a choice or a switch. This
way a power-saving mode turns on only when the battery is low. The stored value does not change;
only what the scene, its scripts and the pages see changes. In the example, the opacity never goes
above 50 at `q0` or above 70 at `q1`.

## Presets

A preset is a named combination of values that the user applies at once.

```json
"presets": [
  {
    "id": "warm",
    "label": "Warm",
    "values": {
      "color_disc": "#ff8a3d",
      "style": "day",
      "opacity": 90
    }
  },
  {
    "id": "cool",
    "label": "Cool",
    "values": {
      "color_disc": "#7dd3fc",
      "style": "night",
      "message": "Good night"
    }
  }
]
```

`presets` is a list of `{ "id", "label", "values" }`. A preset cannot set files, folders, secret
texts or buttons. The user can also save the current values as a preset of their own, rename it,
delete it, export it to a small file and import it; those presets belong to the package id and
survive updates. If a property changes type or disappears, the stored value is ignored and a
warning is given. From the terminal:

```
reiwand-ctl scene act MY-FOLDER add_preset '{"id":"warm","label":"Warm"}'
reiwand-ctl scene act MY-FOLDER set '{"pointer":"/manifest/presets/0/values/color_disc","value":"#ff8a3d"}'
```

Values are stored once for all screens, or per screen: a screen without values of its own follows
and changes the shared ones until it is given a copy of its own.

## Translate the labels

Labels, help texts, options and presets carry text in three forms:

- a text: `"label": "Color"`;
- a text per language: `"label": { "en": "Disc color", "de": "Scheibenfarbe" }`;
- a key from the manifest strings: `"label": "@note"`, with the translations in `strings`.

```json
"strings": {
  "en": {
    "note": "Change anything you like: the scene updates at once."
  },
  "de": {
    "note": "Ändere, was du willst: Die Szene aktualisiert sich sofort."
  }
}
```

```json
"note": {
  "label": "@note", "order": 0, "type": "label", "enabled_if": {}, "quality": {}, "visible_if": {}
}
```

With texts per language or with `strings`, the manifest must carry `default_locale` and every text
must have a value in that language. Languages are tags such as `de`, `en` or `pt-BR`; the program
picks the one of whoever uses it, and if it is not there, the one of the main language. A manifest
allows up to 64 languages, with up to 2048 keys each. The translations of a language can also go in
`locale/<language>.json` if the package declares it (see [package.md](../package.md)).

A few labels already come translated by the program: `"@reiwand.accent_color"`,
`"@reiwand.background_color"`, `"@reiwand.brightness"`, `"@reiwand.color"`, `"@reiwand.density"`,
`"@reiwand.opacity"`, `"@reiwand.quality"`, `"@reiwand.size"`, `"@reiwand.speed"` and
`"@reiwand.volume"`. They need no `strings`, and your keys cannot start with `reiwand.`.

```
reiwand-ctl scene act MY-FOLDER translate '{"pointer":"/manifest/properties/opacity/label","language":"en","text":"Disc opacity"}'
```

## What the user sees

The form appears on the *Settings* page of the app, and on the phone on the scene page. Each screen
can have its own values. To change them from the terminal the user has `reiwand-ctl prop` (see
[cli.md](../cli.md)).
