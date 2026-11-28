# Network and updates

What the program connects to, and when. There is no telemetry, and private mode cuts the whole
network at once.

## Private mode

Turn it on in Settings › Device, or with `reiwand-ctl private on`. It cuts the program's whole
network at once.

While it is on:

- sources and repositories are not updated;
- nothing is downloaded;
- no covers are requested from the internet;
- no server is contacted;
- the OpenRGB lights are turned off (they talk over TCP even if they are on this computer);
- web wallpapers only load files from this computer;
- no name is looked up on the network either;
- every check for a new version stops.

What you already have keeps working: library, included examples, scenes, videos and local web
wallpapers.

It is off by default and is remembered across restarts.

## Checks for a new version

Only the AppImage and the phone app make one. The RPM and the Flatpak are updated by their package
manager.

**The AppImage** asks GitHub (`github.com`) once a day for its latest release. When it is newer and
its `SHA256SUMS` carries the signature of the Reiwand key, it downloads and installs it.

- The request carries the generic User-Agent of `curl` and nothing of yours.
- It is on from the install, which says so.
- Turn it off in Settings › Advanced › Version › Details ("Install new versions by itself") or with
  `reiwand-ctl update auto off`.

**The phone app** does the same from its About page, with its own switch.

> **Note:** Private mode stops both, with no request at all.

## What the service connects to

No telemetry and no usage statistics.

The service only connects to the internet for the wallpaper providers and the repositories that you
add. Each repository belongs to a server, and its catalog, your downloads and, if you have an
account, your session go to that server.

- On startup it only contacts a provider if its automatic update is due.
- The daily check of the repositories (`repos.auto_refresh`, also in the phone settings) starts 10
  minutes later.
- A repository can only redirect to its own address and to the mirrors it declares in its signed
  root.

### The network gate

The service only opens network connections for the providers, repositories and servers that you
add: its own network gate refuses the rest.

> **Note:** The systemd unit only limits it to local, netlink and IP sockets, not to addresses.

Its logs do not include paths, file names, servers of your own or the GPU model except with
`RUST_LOG=debug`.

## Online wallpaper providers

Online wallpaper providers (Bing, APOD, Wikimedia, Wallhaven, WebDAV/Nextcloud, Immich) are only
used if you configure them.

| Rule | Details |
|------|---------|
| Encryption | Public services are always used over HTTPS. |
| User-Agent | All carry a generic User-Agent (`reiwand/<version>` and the project address, nothing of yours). |
| Cookies and Referer | No cookies and no Referer header. |
| API keys and passwords | Only sent to the configured server. |

How keys and passwords are stored is in [Stored data and secrets](stored-data.md).

> **Warning:** The address of your own server may use unencrypted `http://`, always allowed, with a
> permanent notice that the password travels unencrypted.

## Web wallpapers

Web wallpapers have no network unless they declare it and you allow it.

What they see of you (time zone, language, graphics card and their stored data) is announced in
the app, the tray, `reiwand-ctl` and the phone, and you can hide it per wallpaper.

On the phone the WebView runs with its metrics and safe browsing off.

## RGB lighting

RGB lighting is off by default. When on, it only sends colors (never anything else):

- to the OpenRGB server of this computer, or to the address you choose with a visible notice;
- only from the packages you allow it for.

## Depth and fill models

The depth model, the LaMa fill model and ONNX Runtime are only downloaded if you ask and accept
their size, license and servers.

- They carry the same generic User-Agent, no Referer header and no HTTPS-to-HTTP redirects.
- They are verified with SHA-256.
- The maps are computed on the computer, in a separate process.
- ONNX Runtime runs with its telemetry disabled: it stores no identifier and connects to no one.

On Android, ONNX Runtime is downloaded together with the model and only after you accept it,
checked with its SHA-256, and used with telemetry off.

`models.allow_download` stays on each computer and, if the configuration cannot be read, counts as
disabled.

## Servers

A server only knows what you ask of it: your account and what you upload or vote on in its
repositories.

It does not store your library, lists, favorites or settings: there is no synchronization. To take
your settings to another computer you export a file (`reiwand-ctl export`) and import it there.

The server of your own does not store IPs or user agents in its logs unless its administrator turns
on abuse logging, which deletes itself.
