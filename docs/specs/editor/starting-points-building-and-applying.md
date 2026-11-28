# Starting points, building and applying

`reiwand-ctl scene new DIR --from empty|particles|parallax|clock|web|preset|assets [--id ID] [--title TITLE] [--locale
LANGUAGE]` and the *Scenes* page of the app create a folder from a starting point, with the id
`org.example.<folder name>` (accents dropped, so `Crème` becomes `creme`) and the folder name as
title unless given: an empty canvas, falling particles with a density property, layered hills that
follow the pointer, or a clock with a color property. The manifest is a valid
`reiwand.wpak/3` one: `default_locale` is the language of the title, the first language of the
desktop unless given (`es` for `es_ES.UTF-8`), or `en` when the labels of the properties, in English
and Spanish, lack it; `content` `general`, `photosensitive` `false`, `origin` `original`, license
`CC0-1.0`, and the inputs and features the scene uses. The starting point `assets` makes an empty
assets package (`kind` `assets`, entry `assets.json`, feature `assets`) that is edited like a scene: its
layers, particle systems, scripts and effects are the parts it offers, built and shared as in "Shared
parts"; the *Scenes* page lists it as *Assets package*.
`reiwand-ctl scene open DIR --from FILE.wpak` extracts a scene package into an empty folder, which is
how the app edits a copy of a scene from the library. The copy drops `manifest.json.minisig`,
which signed the manifest of the original author, and gets the id `<id>.copy`, so it never takes
the place of the original in the library. A copy of a version 2 package is derived: its origin
credits the original (its authors, or its id without them, its title, license and home page) after
the works the original came from, and its authors stay for whoever edits it to fill in. Its
license, credits and files stay as they were. Every package kind opens this way, pictures and videos
included; `--id ID` gives the copy another id than `<id>.copy` (`<id>.copy2` for a copy of a copy),
which must be a valid id other than the original. A license that allows no derived works (an SPDX
expression whose every alternative holds `ND`, such as `CC-BY-ND-4.0`, or one that is empty) refuses
the copy, and nothing is written, unless `--mine` says that the person made the package or has the
permission of its author; `--facts` prints, without unpacking, the title, kind, license, authors,
the verdict (`allowed` or `refused`), the conditions that go with an allowed license (attribution,
share-alike, non-commercial, copyleft, custom terms) and the id of the copy. The app offers *Edit a
copy…* on the page of each package of the library (any kind that has a `.wpak` file) and in the list
of the *Scenes* page, which lists scenes, pages, pictures and videos: the dialog shows the same
facts, lets the person pick the folder and the id, and asks to confirm that the package is theirs
before it unpacks a refused license.

`validate` lists the problems of the document and of the folder with the pointer of the field
concerned, nested layers included, the same messages the app shows; unused files and a signature
left by an earlier build are warnings. With `--json` each problem also carries `said`, a message id
such as `scene.color` or `format.invalid` with its arguments; the app and `reiwand-ctl` tell the
messages and refusals in the language of the desktop from `po/<language>/reiwand-messages.po`, and the
outline names the steps of `history.undo` and `history.redo` and the layers of the starting points
(`label`) in that language while the ids stay as stored. A starting point made in a language its
labels lack keeps that language, the one of the title, as `default_locale` and copies the English
labels into it. `build` refuses unsaved changes and errors, signs
`manifest.json` with `--key` exactly as `reiwand-ctl pack sign` does, or removes an earlier signature
without it, and packs the folder with the code of `reiwand-ctl pack build` into `-o FILE` or
`<id>-<version>.wpak` next to the folder. `apply` builds the package into
`$XDG_DATA_HOME/reiwand/scenes/`, imports it into the library and shows it on an output, or
on every output.

*Publish…* in the editor brings the Review panel of the Wallpaper layout (see the panels above);
where no Review panel is at hand it opens the same steps as a page over the editor, «Publish “NAME”»,
with the thumbnail and the name of the package on the left. Three numbered steps and a «Done» part. A
step that is done folds into one line with *Change*; the next one opens by itself and takes the focus,
and Enter runs its main action. The page keeps its state while it is closed, so *Go to the problem*
jumps to the pointer in the editor and *Publish…* comes back to the same step; it starts again only
after a result.

