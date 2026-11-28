# Choosing the graphics card

On a computer with more than one graphics card, the daemon picks which one draws the wallpapers.
This page explains the default choice, the `render.gpu` setting and how cards are told apart.

## The default choice

On a computer with an integrated GPU and a dedicated one, the daemon draws by default with the same
one as the compositor, so frames reach it without being copied.

On startup it asks the compositor for the `linux-dmabuf-v1` "feedback", from which it gets the PCI
bus of its card. On Plasma, KPipeWire only offers the modifiers of that card, and with another
PipeWire falls back to memory and the CPU copies each frame.

If the compositor does not say (X11, without that version of the protocol or a card that cannot be
opened), it chooses the one that uses less energy, which is usually the integrated one.

## The `render.gpu` setting

| Value | Meaning |
|-------|---------|
| `auto` | As always. |
| `integrated` | The integrated card. |
| `dedicated` | The dedicated card. |
| `software` | Software rendering, for testing. |
| part of the name of a card | Case-insensitive, such as `Graphics A`. This serves to choose between two of the same type. |

The dedicated one draws heavy scenes with more quality and uses more energy.

```sh
reiwand-ctl gpu                          # list the cards
reiwand-ctl gpu dedicated                # choose one
reiwand-ctl config render.gpu dedicated  # the same
```

The app offers it in *Settings* → *Graphics card*, with the cards the daemon found (name, type and
API). It marks the one in use and the one that draws the desktop.

If the compositor draws with another card (because a card is forced or by resource), these warn
that frames are copied by the processor:

- the app;
- `reiwand-ctl gpu` (`DESKTOP` column);
- `reiwand-ctl state` (`gpu_note`).

## Applying a change

The change applies from the next start of the daemon, because the renderer is created once and
shared by all screens, videos and scenes.

Until then, the app, `reiwand-ctl gpu` and `reiwand-ctl config render.gpu` warn of it (`gpu_restart`
in `reiwand-ctl state`). The app offers *Restart the daemon now* next to the notice, like
`reiwand-ctl restart`. With systemd, it asks systemd; if not, the daemon runs itself again.

Changing only upper and lower case of the name does not ask for a restart.

> **Note:** If no card matches, or the chosen one cannot be opened, the daemon starts anyway with
> the automatic choice. It says so in the app, in `reiwand-ctl gpu` and in `gpu_warning`.

## How cards are identified

A card that is only reached with OpenGL ES, whose driver does not say what type it is, takes the
type the system gives it in `/sys/class/drm`:

| Card | Type |
|------|------|
| Intel on PCI bus 0 | integrated (without a PCI address, according to its model) |
| NVIDIA | dedicated |
| AMD with 2 GiB of own memory or less | integrated |
| Several cards of one make and of different types | that of the one that starts the screen, or of the other if `DRI_PRIME` asks for another |

So `integrated` also chooses it.

If there are two identical cards, the app tells them apart by their PCI bus. This chooses the one
on that bus (the `BUS` column of `reiwand-ctl gpu`):

```sh
reiwand-ctl gpu pci:0000:03:00.0
```

The list follows the nodes of `/dev/dri`. A card that you connect or remove appears or goes away
within a second, without restarting or polling anything meanwhile.

With `WGPU_BACKEND` set, the daemon only loads the drivers of that API, also when listing the cards.

The class of each card is measured and stored separately, so going back to one does not measure it
again.

> **Note:** The setting does not travel in the exported file or enter profiles. The phone, which has
> a single GPU, does not show it and ignores it.
