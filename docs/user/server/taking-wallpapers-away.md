# Taking a wallpaper away

Authors, moderators and administrators can take a package away from a server, each in a different
way. This page covers withdrawing, hiding and deleting packages, and deleting a whole repository.

## The Manage section

The page of a package has a *Manage* section for whoever may act on it.

| Who | Action | Effect |
|-----|--------|--------|
| **Its author** | *Withdraw this wallpaper for good* | Every version leaves the repository and its files are deleted |
| **Moderators** | *Withdraw from the catalog* | Hides it, with a reason the author sees; *Return to the catalog* undoes it |
| **Moderators** | *Withdraw from the repository* | Revokes every published version, with a statement the author sees |
| **Administrators** | *Delete this wallpaper* | Revokes every version and deletes its files and its votes |

## Withdrawing as the author

*Withdraw this wallpaper for good* asks first. Once confirmed:

- every version leaves the repository,
- its files are deleted,
- the devices that installed it stop showing it at their next refresh.

The package id stays with the author, who can upload a newer version later.

The same is available in two other places:

- In the app: *Account*, *My uploads*, *Withdraw for good…* on a row.
- In the terminal: `reiwand-ctl server withdraw ID` shows what is lost and asks (`--yes` for
  scripts).

## Moderator actions

*Withdraw from the catalog* is the hiding described below, with a reason the author sees, and
*Return to the catalog*.

*Withdraw from the repository* comes with a statement the author sees in *Account*. Every
published version is revoked and its files are kept for a counter-notice, as with the same
decision on a case in *Moderation*.

## Deleting as an administrator

Administrators also see *Delete this wallpaper*. It asks first, then revokes every version,
deletes its files and its votes, and keeps the id with its owner.

## Hiding a package

Moderators can hide a package without withdrawing it, with a reason that its author sees.

- It disappears from the catalog and the search.
- Whoever has it installed keeps it and receives its updates.

It is done from *Moderation* on the web, with the API, or in the terminal with a personal token
with the `mod` permission in `REIWAND_SERVER_TOKEN`:

```sh
reiwand-ctl moderate --server https://wallpapers.example.org hide org.example.wallpaper --repo main --reason "…"
```

| Subcommand | What it does |
|------------|--------------|
| `hide` | Hides the package |
| `unhide` | Shows it again |
| `hidden` | Lists the hidden packages |

## Deleting a repository

*Repositories* has *Delete this repository…* under each repository. It asks first and says what is
lost:

- the address stops answering,
- its packages, votes, members, keys and signed index are deleted.

Devices that added it keep what they had installed and get nothing new from it. A repository
created later with the same name has other keys, so those devices refuse it instead of taking it
for the old one.

Backups made before, and mirrors you set up yourself, keep their own copy.

> **Note:** To take a package away from devices, withdraw it before deleting its repository.
