# Origin of the starter pictures

`clock.jpg`, `parallax.jpg` and `particles.jpg` are frames of the editor's starting points of the
same names (`reiwand-ctl scene new --from clock|parallax|particles`), rendered at 480×270. They
belong to the project and carry its license (GPL-3.0-or-later). To make them again:

```sh
reiwand-ctl scene new --from clock /tmp/clock
reiwand-ctl scene render --time-of-day 00:00 --time 3 --simulate --size 480 -o clock.jpg /tmp/clock
```

These pictures have a sky gradient that the starting points no longer draw, so a new render is a
flat darker colour behind the same clock, hills or stars.
