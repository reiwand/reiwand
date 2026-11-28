# Configuration file

`~/.config/reiwand/config.toml` is written atomically with mode `0600`; the directory is
created with mode `0700`. If the file is a symbolic link, the link is kept and its target is
replaced. The file carries `format = 3` and every setting: at the first start the daemon writes it whole
from the template (`Config::template()`), and a file without `format`, with an older one or that lacks a
key is refused (see below). TOML has no `null`, so a setting that is not chosen is written as a word:
`""` for an image, list, preset or profile, `"general"` for the fit, background, clicks, filter, quality, HDR and
transition of a screen that fixes nothing (in `[default]` it means that the package or the factory value
rules), `"none"` for `blank`, `"auto"` for `surfaces.layer_namespace`. Values out of range are clamped when the
file is read and invalid colors fall back to the template value, with a warning naming the key. Saving keeps the user's file: its comments,
blank lines, key order (also within `[providers.*]`) and value spelling stay, only changed
values are rewritten (keeping their trailing comments), removed keys go with the comments above
them and new tables are added at the end. A file that does not parse, or whose merged form would
not read back as the same configuration, is written fresh.

A file without `format` or with a lower one is not read: the daemon says an older version made it and does not start,
and it overwrites nothing; one with a higher `format` asks to update Reiwand. If the file cannot be parsed at startup, the daemon starts with the template configuration and
reports the error in `GetState().config_error` (`null` when the file is fine), as
`invalid configuration at line L, column C: <message>`. Before the first save, the broken file
is copied to `config.toml.broken` (mode `0600`).

On the phone a settings file that still cannot be read (damaged, or with a key of a newer
version) is left untouched: the settings that depend on it report an error and the network
features stay off, so nothing is reset without the person seeing it.

The first keys of the file, with an excerpt of the sections (the whole file has every key; the written file is the reference):

```toml
format = 3
profile = "01J…"          # active profile uid, "" when none

[default]                 # and each [outputs.<name>]
image = "/path"           # may be a .wpak written by Library1.Apply
fit = "fill"
background = "#000000"
playlist = "01J…"         # playlist assigned to this section
preset = "01J…"           # label of the preset applied to this section

[default.scenes."org.example.calm"]   # and [outputs.<name>.scenes."<package id>"]
tint = "#ff8800"          # values of the settings of a scene or video package

[library]
watch = true
thumb_cache_mb = 512

[repos]
auto_refresh = true
auto_update = false

[models]
allow_download = true

[rgb]
enabled = false           # send colors to RGB lighting
server = "127.0.0.1:6742" # host:port of the OpenRGB server

[providers.photos]        # one table per provider
kind = "immich"
server = "https://photos.example.org"
album = "0d6a…"
refresh = "1h"
cache_mb = 1024

[layout]                  # only written when set
bezel_mm = 8.0            # frame width of every screen side, for the span fit

[layout.monitors."Maker Model 1234"]
mm = [0.0, 0.0, 596.7, 335.6]
```

A section shows its `playlist` if set, else its `image`. An output section that sets either one
overrides the content of `[default]` as a whole; `fit` and `background` still fall back field by
field. `preset` is only a label: `Library1.Apply`, `Profiles1.Activate` and
`Profiles1.SavePreset` write the effective fit and background into the section. When the
library is available at startup or `Reload`, `preset`, `playlist` and `profile` uids that name
nothing are dropped with a warning naming the key, and the file is saved.

Each `[providers.<id>]` table holds the provider's `kind` and the options given to it; every
option is written (the kinds and their options are in `GetSchema().providers`). Secrets are never written here. A table
that cannot be read is skipped with a warning naming the provider and kept unchanged when the
daemon saves the file.
