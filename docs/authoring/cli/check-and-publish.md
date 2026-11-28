# Check, package and publish

On this page: checks, packages, signatures and repositories.

## Check

### `pack check` of a folder: all the problems

```
reiwand-ctl pack check my-scene
```

```
problem: /manifest/inputs: inputs: the scene uses pointer at /scripts/0 of scene.json; add "pointer"
warning: /manifest: manifest.json.minisig signs an earlier version; sign again or delete it
warning: /scene/assets: unused.png is not used by the scene but goes into the package
Error: 1 problem
```

One line per problem, with the JSON Pointer of the field, and then the warnings with `warning:`. It
exits with 1 if there is any error. `--json` puts them in the `problems` and `warnings` lists of
the report.

**It also checks the scripts, without running them.** It compiles each module in `scripts/` and
reports a syntax error or a file that `scene.json` names and that is not there, with the file and
line:

```
/scene/scripts/0/path: scripts/traffic_light.js:4: unexpected token in expression: ';'
/scene/scripts/1/path: scripts/missing.js is listed in scripts but is not in the folder
Error: 2 problems
```

The line the engine gives is where it noticed the error, which is sometimes the one after the one
with the typo. What only fails when running (a function that throws an error, a value of the wrong
type) is not seen here: see `scripts.md`, “What `validate` sees and what it does not”.

Among the warnings there are two that name the layer: the one that is never seen because another
one on top covers the whole canvas with no transparency (`The layer "picture" never shows: "sky",
above it, covers the whole canvas with no transparency…`) and the shape that ends up entirely
outside the canvas. They do not prevent building the package; the editor's problems panel shows
them just the same.

### `pack check`: what the package needs and where it plays

```
reiwand-ctl pack check my-scene
reiwand-ctl pack check clock.wpak
```

```
ok scene org.example.clock 1.0.0 (reiwand.wpak/3)
title = Clock with script
rating = general
photosensitive = no
origin = original
needs = scripts (runs scene scripts)
facets = 1080p, android, animated, desktop, landscape, localized, scripts
desktop = plays
android = plays
languages = en, de
```

It works with a folder or a `.wpak`. It stops at the first error in the scene or the manifest
(`Error: …`). Script errors (syntax, missing file) do not stop it: they come out as `problem:`
lines below the first line, which then says `problems scene …` instead of `ok scene …`, and the
command ends with `Error: N problems` and exit code 1:

```
problems scene org.example.clock 1.0.0 (reiwand.wpak/3)
problem: scripts/clock.js:4: unexpected token in expression: ';'
```

What each line and each message means: `package.md`, “Checking the package”. `--json` prints the
result in JSON, with the same problems in `problems`.

### `pack validate`: the signature

```
reiwand-ctl pack validate clock.wpak --key author.pub
```

```
ok scene org.example.clock 1.0.0 sha256=<sha256 fingerprint> signer=<key fingerprint>
```

It validates one or more packages (or folders) and, with `--key`, checks that they were signed by
that public key (a file or its text). Without a good signature it prints `signature rejected: …`
and exits with code 1.

## Package and sign

### `pack build`

```
reiwand-ctl pack build my-scene -o clock.wpak
```

```
<sha256 fingerprint> 1383 clock.wpak
```

It builds the package: it computes `requires.features`, writes it in the manifest and compresses
it. It prints the SHA-256 fingerprint, the size in bytes and the file (with `--json`, `{"path",
"sha256", "size"}`). Without `-o`, `<id>-<version>.wpak` in the current folder. It refuses to build
a package if the scene has errors (including syntax errors in its scripts), with the same message
as `pack check`.

If the folder has a signature (`manifest.json.minisig`) made with another version of the manifest
or of any other file, it refuses to build a package that would carry a signature that does not
match (with other files the message says `it signs other files than this folder holds`):

```
Error: invalid manifest.json.minisig: it signs another version of manifest.json; run pack sign again, or delete it to build unsigned
```

Run `pack sign` again (or delete the signature to build without it). `pack build` does not need
the key to notice, because the signature carries the fingerprint of the manifest it signed.
Signatures made with an earlier version of the program do not carry it; for them, `--key author.pub`
(the public key file or its text) says which key to use to check that the signature is still
valid, and without that option `pack build` asks you to sign again. With a key that is not the
signature's, or with a changed manifest, the message is the same. `--key` does not sign anything.

### `repo keygen`: a new key

