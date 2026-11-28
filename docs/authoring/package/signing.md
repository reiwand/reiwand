# Packaging and signing

On this page: making the package, signing it and trying the installation.

## Packaging

```
reiwand-ctl pack build my-scene -o clock.wpak
```

It writes the package and a line with its SHA-256 fingerprint, its size in bytes and the file:

```
<sha256 fingerprint> 1383 clock.wpak
```

Before packaging, `pack build` computes `requires.features` and writes into the manifest the ones
that are missing (and says so on the error output, like `requires.features += scripts (runs scene
scripts)`). Without `-o`, it leaves `<id>-<version>.wpak` in the current folder. `pack build`
refuses if the scene has errors, just like `pack check`.

## Signing

Signing proves that the package was made by whoever has the key and that nobody changed it
afterwards. A repository can require that packages carry the signature of an author it knows.

**Create the key.** A pair of files, one secret and one public, in the current folder:

```
reiwand-ctl repo keygen author
```

It asks for a password (twice) and leaves `author.key` and `author.pub`, and prints the fingerprint
of the public key. At the end it says which file is the secret (`author.key`, never shared) and
which is the public one (`author.pub`, handed to whoever must check your signatures). With
`--no-password` the secret key is left unencrypted; do it only for automation keys. Without a
terminal to ask, the password is taken from the environment variable `REIWAND_KEY_PASSWORD`; with a
terminal it is asked for and the variable, if it exists, takes priority. In the app, *Author keys…*
in the build dialog does the same, says the same when creating the pair and stores the pair in
`$XDG_DATA_HOME/reiwand/keys` (mode 0700).

**Where it is kept.** `author.key` is a file with permissions 0600 (only you read it) and
`author.pub` is public. Keep the secret key in a place of your own and with a backup: if you lose
it, you cannot sign updates with the same identity.

**What never to share.**

- The `.key` file (the secret key) or its password. Not in your code repository, not in the
  package, not in a message, not in a screenshot.
- The files of other keys of a repository (the root key and the online key): they belong to whoever
  runs the repository.

Sharing the `.pub` and the fingerprint is fine: it is what lets others check your signatures.

**Sign and package.**

```
reiwand-ctl pack sign --key author.key my-scene
reiwand-ctl pack build my-scene -o clock.wpak
reiwand-ctl pack validate clock.wpak --key author.pub
```

`pack sign` completes `requires.features` and writes `manifest.json.minisig` next to the manifest.
Check with `pack validate --key`: if the signature is from that key, it prints `signer=` with its
fingerprint.

**Sign after every change.** The signature covers the exact bytes of `manifest.json` and the
fingerprint of all the other files of the folder. If you change the manifest (even the version),
any file (an image, a script, a scene) or the program rewrites something, you have to sign again
before `pack build`. If you forget, `pack build` does not assemble a package with a signature that
does not match:

```
Error: invalid manifest.json.minisig: it signs another version of manifest.json; run pack sign again, or delete it to build unsigned
```

or, if what changed is another file:

```
Error: invalid manifest.json.minisig: it signs other files than this folder holds; run pack sign again, or delete it to build unsigned
```

Run `pack sign` again, or delete `manifest.json.minisig` to build without a signature. The key is
not needed to notice: the signature carries the fingerprint of the manifest it signed. If the
signature is from an earlier version of the program and does not carry it, `pack build my-scene
--key author.pub` checks with your public key that it still holds, and without `--key` it asks to
sign again. So as not to forget, chain `pack sign` and `pack build`.

## Test the installation

With the daemon running (the app starts it):

```
reiwand-ctl lib import clock.wpak --wait
reiwand-ctl scene apply my-scene
reiwand-ctl scene preview my-scene
```

- `lib import` installs the package in your library: this is how whoever installs it will see it.
  It prints `job 1 import done: 0 failed, 1 imported`.
- `scene apply` builds the folder, imports it and shows it on an output (`-o DP-1`) or on all; at
  the end it says what it showed, on which screen and at what quality (`applied: My clock 1.0.0 on
  DP-1, quality Q3`). If you change the folder, apply it again: it shows the new content even if you
  do not raise the version.
- `scene preview` shows it for 60 seconds without saving anything (`--stop` ends it).
- `reiwand-ctl clear` removes the wallpaper in view and leaves the background color, and
  `reiwand-ctl restore` brings back the previous one: useful to leave the screen as it was.
