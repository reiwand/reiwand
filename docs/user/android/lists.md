# Lists

The *Lists* tab creates lists of wallpapers that change by themselves. The library, the lists and
their rules use the same model and the same rotation logic as the desktop.

## List settings

| Setting | Options |
| --- | --- |
| Interval | every 15 or 30 minutes, every hour, every 6 hours, every day or never |
| Order | in order or at random |
| Videos | if wanted, skip videos on battery |
| Days and hours | change only on certain days of the week and at certain hours (`days` and `at` of its rules) |
| Resume | resume where it left off or start again (`resume`) |
| Home and lock | show the same wallpaper on the home screen and on the lock screen (`synced`) |
| Fade | its own fade (`transition_ms`) |

As on the desktop, these map to the list's rules.

## Edit a list

- Rename it with the pencil on its page.
- Reorder its wallpapers by dragging them by the handle on the right.

## Smart lists

*Smart search* turns a list into a smart list. It uses the same syntax as `reiwand-ctl lib search`,
for example:

```
kind:image tag:sea
```

1. Type the search. The list shows how many wallpapers it finds while you type.
2. Save it with *Save the search*.

*Make manual* leaves the list as it was.

The library also has a search field with that syntax, which adds to the chosen filter.

## When the wallpaper changes

The change is decided when the wallpaper is seen again, without alarms or waking the phone. This
holds also with days and hours.
