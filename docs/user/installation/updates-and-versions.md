# Updates and versions

How the AppImage updates itself, what happens with data from another version, and how version
numbers are made and published.

## Automatic updates

Reiwand installed from the AppImage updates itself. A systemd user timer (`reiwand-update.timer`)
looks once a day for a newer release on GitHub and installs it.

- It waits while the computer is on battery or on a metered connection.
- It shows a notification when it is done. In Plasma it asks to restart Plasma, which keeps the old
  tray icon and wallpaper plugin until then.
- It never installs an older version.
- It keeps `--no-examples` if the example scenes were left out.
- It needs `curl` and `gpgv` (GnuPG).

Before installing, it checks:

1. the signature of the release's `SHA256SUMS` (`SHA256SUMS.asc`, made with the release key, the
   same one that signs the Flatpak repository; the AppImage carries its public part);
2. the AppImage's checksum in it.

> **Warning:** If either does not match, it installs nothing.

### By hand

| Command | What it does |
|---------|--------------|
| `reiwand --update --check` | Says whether there is a newer version. |
| `reiwand --update` | Installs it. |

### Turning it off

The install says that the daily check is on. To stop it:

- in the app: Settings › Advanced › Version › Details ("Install new versions by itself");
- or `reiwand-ctl update auto off` (`on` turns it on again).

The choice stays across updates.

> **Note:** With private mode on there is no check at all, neither daily nor by hand
> (`reiwand-ctl update allowed` tells why).

An AppImage older than this function does not update itself: install a newer one over it once.

The RPM is updated by its package manager and the Flatpak with `flatpak update`.

## Data from another version

Each version knows only the current format of each piece of data it stores: settings, image edits,
permissions, accounts, library, repository records, exported files, scenes and packages.

A change that breaks a format is a major version, and no program reads or converts the previous
format.

If a program finds data from another version it refuses to start or to open it, says so and
overwrites nothing:

| Who | What it says |
|-----|--------------|
| Daemon: settings, edits, permissions, accounts | «… was made by an older version, which this version does not read; nothing was overwritten» or «… has format N and this version reads only format M; nothing was overwritten» |
| Daemon: library | «the database was made by another version of the program (schema N) and is not read; it was left untouched» |
| Repository records | «the stored repositories in FOLDER were made by another version, which this version does not read; nothing was changed» |
| Any reader of scenes, packages and edits | «"FIELD" is missing. Reiwand does not fill in values: write it.» |
| Package reader (`pack build`, library) | «… is no longer read: this version reads only reiwand.wpak/3, so the package has to be made again in that format and signed again» |
| Exported file (terminal and app) | «the file has format N, from an older version, and this program reads only format M; export it again with this version» |
| Server | see `server.md`, "Data from another version" ([the server](../server.md)) |

> **Note:** Addresses with `?fingerprint=` or `?repo_id=` are not accepted: use the plain address
> of the repository.

## Versions

Versions strictly follow Semantic Versioning 2.0.0. GitVersion computes them in the CI from the
commits of the main branch, written with Conventional Commits:

```
type(scope)!: description
```

The scope and `!` are optional.

| Commit | Effect on the version |
|--------|------------------------|
| `fix` and `perf` | Raise the patch: 1.0.0 → 1.0.1. |
| `feat` | Raises the minor version: 1.0.0 → 1.1.0. |
| `!` after the type or the scope, or a `BREAKING CHANGE:` or `BREAKING-CHANGE:` footer | Raises the major, also from 0.x to 1.0.0. |
| `build`, `chore`, `ci`, `docs`, `refactor`, `revert`, `style` and `test` | Do not change the version. |

### Checks in the CI

The CI checks the subject of every commit since the last published version and the title of each
pull request, also each time it is edited. While any of them does not follow that format, the main
branch or the pull request fails and nothing is published.

A badly written commit that is already on the branch is fixed by adding its hash to `ignore.sha` in
`GitVersion.yml` together with a new commit of the right type.

GitVersion takes the commit with the most recent date as the current one. So each commit needs a
commit date later than that of its whole history, something the CI also checks. If the date is set
by hand, this works:

```sh
GIT_COMMITTER_DATE=@$(( $(git log --format=%ct | sort -n | tail -1) + 1 ))
```

### Publishing

Only commits that raise the version publish a new one. When the last commit of the main branch
builds and its version is greater than all those published, the CI creates `v<version>` on the
*Releases* page of the repository, with:

- the files;
- an SPDX SBOM of the dependencies (`reiwand.spdx.json`);
- a signed `SHA256SUMS`;
- the notes grouped into breaking changes, new features and fixes.

It also publishes the signed image of the server of your own.

> **Note:** The signature and the image are only made when the repository is in the `reiwand`
> organization.

A published version never changes and rerunning an old run publishes nothing.

### Prereleases

The other branches and the pull requests carry a prerelease of the next version whose number grows
with each commit (`0.3.1-branch-name.2`, `0.4.0-feature-name.1`, `0.3.1-PullRequest12.1`). They are
not published.

### Where the version is shown

`--version`, the daemon over D-Bus, the server and the Plasma and KWin packages show the version.
A local build without `REIWAND_VERSION` uses that of `Cargo.toml` with the suffix `-local`.
