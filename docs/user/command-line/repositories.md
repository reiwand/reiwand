# Repositories

Repositories are signed catalogs of packages. This page explains how they withstand failures, how
provider downloads are bounded, animated previews, and how to use a home-network repository.

## How repositories withstand failures

- Searches read all repositories and the installed packages at once, so they never mix a catalog
  with another list of installed ones.
- Two updates of the same repository from different processes never return it to an earlier state.
- An installation checks again, when the download ends and before moving to the library, that the
  repository is still there and that neither the version nor its key were revoked.
- `file://` mirrors of a signed repository are only used when the repository itself was added from a
  folder or a `file://` URL.

### A damaged repository

If the stored copy of a repository is damaged (its `state.json` is missing, for example),
`reiwand-ctl remote ls` and the app show it as broken without affecting the others.

1. Run `reiwand-ctl remote refresh` to download it again from its address.
2. If the address was lost, repair it by adding it again with its URL, or remove it.

### Downloads

- Cancelling a download stops it within a tenth of a second even if the server sends nothing
  (before, it waited up to 30 s), also in copies from `file://`.
- The previews of earlier versions are downloaded only once and deleted with their repository.

## Provider downloads and secrets

- A provider cuts a download as soon as it does not fit in `cache_mb` (by its `Content-Length` or on
  reaching that size).
- Its cache cleanup resolves symbolic links and never deletes outside its folder.
- Removing it without `--purge` also deletes its sync state, so when added again it syncs right
  away.

> **Note:** If the keyring is locked or does not respond when a provider is added with the name of a
> previous one, the old secret is never used. `reiwand-ctl provider ls` and the app warn of it, and
> it is deleted by itself as soon as the keyring unlocks or returns.

## Animated previews

The animated preview of a package (`previews.animated`) moves in:

- the app's details;
- the thumbnail of each screen of the app and the tray;
- the phone's page of a package that it cannot play.

It moves only while it is seen, on the desktop with the computer plugged in and, if the package is
for adults or photosensitive, after allowing it.

When exploring repositories in the app, the animated preview of the package that has the pointer or
focus also moves:

- It is then downloaded, and only with the computer plugged in.
- It must be within its signed size and hash, and is never downloaded if it exceeds the 8 MiB that
  a package admits.
- It is stored with the other previews and deleted with its repository.
- The one of a photosensitive package is never moved, nor one that is still blurred.

`reiwand-ctl pkg preview --animated PACKAGE` gives the same path.

## A home-network repository

To use a home-network repository without a domain or a public certificate, a computer can serve
`reiwand-server` with its own certificate (`REIWAND_TLS=self-signed`, see
[Your own server](../server.md)). That server is not accepted blindly:

1. When adding its repository, the app shows the SHA-256 fingerprint of its certificate.
2. Compare it with the one `reiwand-server fingerprint` gives.
3. Only if you choose *Trust this certificate* does it pin it.

From then on the phone only talks to that server if it presents that same certificate, and the tab
says so.

From the terminal:

```sh
reiwand-ctl remote add 'https://example.org/r/wallpapers'   # a home server with its certificate: --trust-certificate FINGERPRINT
```
