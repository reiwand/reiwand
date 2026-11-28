# Author keys and package tools

Packages are signed by their authors. This page covers online author keys on a server, the key and
check tools of the editor and the terminal, and *Edit a copy…* for making a derived package.

## Online authors

With `repo init --online-authors`, each account registers in *Account* the keys with which it
signs its packages, without going through the root key.

Each client trusts the key that signed the first version it installed of each package. It only
accepts another if the previous one authorized it. This command signs that handover, which is
stored in *Account*:

```sh
reiwand-ctl pack rotate-key old.key new.pub --author u<id>
```

That way, whoever steals the server's online key can add new authors and packages, but cannot
update an already installed package.

The authors that the root lists take priority.

## Keys in the editor

In the editor, the build dialog has *Keys…*. It:

- creates the author key pair in the application's key folder, with permission only for you and an
  optional password,
- never shows a secret key,
- says which of the two files is the secret and which the public one,
- asks before replacing a pair,
- signs the handover of an online author to another key.

## Checking a package in the editor

The build dialog also has *Check the package…*. It shows:

- each finding with its level, the file and a jump to the field,
- where the package plays,
- whether the signature of the folder or of a `.wpak` matches the author's public key.

The package license is chosen from a list of ten, each with its explanation, and another can be
typed.

## Terminal commands

The same tools exist in the terminal:

- `reiwand-ctl pack check FOLDER`: the checks of *Check the package…*.
- `pack validate FOLDER|FILE --key author.pub`: whether the signature matches the author's public
  key.
- `repo keygen author`: creates an author key pair. The password is asked, or taken from
  `REIWAND_KEY_PASSWORD` without a terminal.
- `pack rotate-key`: signs the handover to another key.

> **Note:** `pack build` does not build a package with a signature of another version of the
> manifest: it asks for `pack sign` again.

## Edit a copy

*Edit a copy…* opens a copy of any kind of package (scenes, pages, images and videos) in a new
folder and with a new id, so the installed one never changes. It keeps the license and the
credits.

It is found:

- in *Details* of each item of the library, at the end of the panel,
- in the *Scenes* list, when the library has packages stored as a `.wpak` file.

The dialog shows the license of the original, what it allows and what it asks (for example,
putting your name among the authors).

If the license does not allow derivative works (for example `CC-BY-ND-4.0`), it does not open it,
unless you check the box that confirms the package is yours or that its author allows it. That box
only appears in that case.

From the terminal:

```sh
reiwand-ctl scene open FOLDER --from package.wpak [--id ID] [--mine] [--facts]
```
