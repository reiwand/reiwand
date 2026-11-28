# Static repositories

A static repository is a folder of signed files, managed from the terminal or from the *Static
repository* tab of *Repositories* in the desktop app. This page lists what each tool does and
how the desktop app runs those changes.

## Hiding packages before publishing

In a static repository, `reiwand-ctl repo add --unlisted`, `repo unlist` and `repo list-again` do
the same as hidden uploads on a server (see [Uploads and the catalog](uploads-and-catalog.md)),
before `repo publish`.

## The Static repository tab

The *Static repository* tab of *Repositories* matches the terminal commands:

| In the tab | What it does | Terminal |
|------------|--------------|----------|
| Open the repository folder and add packages | Adds packages, hidden too | `repo add --unlisted` for a hidden one |
| Hide or show again | A package or a version, with the same confirmation | `repo unlist`, `repo list-again` |
| Feature or unfeature | A package, with the same confirmation | `repo feature` |
| *Publish…* | Signs the index with an online key file (with its password, if it has one) and the days until it expires | `reiwand-ctl repo publish` |
| Revoke | A package or a version, with its reason and a message | `repo revoke` |
| Withdraw the revocation | Undoes a revocation | `repo unrevoke` |
| *Author keys…* | Revokes an author key by its fingerprint | `repo revoke-key` |
| *New repository…* | Creates one in an empty folder | `repo init` |
| *Root…* | Shows and signs the root | `repo sign-root` |
| *Rotate the root…* | Writes the next root | `repo rotate-root` |

## Creating a repository

*New repository…* creates one in an empty folder with:

- its name,
- the root and online keys (`.pub` files),
- how many root keys must sign,
- an author with its keys,
- the policy.

This is the same as `repo init`.

## The root

*Root…* shows the version of the root, how many keys signed it and the address of the repository.
It signs it with a root key file and its password (`repo sign-root`).

*Rotate the root…* writes the next one with other keys, authors removed or added and another
policy (`repo rotate-root`).

- The keys you give for an author replace those it had, so a leaked key stops being valid
  (`--author ID=PUB`, repeatable for several keys).
- The new root lasts as long as the previous one unless you say other days.
- It keeps the mirrors unless you give others with `--mirror`.
- `--no-mirrors` (or emptying the field in the dialog) removes them all.

## How the app runs changes

- Each change to the catalog works outside the window, one after another.
- While it lasts, the other changes and *Open* wait disabled.
- Each step leaves the buttons disabled until all those in progress finish.

> **Note:** On the phone there are no author tools: only repositories are used (add, explore,
> install, update, remove).
