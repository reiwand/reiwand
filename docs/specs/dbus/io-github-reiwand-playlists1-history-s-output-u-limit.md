# `io.github.reiwand.Playlists1`: `History →` to Signal `BrowseChanged`

On this page: `history →`, `browse →`, `browseapply →`, `browsetransition →`, `browsecancel →`, `getbrowse →`, signal `browsechanged`.

## `History(s output, u limit) → (s json)`

A `HistoryReply`, newest first. `output` is a connector name; `limit` is `1..=1000`.

**Browsing wallpapers.** A browsing session lets the user look at wallpapers one by one on a screen and keep one, without
changing anything until they apply it. It belongs to one output (`""` means the only screen when
there is one; with several screens and an empty `output`, the daemon asks KWin (`org.kde.KWin /KWin activeOutputName`, 150 ms timeout) for the screen with the pointer and the switcher panel opens full screen on that screen and browses its list; without KWin the first open session or the first screen is used). It holds a
candidate, never changes the wallpaper (the switcher panel and the tray read the
`BrowseState` data), and ends by itself after 30 s without input when no switcher panel holds it: while the panel
is open the daemon checks every 30 s that the panel still holds its bus name and keeps the session,
and the panel itself closes (like Esc, changing nothing) after 5 s without a key, wheel turn,
shortcut or pointer movement over it, never while a button is held on it. Nothing runs while nobody browses:
one timer per open session.

`source` is `playlist` (the playlist the screen plays: the candidate is exactly the item `Next`
or `Previous` would show, shuffle and history included, and `position` is its place in the list),
`library` (every item of the library that can be shown, hidden mature items excepted, in library
order; `position` and `count` are its place and the number of items) or `""`: the playlist when the
screen plays one, else the library. Every method answers with a `BrowseState` and sends
`BrowseChanged` with the same JSON. Fields: `output`, `active`, `ended` (`applied`, `cancelled`,
`expired`; only in the reply or signal that ends a session), `source`, `playlist`,
`playlist_name`, `steps` (how far the candidate is from what the screen shows), `position`,
`count`, `shown` (uid shown when browsing began), `candidate` (an `ItemSummary`: title, kind,
thumbnail and package), `now` (the `ItemSummary` the screen shows), `behind` and `ahead` (up to two
`ItemSummary` that follow the candidate, without repeats), `plays` (how the candidate would come
in: `kind` `fade`, `none` or `own`; `why` `chosen` for the transition picked with
`BrowseTransition`, `screen` for the screen's own setting, `general` for the general one, `calm`
when the system asks for less motion and `still` when the user does; `duration_ms` of the fade, 0
otherwise), `path`, `transition`, `idle_ms` (time left before it ends by itself) and `shortcuts`
(`off`, `asking`, `bound`, `refused`, `unavailable`).

**The panel.** Whenever a session begins, whoever began it (a shortcut, the tray or
`reiwand-ctl next --preview`), the daemon starts `reiwand-app --browse OUTPUT` (the program named by
`REIWAND_PREVIEW_APP` when set), which claims the bus name `<app id>.Switcher` so that only one panel exists
per output. It is a window of its own with keyboard focus: a centred panel over a dimmed screen with
the candidate large in the middle (a scene or video plays there, rendered by the daemon into a muted
`preview:card-*` stream requested through `RequestStream` and `Apply`, as the library cards do, and released when the
candidate changes or the panel closes; other kinds use an animated GIF or WebP preview when the package has one),
sharp still neighbours
at both sides, its name and position below and a line of keys. It reads and drives the session through
`GetBrowse`, `Browse`, `BrowseApply` and `BrowseCancel`: arrows, the wheel and clicks on a neighbour
move, Enter or a click on the middle one applies, Esc or a click outside cancels. The daemon draws
nothing into the wallpaper frames. After `BrowseApply` or `BrowseCancel` the window hides and the process stays resident for 5 minutes without
working, hearing the `browse` signal; the next session shows the same process at once (the daemon sees the
bus name taken and does not start another). After 5 minutes hidden it exits. Exit code 3 of the panel means "open the library" (shown when the
screen has a single wallpaper). The real wallpaper does not change until `BrowseApply`.

## `Browse(s output, s source, i steps) → (s json)`

Starts a session on `output` (from `source`) or moves the candidate of the open one by `steps`
(negative goes back, `0` only starts or refreshes it). A `source` other than the open session's
starts a new session. `NotFound` when `source` is `playlist` and the screen plays none; `Failed`
when the library is empty.

## `BrowseApply(s output, s transition) → (s json)`

Shows the candidate on `output` and ends the session. A playlist candidate moves the playlist to
it, as `Next` would, so the rotation goes on from there; a library candidate replaces the
playlist of that screen, as `Apply` does. `transition` is `fade`, `none` or `""` (the one chosen
with `BrowseTransition`, else the usual one); it is used only for this change. `Failed`, keeping
the session, when the playlist moved on by itself while browsing: the reply's new candidate
must be looked at first. `NotFound` without a session.

## `BrowseTransition(s output, s transition) → (s json)`

Chooses the transition `BrowseApply` uses: `fade`, `none` or `""`. `InvalidArgs` for another
word.

## `BrowseCancel(s output) → (s json)`

Ends the session and leaves everything as it was. Without a session it changes nothing and
answers `active: false` with no `ended`.

## `GetBrowse(s output) → (s json)`

The `BrowseState` of `output`, with `active: false` when nobody browses there.

## Signal `BrowseChanged(s output, s json)`

Sent on every change of a session, with the `BrowseState`, also when it ends by itself.

**Global shortcuts.** With `shortcuts.enabled` the daemon asks the desktop portal for global shortcuts to bind four
keys: `browse-next`, `browse-previous`, `browse-apply` and `browse-cancel`, proposing `shortcuts.next`,
`shortcuts.previous`, `shortcuts.apply` and `shortcuts.cancel`
(`LOGO+ALT+N`, `LOGO+ALT+P`, `LOGO+ALT+Return` and `LOGO+ALT+Escape`; the desktop may show its own dialog and let the user pick other
keys). A key press starts or moves a session on the only screen (on every screen with several)
and the panel opens as said above; the other two keys apply or cancel. Where the desktop
has no such portal (wlroots compositors, X11) the state is `unavailable` and the user binds
`reiwand-ctl next --preview`, `prev --preview`, `browse apply` and `browse cancel` to keys in the compositor.
The daemon opens its own session-bus connection for this, so that the first portal call of that
connection is `org.freedesktop.host.portal.Registry.Register` with the app id `io.github.reiwand` (the id of the
installed desktop entry); a failed registration is logged at WARN. Outside Flatpak the portal refuses a
connection that already talked to it, which is why the shared connection is not used. Settings reads the state
with `GetBrowse` and, when it is `refused` or `unavailable`, says next to "Keyboard shortcuts" that the keys
must be bound by hand to the commands above.
