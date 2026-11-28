# Stored data and secrets

Where the program keeps keys, passwords, the configuration, the library and its caches, and who can
read them.

## Keys and passwords

API keys and passwords are stored in the system keyring (Secret Service) and nowhere else:

- not in the configuration;
- not in the logs;
- not on the command line.

They do not travel to another computer except in a file exported with `--secrets`, encrypted with a
password. Providers and their keys belong to each computer.

### Without a keyring

A computer without a keyring can store all the secrets (providers, scene scripts and sessions of
your accounts) in a private file (0600) that you choose knowingly.

| To | Do |
|----|----|
| Store them in a file | `reiwand-ctl provider store file`, or the notice on the providers page |
| Go back to the keyring | `reiwand-ctl provider store keyring` |

A visible notice stays while it is that way. You can go back to the keyring without removing
anything.

## Files and permissions

The configuration is written with `0600` permissions inside a `0700` folder: only your user can
read it.

The download cache is kept in a private folder.

| What | Where | Permissions |
|------|-------|-------------|
| The library and imported copies | `~/.local/share/reiwand` | `0600` files in `0700` folders |
| The thumbnails and the package entries | `~/.cache/reiwand` | `0600` files in `0700` folders |
| The view of the image editor | the runtime folder of your session (`XDG_RUNTIME_DIR`, or the temporary folder if there is none) | a `0600` file |

The view of the image editor is deleted when the next one is drawn or when the editor is closed.

Thumbnails and the analysis of the library (size, duration, color) are done by a separate process
with the lowest CPU and disk priority.

## Packages

Packages are generated without paths, users or the name of the computer of whoever builds them.