```
reiwand-ctl repo keygen author
```

```
<public key fingerprint>
```

It creates `author.key` (secret, mode 0600) and `author.pub` in the current folder and prints the
fingerprint of the public one. It asks for a password twice; without a terminal, it reads it from
the `REIWAND_KEY_PASSWORD` environment variable (`pack sign` and `repo sign-root` also use it to
open the key), and if it is not there it warns before creating anything. `--no-password` leaves the
key unencrypted. When it finishes it says which file is the secret and which is the public one.
**Never share the `.key`.**

### `pack sign`

```
reiwand-ctl pack sign --key author.key my-scene
```

```
<key fingerprint>
```

It completes `requires.features` and signs the folder's manifest together with the fingerprint of
all its files (`manifest.json.minisig`). Run it right before `pack build`, and again after any
change.

### `pack rotate-key`

```
reiwand-ctl pack rotate-key --author alex author.key new.pub
```

It prints the signature, made with the old key, of the handover of an online author to a new key;
you give it to `repo rotate-author` or to `server author-key rotate`.

## Repositories

A repository is a folder of signed files that any static server can serve. The complete path, with
the keys and what each step prints, is in `package.md`, “Publishing to a repository”. Here are its
commands. None of them needs the daemon.

| Command | What it does |
|---------|--------------|
| `repo init DIR --name N --root-key root.pub --online-key online.pub [--author ID=author.pub]` | Creates the repository in an empty folder and writes `root/1.json`. It prints its state (`repo_id`, `signers`, `complete`, `address`). |
| `repo sign-root DIR --key root.key` | Signs the newest root with a root key. Until `complete = true`, the repository is not usable. |
| `repo status DIR` | Prints the root's signing state and `address`, the folder's simple address that you paste into *Add a repository* or into `remote add`. |
| `repo add DIR package.wpak… [--author ID] [--unlisted]` | Adds packages to the catalog; you then have to publish. It prints `id version sha256=… author=…`. |
| `repo publish DIR --key online.key [--days N]` | Signs and writes the index. It prints `index N published with M packages; it expires DATE`. |
| `repo refresh DIR --key online.key [--days N]` | Signs the unchanged index again with a new expiry. |
| `repo unlist DIR ID [--version V]`, `repo list-again DIR ID [--version V]` | Hides it from browsing and search (it can still be installed by id), or shows it again. |
| `repo feature DIR ID [--note TEXT]`, `repo unfeature DIR ID` | Features a package, or stops featuring it. |
| `repo revoke DIR ID --reason R [--version V] [--message M]`, `repo unrevoke DIR ID [--version V]` | Revokes a package or a version (`malware`, `license`, `takedown`, `broken`, `other`), or withdraws the revocation. |
| `repo revoke-key DIR FINGERPRINT --reason R` | Revokes an author key. |
| `repo rotate-root DIR [--root-key …] [--online-key …] [--author …]` | Writes the next root with the keys or rules changed; it is signed with `sign-root` and published. |
| `repo register-author SERVER --key author.pub`, `repo rotate-author`, `repo drop-author-key` | Registers, hands over or removes an online author's key on a server. |

`repo add` rejects `imported` packages and, with `--require-author-signature` on the root, those
that do not carry the signature of a known author (`signature rejected: the package must carry an
author signature`). `repo add` with different content for an already published version fails with
“is already published with other content”.

To try it the way another computer would (with the daemon):

```
reiwand-ctl remote add 'file:///path/to/my-repo/' --wait
reiwand-ctl pkg search clock
reiwand-ctl pkg install org.example.clock --wait
```

```
job 1 repo_add done: My repository (1 packages)
job 2 install done: org.example.clock 1.0.0
Installed org.example.clock 1.0.0.
```

The address is the `address` line of `repo status`; the folder also works (`reiwand-ctl remote add
my-repo`). An address with `?fingerprint=` or `?repo_id=` (from version 1.x) is rejected: the root's
fingerprint is learned when the repository is added. Without `--wait`, `remote add` and `pkg
install` only print the job number.

### Your own server

The server, starting it without Docker, the repositories and the accounts are in
[Your own server](../../user/server.md). With an account on the repository:

```
reiwand-ctl server upload clock.wpak --wait
```

It uploads an already signed package to the chosen repository, with the account linked to it.
Register your public key first with `server author-key add`. What the server checks and what it
rejects: `package.md`.
