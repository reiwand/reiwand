# Download and install the AppImage

How to check a release, install the AppImage in your home directory without root, run its programs
without installing, and uninstall.

## Check the download

Each release on the *Releases* page always has the same file names and a `SHA256SUMS` to check
them.

`SHA256SUMS` is signed with [cosign](https://github.com/sigstore/cosign) without a key of its own:

- The signature (`SHA256SUMS.sigstore.json`) carries a short-lived certificate that Sigstore issues
  for the CI of this repository on the main branch.
- It is recorded in Sigstore's public transparency log.

> **Note:** Releases published before the move to the `reiwand` organization are not signed.

With cosign 3 or later, run this next to the downloaded files:

```sh
cosign verify-blob SHA256SUMS --bundle SHA256SUMS.sigstore.json \
  --certificate-identity https://github.com/reiwand/reiwand/.github/workflows/ci.yml@refs/heads/master \
  --certificate-oidc-issuer https://token.actions.githubusercontent.com
sha256sum -c --ignore-missing SHA256SUMS
```

### With GnuPG

`SHA256SUMS.asc` signs the same file with the Reiwand release key:

| | |
|---|---|
| Fingerprint | `9C23 C7EF 7478 9850 1E4D 8EBF 0C9F 3D23 2716 55A8` |
| Public part | `packaging/flatpak/repo-key.asc` of the source code |

After importing the key, run `gpg --verify SHA256SUMS.asc SHA256SUMS`.

## Install

1. Download `reiwand-x86_64.AppImage` from the latest release.
2. Run it:

   ```sh
   chmod +x reiwand-x86_64.AppImage
   ./reiwand-x86_64.AppImage --install
   ```

Opening the file from the file manager does the same and then opens the app.

The downloaded file is no longer needed afterwards. It also works on distributions with an
immutable file system.

### What it installs

It installs everything in your home directory, without root:

- the version in `~/.local/lib/reiwand/<version>`, with `~/.local/lib/reiwand/current` pointing
  to it;
- the commands `reiwand`, `reiwandd`, `reiwand-ctl`, `reiwand-app` and `reiwand-web` in
  `~/.local/bin`;
- a systemd user service;
- D-Bus activation;
- the desktop app, with its launcher and icon;
- the wallpaper plugin and the Plasma tray icon.

The app asks the desktop for its file picker through the portal, with the Qt portal theme it
carries.

### Upgrading

`--install` with a newer AppImage upgrades Reiwand and restarts the service, and so does opening
it. Opening an older one only opens the installed app.

For automatic updates, see [Updates and versions](updates-and-versions.md).

### The session bus

The install asks the session bus to re-read its services, so that the tray opens the app without
logging in again. This also works with dbus-broker, which does not watch for new services.

If the bus still does not know the app, the tray asks it to re-read its services and tries again.
If it still does not open, it says so.

## Uninstall

Run `reiwand --uninstall`. Add `--purge` to also delete the configuration.

## Run without installing

The AppImage also runs its programs without installing anything:

| Command | What it does |
|---------|--------------|
| `./reiwand-x86_64.AppImage reiwand-ctl --help` | Runs `reiwand-ctl` |
| `./reiwand-x86_64.AppImage --version` | Shows the version |
| `./reiwand-x86_64.AppImage --help` | Lists the rest |

That needs FUSE (`fusermount3` or `fusermount`, present on most desktops). Without it, put
`--appimage-extract-and-run` first, as in
`./reiwand-x86_64.AppImage --appimage-extract-and-run --install`.

> **Note:** The service trusts only the `reiwand-ctl` and app installed next to it. The few actions
> reserved to them (such as `reiwand-ctl input pointer`) are refused to the copies run from the
> file.

The AppImage carries the programs of Reiwand with the Qt that the app and the web helper use, not
GStreamer or PipeWire: those come from the system (see the [requirements](requirements.md)).
