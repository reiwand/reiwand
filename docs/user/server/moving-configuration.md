# Moving the configuration to another computer

The library and settings of each computer are local; there is no synchronization. To take them to
another computer, export them to a file and import that file there.

## Exporting

`reiwand-ctl export FILE` (or *Settings > Device > Move your configuration*) writes a file with:

- the added repositories,
- the photo providers,
- the lists, the favorites, the ratings and the tags,
- the installed wallpapers, as references, which are downloaded again,
- the settings that make sense on another computer.

| Option | What it adds |
|--------|--------------|
| `--own` | Your own wallpapers |
| `--secrets` | The keys of the providers and the sessions of your accounts; the file is encrypted with a password |

```sh
reiwand-ctl export config.reiwand                            # the configuration of this computer in a file (see below)
```

## Importing

`reiwand-ctl import FILE` only adds and never replaces. `--list` shows beforehand what it brings.

```sh
reiwand-ctl import config.reiwand                            # … and it is added on another computer
```

With an imported account, the server gives each computer its own session, which is revoked by
itself.

The detail is in [`docs/specs/export.md`](../../specs/export.md).