1. *Review* shows what `reiwand_editor::review::report` returns, which `rig` answers for `{"review": {}}`:
   every finding (level `error`, `warning` or `note`, message in the language of the interface, the JSON Pointer to jump to and the
   file that holds it), the platforms that refuse the package or play it without a feature and the
   features a build adds. It holds the version, the license (a list of the common ones with what each
   allows; *All rights* is `LicenseRef-all-rights-reserved`) and two questions the author answers
   once, flashes (`/manifest/photosensitive`) and adult content (`/manifest/content`, rating `mature`
   or `general`); for a copy of a package whose license allows no changes it asks to confirm the copy.
   *Go on* stays off, with the reason, while there are errors, unanswered questions or no license.
   When the package has no thumbnail, or its picture is older than the newest saved file of the folder
   (`reiwand_editor::previews::outdated`, two seconds of margin, signatures left out), one line says so
   with *Update the thumbnail*, the same capture as the package details; the picture on the left
   changes as soon as the capture lands. Without a thumbnail the library draws a cover by itself, but
   a server refuses the upload.
   *More* holds *Check the signature*, which answers `{"signature": {"path", "keys"}}` for a built
   `.wpak` or the folder: `valid` with the fingerprints that signed, `invalid` with the reason,
   `unsigned`, or `unchecked` while no key is given. `reiwand-ctl pack check DIR|FILE` prints the same
   facts and `reiwand-ctl pack validate DIR|FILE --key PUB` checks the signature.
2. *Sign*, or *Your signature* the first time, when the keys folder has no key: the author name and an
   optional password make the pair, and a notice offers *Save a copy…*. With a key it signs with the
   author key and asks its password when it has one, checked before going on; a wrong one says so and
   offers *Manage keys…* and *Publish without signing*. A key without a password skips the step. *More*
   chooses another key or a key file, shows the fingerprint and saves a copy of the pair into a folder;
   until a copy is saved the step says *No copy saved*.
   The signature goes into the package only: a folder without `manifest.json.minisig` before publishing
   has none after it (`reiwand_editor::package::publish`), so the review of the next publish does not warn
   about an earlier signature. `reiwand-ctl pack sign` still signs the folder.
3. *Where?* offers *Save the file* (`<id>-<version>.wpak` next to the folder, or another path),
   *Upload to my server* (public or hidden, through `UploadAs` of `Publish1`, off while no server is
   connected) and *Add to a repository in a folder* (a recent one, *Open another…* or *Create one
   here…*, which makes its key, the repository, signs its root, adds the package and publishes its
   index). The last destination is remembered. Errors keep the page and offer their own way out: a
   version that exists already uploads with the next number, a key the server does not know is
   registered with `AddAuthorKey` (the first time it is registered by itself; when the upload is still
   refused after that, the server only takes the author keys its root lists, created without
   `--online-authors`, and the page says to ask its administrator; the `upload` unit of
   `reiwand-app --check` walks the refusal, the registration and the retry up to the result), no connection retries or
   saves the file, an unsigned root is signed now and an expired index is published again. *More*
   holds the days the index lasts, *Add it hidden* and *Feature it* with a note of up to 140 characters.

«Done» says what to give the other person: the file with *Show in the folder*, the link of the server
with *Copy the link* while it is reviewed, or the address of the repository with *Copy the address* and
the day its index expires; *Set as wallpaper* and *Back to the editor* close it. In the scene editor
*Set as wallpaper* is the one of the top bar: it asks for the screen and offers *Undo*. *Upload package…* in
*Repositories* opens the same page on step 3 for a finished `.wpak`.

*Manage keys…* manages author keys. A pair is two files, `NAME.key` (secret, mode 0600) and
`NAME.pub`, in the keys folder of the app (`$XDG_DATA_HOME/reiwand/keys`, mode 0700); a key may
be encrypted with a password, which is asked only to create it and to sign. The files are protected only by
their permissions, like any key file of the user: wallpapers cannot reach them (web pages read only
their own package, scripts have no files), but any other program that runs as the same user can
copy them, so a key that matters should have a password. The app never shows or
logs a secret key or a password, replaces a pair only after asking and warning that the old secret key
is lost for good, and gives a key by its path, as `--key` does. The same dialog signs the hand-over of
an online author to a new key (`reiwand-ctl pack rotate-key OLD.key NEW.pub --author ID`) and shows the
signature to paste in the account. `reiwand-ctl repo keygen NAME` makes the same pair in the current
folder, with the code of `reiwand_editor::author_keys`.

The repository destination calls the same `reiwand_format::stall` operations as the *Static repository*
tab of *Repositories*, which also keeps the operations of the root: rotating it, revoking a package or
a key and the author keys. The command line has them as `reiwand-ctl repo add [--unlisted]`, `unlist`,
`list-again`, `feature`, `unfeature`, `revoke`, `unrevoke`, `revoke-key`, `init`, `sign-root`,
`rotate-root` and `publish`.
