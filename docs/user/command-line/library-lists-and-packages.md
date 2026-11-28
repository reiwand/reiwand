# Library, lists and packages

The library, the lists, the profiles, the providers and the packages are handled the same way from
the terminal. `reiwand-ctl <group> --help` shows each command. This page gives examples for each
group and explains how the library behaves.

## Library

```sh
reiwand-ctl lib add ~/Pictures --wait              # add a folder and wait for the scan
reiwand-ctl lib search 'tag:sea sort:random'       # search; items are named by their uid
reiwand-ctl lib search 'provider:yes kind:image'   # only the photos that the photo sources brought
reiwand-ctl lib apply 1R1AB2 -o DP-1               # show an item (uid or its last 6+)
reiwand-ctl lib sources                            # the folders; `lib scan` and `lib unsource` accept the short id it shows or the path
reiwand-ctl lib import *.wpak *.jpg --wait          # import several: counts those imported, those already there and those that failed, with the reason for each
```

The app's library asks again only for the items that each change names, with `uid:`.
`reiwand-ctl lib search 'uid:<uid>,<uid>'` also accepts it, with full uids. When changing or
repeating a search, the app keeps the selection and the position of the items that remain in it.

## Lists and profiles

```sh
reiwand-ctl playlist new Sea --query tag:sea       # smart list
reiwand-ctl playlist start Sea && reiwand-ctl next    # rotate a list and advance
reiwand-ctl profile save Desktop                   # save what each screen shows
```

## Providers, repositories and accounts

```sh
reiwand-ctl provider add daily bing                # an online source
reiwand-ctl provider wallhaven add art --ratios 16x9,landscape --colors 660000 --sorting random --no-secret
reiwand-ctl provider plugin install gallery.zip    # a provider plugin: shows its hosts and secret and asks; then provider add ID plugin:ID
reiwand-ctl remote add 'https://example.org/r/wallpapers'   # a home server with its certificate: --trust-certificate FINGERPRINT
reiwand-ctl account sign-in 'https://example.org/r/private'   # accounts per repository: sign-in, ls, use, unlink, rm
reiwand-ctl export config.reiwand && reiwand-ctl import config.reiwand --list   # take the configuration to another computer in a file
```

See [Providers](providers.md), [Repositories](repositories.md) and
[Accounts and the phone's repositories](accounts-and-phone.md).

## Packages

```sh
reiwand-ctl pkg install org.example.dawn --wait    # install a package from a repository
reiwand-ctl pkg import wallpaper.wpak other.wpak --wait  # import local packages; --allow-older accepts an older version
```

## Jobs and live changes

```sh
reiwand-ctl jobs | watch | schema                  # jobs, live changes, valid keys
```

See [Scripts, jobs and output](scripts-jobs-and-output.md).

## Scene input

```sh
reiwand-ctl input decide org.example.dawn -a audio,pointer -x network   # one answer for everything the scene asks for
reiwand-ctl input grant org.example.dawn audio     # let the scene that is seen follow the sound (-o chooses the screen)
reiwand-ctl input status                           # permissions and whether sound or pointer is captured
reiwand-ctl input clicks on -o DP-1                # let the clicks of that screen reach its wallpaper
```

`reiwand-ctl input grant` gives the permission to the package seen with that id.

- If several screens show different packages with the same id, `-o` chooses the screen.
- If the daemon asks in a notification (in the Flatpak, always), it says so and ends fine. The scene
  receives the input when you allow it there.

Clicks and touches need two permissions:

- the package's: `reiwand-ctl input grant <scene> clicks`;
- the screen's, which starts off on all: `reiwand-ctl input clicks on|off [-o screen]` (without
  `-o`, all).

The app offers them in the scene settings of each screen, and the tray on each screen whose scene
asks for them.

## How the library keeps up to date

The library watches its folders, also hard and symbolic links and moved or deleted folders. A
change during the scan of a folder brings another scan when it ends.

The scan reads the files (size, quick fingerprint and manifest of packages) outside the database
and saves the changes in short batches, without writing the files that did not change. So what you
change in the app or the tray does not wait for it to finish reading:

- When checking 1500 loose 1 MiB files that changed, another write waited at most 18 ms instead of
  0.73 s.
- If none changed, the check writes nothing anymore (12 ms instead of 51 ms).

## Tags, history and errors

- A tag that you remove from a package does not come back when the file changes or another version
  arrives (it does if you add it again). When renaming a tag, the old name does not come back
  either.
- In a random list, *Previous* and then *Next* repeat exactly what was shown, in order (the last 32
  wallpapers are remembered), also after changing the list or restarting the daemon.
- A file that the decoder takes too long to read (15 s plus 1 s per MiB, 2 min at most) or that
  makes it crash is not marked as broken. It is tried again when the daemon starts or when it
  changes.
- If the library cannot save something (full disk, locked database), the applied wallpaper stays
  and the position of a list is saved as soon as possible, without advancing again.
- In a list with an interval and fixed hours, a change of time zone recomputes the hours in the new
  zone, and the interval keeps counting from the last change.
- If the keyring does not respond, `reiwand-ctl provider ls` waits 1 s at most and shows the
  provider as locked until the keyring answers. During the next minute, nothing waits for it again.

## Image size limits

An image is decoded with size limits: 32768 pixels per side and 1 GiB of memory. They are the same
in the library, the daemon, the app and `reiwand-ctl`, so a photo that has a thumbnail can also be
set as wallpaper.

- A JPEG photo of up to 300 megapixels is decoded directly to a reduced size.
- A header that promises an enormous size is rejected at once instead of filling memory.
