# D-Bus interface

Every component takes these names from `reiwand_core::dbus`. The command line client (`reiwand-ctl`)
and the desktop app are plain clients of this interface: every action is available in both. The
system tray and the desktop integration offer a subset.

| Item | Value |
|------|-------|
| Bus | session |
| Well-known name | `io.github.reiwand.Daemon` |
| Object path | `/io/github/reiwand/Daemon` |
| Interfaces | `io.github.reiwand.Daemon1`, `Library1`, `Playlists1`, `Profiles1`, `Providers1`, `Repos1`, `Inputs1`, `Publish1`, `Accounts1`, `Carry1`, all on that object |

The daemon is D-Bus activatable (`packaging/dbus`) and runs as the systemd user unit
`reiwandd.service`. In the Flatpak (`packaging/flatpak`) it owns the name through `--own-name`, so
Flatpak exports the same activation file, starts the daemon with `flatpak run` and drops
`SystemdService`; there is no systemd unit there. Inside the Flatpak the daemon asks the Background
portal (`org.freedesktop.portal.Background.RequestBackground`, reason "Draws the desktop wallpaper",
no autostart) to run with no window, and sets its status line with `SetStatus` ("Renders your
wallpapers", translated, at most 96 characters; an older portal without it is only logged): the
desktop shows that line for the service in its list of background apps. Outside the Flatpak it does
neither. The name is owned without allowing
replacement. A second daemon checks the name before initializing the GPU and PipeWire and exits
at once with an error saying that another reiwandd owns it.

Every connection of the workspace to the session or the system bus (daemon, `reiwand-ctl`, the app)
goes through `reiwand_client::bus`, which sends each authentication command only after the bus
answered the one before. Inside the Flatpak both buses are reached through `xdg-dbus-proxy`, and
its releases before 0.1.6 drop a client that sends its authentication commands in one write; with
this handshake the daemon and the clients connect through any proxy version.
Those releases also drop a client whose message serials do not increase strictly, and the bus
library numbers a message when it builds it, not when it writes it; so after the handshake the
adapter gives every outgoing message the next serial of its connection, in the order the messages
reach the socket, and gives each reply back the serial of the call it answers.
`scripts/integration-proxy.sh` checks both when `REIWAND_BUS_PROXY` names a proxy binary, restarting the daemon fifteen times behind it.

A panic in any thread of the daemon, a lost GPU device, a GPU that stops responding or a Wayland
protocol error from the compositor makes the daemon exit with status 70 at once, without running the
exit handlers of the libraries it uses (a GPU driver that stopped answering cannot hang it there),
and the end of the display server connection with status 0 the same way; the user unit restarts
it (`Restart=on-failure`, at most 5 starts in 60 s) and clients get their streams again through
`RequestStream`. Only the methods that change what is shown (`Library1.Apply`,
`Playlists1.Start`, `Stop`, `Next` and `Previous`, `Profiles1.Activate` and the `Daemon1` methods
that change something) need the renderer: while it is not running they return `Failed` (`the
renderer stopped after an error; restart reiwandd`) and change nothing. Every other method keeps
working. Debug builds started with `REIWAND_RENDER_STOP=1` stop their renderer on `SIGUSR2`, so tests
can check this.

The daemon opens network connections only to the providers, repositories and the servers
the user added (the accounts they signed in with) (including the signed mirrors of a repository, the only hosts a repository request
may be redirected to), with a generic `User-Agent:
reiwand/<version> (+https://github.com/reiwand/reiwand)` (the program and the address of the
project, nothing about the person or the computer) and without cookies or `Referer`. At startup it contacts only providers
due for their automatic sync; the daily
repository refresh starts 10 minutes later.

The specification, part by part:

- [Conventions, Errors and Streams and claims](dbus/conventions.md): conventions, errors, streams and claims.
- [Layer-shell and X11 outputs](dbus/layer-shell-and-x11-outputs.md): layer-shell and x11 outputs.
- [Frame rate and Video decoding](dbus/frame-rate.md): frame rate, video decoding.
- [Stream buffers](dbus/stream-buffers.md): stream buffers.
- [`io.github.reiwand.Daemon1`](dbus/io-github-reiwand-daemon1.md): `requeststream →`, `holdstream →`, `releasestream`, `setoutputcovered`, `setoutputinfo`, `previewedit`, `showcalibration`, `previewscene`, `setwallpaper`.
- [`io.github.reiwand.Daemon1`: `ClearWallpaper` to `SetSceneProperty`](dbus/io-github-reiwand-daemon1-clearwallpaper-s-output.md): `clearwallpaper`, `restorewallpaper`, `setlockwallpaper`, `setlockplaylist`, `followdesktoponlock`, `showdesktoponlock`, `getlock →`, `handbacktoplasma →`, `setbackground`, `setfilter`, `getsceneproperties →`, `setsceneproperty`.
- [`io.github.reiwand.Daemon1`: `ListApplications →` to `ShareSceneValues`](dbus/io-github-reiwand-daemon1-listapplications-s-json.md): `listapplications →`, `checkshortcut`, `applyscenepreset`, `resetsceneproperties`, `savescenepreset →`, `renamescenepreset`, `deletescenepreset`, `exportscenevalues →`, `importscenevalues →`, `pressscenebutton`, `capturepage →`, `getparticlestats →`, `sharescenevalues`.
- [`io.github.reiwand.Daemon1`: `SetOption`](dbus/io-github-reiwand-daemon1-setoption-s-key-s-value.md): `setoption`.
- [`io.github.reiwand.Daemon1`: `Reload` to Signal `JobFinished`](dbus/io-github-reiwand-daemon1-reload.md): `reload`, `restart`, `retryeffects`, `retrylayers`, `getstate →`, `getschema →`, `setpaused`, `setvolume`, `setmuted`, `setprivatemode`, `waitchanged →`, `listjobs →`, `getjob →`, `canceljob`, property `paused`, property `volume`, property `muted`, property `privatemode`, property `serial`, property `model`, signal `streamchanged`, signal `changed`, signal `jobprogress`, signal `jobfinished`.
- [`io.github.reiwand.Library1`](dbus/io-github-reiwand-library1.md): `addsource →`, `removesource`, `listsources →`, `rescan →`, `import →`, `importpackages →`, `resolve →`, `remove`, `search →`, `getitem →`, `settitle`, `setrating`, `setsound`, `setfavorite`, `settags`, `listtags →`.
- [`io.github.reiwand.Library1`: `RenameTag` to Signal `ThumbnailReady`](dbus/io-github-reiwand-library1-renametag-s-from-s-to.md): `renametag`, `thumbnail →`, `requestthumbnails →`, `apply`, `confirm`, `unconfirm`, `cleardata`, signal `thumbnailready`.
- [`io.github.reiwand.Playlists1`](dbus/io-github-reiwand-playlists1.md): `list →`, `get →`, `create →`, `rename`, `setquery`, `delete`, `additems`, `addfolder →`, `rotate →`, `removeitems`, `removeentries`, `moveitem`, `setrule`, `start`, `stop`, `next`, `previous`.
- [`io.github.reiwand.Playlists1`: `History →` to Signal `BrowseChanged`](dbus/io-github-reiwand-playlists1-history-s-output-u-limit.md): `history →`, `browse →`, `browseapply →`, `browsetransition →`, `browsecancel →`, `getbrowse →`, signal `browsechanged`.
- [`io.github.reiwand.Profiles1`](dbus/io-github-reiwand-profiles1.md): `io.github.reiwand.profiles1`.
- [`io.github.reiwand.Providers1`](dbus/io-github-reiwand-providers1.md): `list →`, `add →`, `set`, `update`, `remove`.
- [`io.github.reiwand.Providers1`: `ClearCache` to `RemovePlugin`](dbus/io-github-reiwand-providers1-clearcache-s-id.md): `clearcache`, `sync →`, `collections →`, `reseed →`, `setsecret`, `forgetsecret`, `setsecretstore`, `inspectplugin →`, `installplugin →`, `listplugins →`, `removeplugin`.
- [`io.github.reiwand.Repos1`](dbus/io-github-reiwand-repos1.md): `io.github.reiwand.repos1`.
- [`io.github.reiwand.Inputs1`](dbus/io-github-reiwand-inputs1.md): `grant`, `grantpackage`.
- [`io.github.reiwand.Inputs1`: `Decide` to `Bands →`](dbus/io-github-reiwand-inputs1-decide-s-scene-s-key-as-.md): `decide`, `revoke`, `getinputs →`, `setclicks`, `simulatemedia`, `simulatetouch`, `simulatetrigger`, `bands →`.
- [`io.github.reiwand.Inputs1`: `Touch` to `SetPointer`](dbus/io-github-reiwand-inputs1-touch-s-output-d-x-d-y-u.md): `touch`, `setpointer`.
- [`io.github.reiwand.Rgb1`](dbus/io-github-reiwand-rgb1.md): `io.github.reiwand.rgb1`.
- [`io.github.reiwand.Publish1`, `io.github.reiwand.Carry1` and `io.github.reiwand.Accounts1`](dbus/io-github-reiwand-publish1.md): `io.github.reiwand.publish1`, `io.github.reiwand.carry1`, `io.github.reiwand.accounts1`.
- [The app: `io.github.reiwand`](dbus/the-app-io-github-reiwand.md): the app: `io.github.reiwand`.
- [Edits and spanning](dbus/edits-and-spanning.md): edits and spanning.
- [Scenes, Scene sound and Change notification](dbus/scenes.md): scenes, scene sound, change notification.
- [Jobs and Thumbnails](dbus/jobs.md): jobs, thumbnails.
- [JSON model](dbus/json-model.md): json model.
- [Configuration file](dbus/configuration-file.md): configuration file.

## Where the sections went

The sections of this page moved to the pages above; links to them still land here:

- <a id="conventions"></a>[Conventions](dbus/conventions.md#conventions)
- <a id="errors"></a>[Errors](dbus/conventions.md#errors)
- <a id="streams-and-claims"></a>[Streams and claims](dbus/conventions.md#streams-and-claims)
- <a id="layer-shell-and-x11-outputs"></a>[Layer-shell and X11 outputs](dbus/layer-shell-and-x11-outputs.md#layer-shell-and-x11-outputs)
- <a id="frame-rate"></a>[Frame rate](dbus/frame-rate.md#frame-rate)
- <a id="graphics-card"></a>[Graphics card](dbus/frame-rate.md#graphics-card)
- <a id="video-decoding"></a>[Video decoding](dbus/frame-rate.md#video-decoding)
- <a id="stream-buffers"></a>[Stream buffers](dbus/stream-buffers.md#stream-buffers)
- <a id="iogithubreiwanddaemon1"></a>[`io.github.reiwand.Daemon1`](dbus/io-github-reiwand-daemon1.md#iogithubreiwanddaemon1)
- <a id="requeststreams-output-u-width-u-height--u-node_id-t-serial"></a>[`RequestStream(s output, u width, u height) → (u node_id, t serial)`](dbus/io-github-reiwand-daemon1.md#requeststreams-output-u-width-u-height--u-node_id-t-serial)
- <a id="holdstreams-output-u-width-u-height--u-node_id-t-serial"></a>[`HoldStream(s output, u width, u height) → (u node_id, t serial)`](dbus/io-github-reiwand-daemon1.md#holdstreams-output-u-width-u-height--u-node_id-t-serial)
- <a id="releasestreams-output"></a>[`ReleaseStream(s output)`](dbus/io-github-reiwand-daemon1.md#releasestreams-output)
- <a id="setoutputcovereds-output-b-covered"></a>[`SetOutputCovered(s output, b covered)`](dbus/io-github-reiwand-daemon1.md#setoutputcovereds-output-b-covered)
- <a id="setoutputinfos-output-i-x-i-y-u-width-u-height-d-scale-d-px_per_mm-s-identity"></a>[`SetOutputInfo(s output, i x, i y, u width, u height, d scale, d px_per_mm, s identity)`](dbus/io-github-reiwand-daemon1.md#setoutputinfos-output-i-x-i-y-u-width-u-height-d-scale-d-px_per_mm-s-identity)
- <a id="previewedits-path-s-edit_json"></a>[`PreviewEdit(s path, s edit_json)`](dbus/io-github-reiwand-daemon1.md#previewedits-path-s-edit_json)
- <a id="showcalibrationb-on"></a>[`ShowCalibration(b on)`](dbus/io-github-reiwand-daemon1.md#showcalibrationb-on)
- <a id="previewscenes-output-s-dir-s-document-d-time"></a>[`PreviewScene(s output, s dir, s document, d time)`](dbus/io-github-reiwand-daemon1.md#previewscenes-output-s-dir-s-document-d-time)
- <a id="setwallpapers-path-s-output-s-fit"></a>[`SetWallpaper(s path, s output, s fit)`](dbus/io-github-reiwand-daemon1.md#setwallpapers-path-s-output-s-fit)
- <a id="clearwallpapers-output"></a>[`ClearWallpaper(s output)`](dbus/io-github-reiwand-daemon1-clearwallpaper-s-output.md#clearwallpapers-output)
- <a id="restorewallpapers-output"></a>[`RestoreWallpaper(s output)`](dbus/io-github-reiwand-daemon1-clearwallpaper-s-output.md#restorewallpapers-output)
- <a id="setlockwallpapers-uid-s-output"></a>[`SetLockWallpaper(s uid, s output)`](dbus/io-github-reiwand-daemon1-clearwallpaper-s-output.md#setlockwallpapers-uid-s-output)
- <a id="setlockplaylists-playlist-s-output"></a>[`SetLockPlaylist(s playlist, s output)`](dbus/io-github-reiwand-daemon1-clearwallpaper-s-output.md#setlockplaylists-playlist-s-output)
- <a id="followdesktoponlocks-output"></a>[`FollowDesktopOnLock(s output)`](dbus/io-github-reiwand-daemon1-clearwallpaper-s-output.md#followdesktoponlocks-output)
- <a id="showdesktoponlocks-output"></a>[`ShowDesktopOnLock(s output)`](dbus/io-github-reiwand-daemon1-clearwallpaper-s-output.md#showdesktoponlocks-output)
- <a id="getlock--s-json"></a>[`GetLock() → (s json)`](dbus/io-github-reiwand-daemon1-clearwallpaper-s-output.md#getlock--s-json)
- <a id="handbacktoplasma--u-screens"></a>[`HandBackToPlasma() → (u screens)`](dbus/io-github-reiwand-daemon1-clearwallpaper-s-output.md#handbacktoplasma--u-screens)
- <a id="setbackgrounds-color-s-output"></a>[`SetBackground(s color, s output)`](dbus/io-github-reiwand-daemon1-clearwallpaper-s-output.md#setbackgrounds-color-s-output)
- <a id="setfilters-filter-s-output"></a>[`SetFilter(s filter, s output)`](dbus/io-github-reiwand-daemon1-clearwallpaper-s-output.md#setfilters-filter-s-output)
- <a id="getscenepropertiess-output--s-json"></a>[`GetSceneProperties(s output) → (s json)`](dbus/io-github-reiwand-daemon1-clearwallpaper-s-output.md#getscenepropertiess-output--s-json)
- <a id="setscenepropertys-output-s-id-s-value_json"></a>[`SetSceneProperty(s output, s id, s value_json)`](dbus/io-github-reiwand-daemon1-clearwallpaper-s-output.md#setscenepropertys-output-s-id-s-value_json)
- <a id="listapplications--s-json"></a>[`ListApplications() → (s json)`](dbus/io-github-reiwand-daemon1-listapplications-s-json.md#listapplications--s-json)
- <a id="checkshortcuts-value_json"></a>[`CheckShortcut(s value_json)`](dbus/io-github-reiwand-daemon1-listapplications-s-json.md#checkshortcuts-value_json)
- <a id="applyscenepresets-output-s-preset"></a>[`ApplyScenePreset(s output, s preset)`](dbus/io-github-reiwand-daemon1-listapplications-s-json.md#applyscenepresets-output-s-preset)
- <a id="resetscenepropertiess-output"></a>[`ResetSceneProperties(s output)`](dbus/io-github-reiwand-daemon1-listapplications-s-json.md#resetscenepropertiess-output)
- <a id="savescenepresets-output-s-name--s-preset"></a>[`SaveScenePreset(s output, s name) → (s preset)`](dbus/io-github-reiwand-daemon1-listapplications-s-json.md#savescenepresets-output-s-name--s-preset)
- <a id="renamescenepresets-preset-s-name"></a>[`RenameScenePreset(s preset, s name)`](dbus/io-github-reiwand-daemon1-listapplications-s-json.md#renamescenepresets-preset-s-name)
- <a id="deletescenepresets-preset"></a>[`DeleteScenePreset(s preset)`](dbus/io-github-reiwand-daemon1-listapplications-s-json.md#deletescenepresets-preset)
- <a id="exportscenevaluess-output-s-preset--s-values"></a>[`ExportSceneValues(s output, s preset) → (s values)`](dbus/io-github-reiwand-daemon1-listapplications-s-json.md#exportscenevaluess-output-s-preset--s-values)
- <a id="importscenevaluess-output-s-json-s-name--as-warnings"></a>[`ImportSceneValues(s output, s json, s name) → (as warnings)`](dbus/io-github-reiwand-daemon1-listapplications-s-json.md#importscenevaluess-output-s-json-s-name--as-warnings)
- <a id="pressscenebuttons-output-s-id"></a>[`PressSceneButton(s output, s id)`](dbus/io-github-reiwand-daemon1-listapplications-s-json.md#pressscenebuttons-output-s-id)
- <a id="capturepages-output--s-path"></a>[`CapturePage(s output) → (s path)`](dbus/io-github-reiwand-daemon1-listapplications-s-json.md#capturepages-output--s-path)
- <a id="getparticlestatss-output--s-json"></a>[`GetParticleStats(s output) → (s json)`](dbus/io-github-reiwand-daemon1-listapplications-s-json.md#getparticlestatss-output--s-json)
- <a id="sharescenevaluess-output-b-shared"></a>[`ShareSceneValues(s output, b shared)`](dbus/io-github-reiwand-daemon1-listapplications-s-json.md#sharescenevaluess-output-b-shared)
- <a id="setoptions-key-s-value"></a>[`SetOption(s key, s value)`](dbus/io-github-reiwand-daemon1-setoption-s-key-s-value.md#setoptions-key-s-value)
- <a id="reload"></a>[`Reload()`](dbus/io-github-reiwand-daemon1-reload.md#reload)
- <a id="restart"></a>[`Restart()`](dbus/io-github-reiwand-daemon1-reload.md#restart)
- <a id="retryeffects"></a>[`RetryEffects()`](dbus/io-github-reiwand-daemon1-reload.md#retryeffects)
- <a id="retrylayerss-path"></a>[`RetryLayers(s path)`](dbus/io-github-reiwand-daemon1-reload.md#retrylayerss-path)
- <a id="getstate--s-json"></a>[`GetState() → (s json)`](dbus/io-github-reiwand-daemon1-reload.md#getstate--s-json)
- <a id="getschema--s-json"></a>[`GetSchema() → (s json)`](dbus/io-github-reiwand-daemon1-reload.md#getschema--s-json)
- <a id="setpausedb-paused"></a>[`SetPaused(b paused)`](dbus/io-github-reiwand-daemon1-reload.md#setpausedb-paused)
- <a id="setvolumed-volume"></a>[`SetVolume(d volume)`](dbus/io-github-reiwand-daemon1-reload.md#setvolumed-volume)
- <a id="setmutedb-muted"></a>[`SetMuted(b muted)`](dbus/io-github-reiwand-daemon1-reload.md#setmutedb-muted)
- <a id="setprivatemodeb-on"></a>[`SetPrivateMode(b on)`](dbus/io-github-reiwand-daemon1-reload.md#setprivatemodeb-on)
- <a id="waitchangedt-serial-u-timeout_ms--t-serial-as-topics"></a>[`WaitChanged(t serial, u timeout_ms) → (t serial, as topics)`](dbus/io-github-reiwand-daemon1-reload.md#waitchangedt-serial-u-timeout_ms--t-serial-as-topics)
- <a id="listjobs--s-json"></a>[`ListJobs() → (s json)`](dbus/io-github-reiwand-daemon1-reload.md#listjobs--s-json)
- <a id="getjobt-job--s-json"></a>[`GetJob(t job) → (s json)`](dbus/io-github-reiwand-daemon1-reload.md#getjobt-job--s-json)
- <a id="canceljobt-job"></a>[`CancelJob(t job)`](dbus/io-github-reiwand-daemon1-reload.md#canceljobt-job)
- <a id="property-paused-b"></a>[Property `Paused` (`b`)](dbus/io-github-reiwand-daemon1-reload.md#property-paused-b)
- <a id="property-volume-d"></a>[Property `Volume` (`d`)](dbus/io-github-reiwand-daemon1-reload.md#property-volume-d)
- <a id="property-muted-b"></a>[Property `Muted` (`b`)](dbus/io-github-reiwand-daemon1-reload.md#property-muted-b)
- <a id="property-privatemode-b"></a>[Property `PrivateMode` (`b`)](dbus/io-github-reiwand-daemon1-reload.md#property-privatemode-b)
- <a id="property-serial-t"></a>[Property `Serial` (`t`)](dbus/io-github-reiwand-daemon1-reload.md#property-serial-t)
- <a id="property-model-u"></a>[Property `Model` (`u`)](dbus/io-github-reiwand-daemon1-reload.md#property-model-u)
- <a id="signal-streamchangeds-output-u-node_id-t-serial"></a>[Signal `StreamChanged(s output, u node_id, t serial)`](dbus/io-github-reiwand-daemon1-reload.md#signal-streamchangeds-output-u-node_id-t-serial)
- <a id="signal-changeds-topic-as-ids-t-serial"></a>[Signal `Changed(s topic, as ids, t serial)`](dbus/io-github-reiwand-daemon1-reload.md#signal-changeds-topic-as-ids-t-serial)
- <a id="signal-jobprogresst-job-t-done-t-total"></a>[Signal `JobProgress(t job, t done, t total)`](dbus/io-github-reiwand-daemon1-reload.md#signal-jobprogresst-job-t-done-t-total)
- <a id="signal-jobfinishedt-job-s-state-s-error_name-s-message"></a>[Signal `JobFinished(t job, s state, s error_name, s message)`](dbus/io-github-reiwand-daemon1-reload.md#signal-jobfinishedt-job-s-state-s-error_name-s-message)
- <a id="iogithubreiwandlibrary1"></a>[`io.github.reiwand.Library1`](dbus/io-github-reiwand-library1.md#iogithubreiwandlibrary1)
- <a id="addsources-path-b-recursive--s-source-t-job"></a>[`AddSource(s path, b recursive) → (s source, t job)`](dbus/io-github-reiwand-library1.md#addsources-path-b-recursive--s-source-t-job)
- <a id="removesources-source"></a>[`RemoveSource(s source)`](dbus/io-github-reiwand-library1.md#removesources-source)
- <a id="listsources--s-json"></a>[`ListSources() → (s json)`](dbus/io-github-reiwand-library1.md#listsources--s-json)
- <a id="rescans-source--t-job"></a>[`Rescan(s source) → (t job)`](dbus/io-github-reiwand-library1.md#rescans-source--t-job)
- <a id="importas-paths-b-copy--t-job"></a>[`Import(as paths, b copy) → (t job)`](dbus/io-github-reiwand-library1.md#importas-paths-b-copy--t-job)
- <a id="importpackagesas-paths-b-allow_older--t-job"></a>[`ImportPackages(as paths, b allow_older) → (t job)`](dbus/io-github-reiwand-library1.md#importpackagesas-paths-b-allow_older--t-job)
- <a id="resolveas-refs--as-uids"></a>[`Resolve(as refs) → (as uids)`](dbus/io-github-reiwand-library1.md#resolveas-refs--as-uids)
- <a id="removeas-uids-b-delete_files"></a>[`Remove(as uids, b delete_files)`](dbus/io-github-reiwand-library1.md#removeas-uids-b-delete_files)
- <a id="searchs-query-u-offset-u-limit--s-json"></a>[`Search(s query, u offset, u limit) → (s json)`](dbus/io-github-reiwand-library1.md#searchs-query-u-offset-u-limit--s-json)
- <a id="getitems-uid--s-json"></a>[`GetItem(s uid) → (s json)`](dbus/io-github-reiwand-library1.md#getitems-uid--s-json)
- <a id="settitles-uid-s-title"></a>[`SetTitle(s uid, s title)`](dbus/io-github-reiwand-library1.md#settitles-uid-s-title)
- <a id="setratingas-uids-u-rating"></a>[`SetRating(as uids, u rating)`](dbus/io-github-reiwand-library1.md#setratingas-uids-u-rating)
- <a id="setsoundas-uids-s-muted-s-volume"></a>[`SetSound(as uids, s muted, s volume)`](dbus/io-github-reiwand-library1.md#setsoundas-uids-s-muted-s-volume)
- <a id="setfavoriteas-uids-b-favorite"></a>[`SetFavorite(as uids, b favorite)`](dbus/io-github-reiwand-library1.md#setfavoriteas-uids-b-favorite)
- <a id="settagsas-uids-as-add-as-remove"></a>[`SetTags(as uids, as add, as remove)`](dbus/io-github-reiwand-library1.md#settagsas-uids-as-add-as-remove)
- <a id="listtags--s-json"></a>[`ListTags() → (s json)`](dbus/io-github-reiwand-library1.md#listtags--s-json)
- <a id="renametags-from-s-to"></a>[`RenameTag(s from, s to)`](dbus/io-github-reiwand-library1-renametag-s-from-s-to.md#renametags-from-s-to)
- <a id="thumbnails-uid-u-size--s-path"></a>[`Thumbnail(s uid, u size) → (s path)`](dbus/io-github-reiwand-library1-renametag-s-from-s-to.md#thumbnails-uid-u-size--s-path)
- <a id="requestthumbnailsas-uids-u-size--ass-paths"></a>[`RequestThumbnails(as uids, u size) → (a{ss} paths)`](dbus/io-github-reiwand-library1-renametag-s-from-s-to.md#requestthumbnailsas-uids-u-size--ass-paths)
- <a id="applys-uid-s-output-s-fit-s-preset"></a>[`Apply(s uid, s output, s fit, s preset)`](dbus/io-github-reiwand-library1-renametag-s-from-s-to.md#applys-uid-s-output-s-fit-s-preset)
- <a id="confirmas-uids-s-what"></a>[`Confirm(as uids, s what)`](dbus/io-github-reiwand-library1-renametag-s-from-s-to.md#confirmas-uids-s-what)
- <a id="unconfirmas-uids-s-what"></a>[`Unconfirm(as uids, s what)`](dbus/io-github-reiwand-library1-renametag-s-from-s-to.md#unconfirmas-uids-s-what)
- <a id="cleardataas-uids"></a>[`ClearData(as uids)`](dbus/io-github-reiwand-library1-renametag-s-from-s-to.md#cleardataas-uids)
- <a id="signal-thumbnailreadys-uid-u-size-s-path"></a>[Signal `ThumbnailReady(s uid, u size, s path)`](dbus/io-github-reiwand-library1-renametag-s-from-s-to.md#signal-thumbnailreadys-uid-u-size-s-path)
- <a id="iogithubreiwandplaylists1"></a>[`io.github.reiwand.Playlists1`](dbus/io-github-reiwand-playlists1.md#iogithubreiwandplaylists1)
- <a id="list--s-json"></a>[`List() → (s json)`](dbus/io-github-reiwand-playlists1.md#list--s-json)
- <a id="gets-playlist-u-offset-u-limit--s-json"></a>[`Get(s playlist, u offset, u limit) → (s json)`](dbus/io-github-reiwand-playlists1.md#gets-playlist-u-offset-u-limit--s-json)
- <a id="creates-name-s-query--s-uid"></a>[`Create(s name, s query) → (s uid)`](dbus/io-github-reiwand-playlists1.md#creates-name-s-query--s-uid)
- <a id="renames-playlist-s-name"></a>[`Rename(s playlist, s name)`](dbus/io-github-reiwand-playlists1.md#renames-playlist-s-name)
- <a id="setquerys-playlist-s-query"></a>[`SetQuery(s playlist, s query)`](dbus/io-github-reiwand-playlists1.md#setquerys-playlist-s-query)
- <a id="deletes-playlist"></a>[`Delete(s playlist)`](dbus/io-github-reiwand-playlists1.md#deletes-playlist)
- <a id="additemss-playlist-as-uids"></a>[`AddItems(s playlist, as uids)`](dbus/io-github-reiwand-playlists1.md#additemss-playlist-as-uids)
- <a id="addfolders-playlist-s-path-b-follow--u-count"></a>[`AddFolder(s playlist, s path, b follow) → (u count)`](dbus/io-github-reiwand-playlists1.md#addfolders-playlist-s-path-b-follow--u-count)
- <a id="rotates-name-as-uids-s-folder-b-follow-s-every-s-output--s-uid"></a>[`Rotate(s name, as uids, s folder, b follow, s every, s output) → (s uid)`](dbus/io-github-reiwand-playlists1.md#rotates-name-as-uids-s-folder-b-follow-s-every-s-output--s-uid)
- <a id="removeitemss-playlist-as-uids"></a>[`RemoveItems(s playlist, as uids)`](dbus/io-github-reiwand-playlists1.md#removeitemss-playlist-as-uids)
- <a id="removeentriess-playlist-au-indexes-as-expected"></a>[`RemoveEntries(s playlist, au indexes, as expected)`](dbus/io-github-reiwand-playlists1.md#removeentriess-playlist-au-indexes-as-expected)
- <a id="moveitems-playlist-u-from-u-to-s-expected"></a>[`MoveItem(s playlist, u from, u to, s expected)`](dbus/io-github-reiwand-playlists1.md#moveitems-playlist-u-from-u-to-s-expected)
- <a id="setrules-playlist-s-key-s-value"></a>[`SetRule(s playlist, s key, s value)`](dbus/io-github-reiwand-playlists1.md#setrules-playlist-s-key-s-value)
- <a id="starts-playlist-s-output"></a>[`Start(s playlist, s output)`](dbus/io-github-reiwand-playlists1.md#starts-playlist-s-output)
- <a id="stops-output"></a>[`Stop(s output)`](dbus/io-github-reiwand-playlists1.md#stops-output)
- <a id="nexts-output"></a>[`Next(s output)`](dbus/io-github-reiwand-playlists1.md#nexts-output)
- <a id="previouss-output"></a>[`Previous(s output)`](dbus/io-github-reiwand-playlists1.md#previouss-output)
- <a id="historys-output-u-limit--s-json"></a>[`History(s output, u limit) → (s json)`](dbus/io-github-reiwand-playlists1-history-s-output-u-limit.md#historys-output-u-limit--s-json)
- <a id="browses-output-s-source-i-steps--s-json"></a>[`Browse(s output, s source, i steps) → (s json)`](dbus/io-github-reiwand-playlists1-history-s-output-u-limit.md#browses-output-s-source-i-steps--s-json)
- <a id="browseapplys-output-s-transition--s-json"></a>[`BrowseApply(s output, s transition) → (s json)`](dbus/io-github-reiwand-playlists1-history-s-output-u-limit.md#browseapplys-output-s-transition--s-json)
- <a id="browsetransitions-output-s-transition--s-json"></a>[`BrowseTransition(s output, s transition) → (s json)`](dbus/io-github-reiwand-playlists1-history-s-output-u-limit.md#browsetransitions-output-s-transition--s-json)
- <a id="browsecancels-output--s-json"></a>[`BrowseCancel(s output) → (s json)`](dbus/io-github-reiwand-playlists1-history-s-output-u-limit.md#browsecancels-output--s-json)
- <a id="getbrowses-output--s-json"></a>[`GetBrowse(s output) → (s json)`](dbus/io-github-reiwand-playlists1-history-s-output-u-limit.md#getbrowses-output--s-json)
- <a id="signal-browsechangeds-output-s-json"></a>[Signal `BrowseChanged(s output, s json)`](dbus/io-github-reiwand-playlists1-history-s-output-u-limit.md#signal-browsechangeds-output-s-json)
- <a id="iogithubreiwandprofiles1"></a>[`io.github.reiwand.Profiles1`](dbus/io-github-reiwand-profiles1.md#iogithubreiwandprofiles1)
- <a id="list--s-json-1"></a>[`List() → (s json)`](dbus/io-github-reiwand-profiles1.md#list--s-json)
- <a id="gets-profile--s-json"></a>[`Get(s profile) → (s json)`](dbus/io-github-reiwand-profiles1.md#gets-profile--s-json)
- <a id="saves-name-b-with_options--s-uid"></a>[`Save(s name, b with_options) → (s uid)`](dbus/io-github-reiwand-profiles1.md#saves-name-b-with_options--s-uid)
- <a id="activates-profile"></a>[`Activate(s profile)`](dbus/io-github-reiwand-profiles1.md#activates-profile)
- <a id="renames-profile-s-name"></a>[`Rename(s profile, s name)`](dbus/io-github-reiwand-profiles1.md#renames-profile-s-name)
- <a id="deletes-profile"></a>[`Delete(s profile)`](dbus/io-github-reiwand-profiles1.md#deletes-profile)
- <a id="setautos-profile-s-key-s-value"></a>[`SetAuto(s profile, s key, s value)`](dbus/io-github-reiwand-profiles1.md#setautos-profile-s-key-s-value)
- <a id="listpresetss-item--s-json"></a>[`ListPresets(s item) → (s json)`](dbus/io-github-reiwand-profiles1.md#listpresetss-item--s-json)
- <a id="savepresets-item-s-name-ass-values--s-uid"></a>[`SavePreset(s item, s name, a{ss} values) → (s uid)`](dbus/io-github-reiwand-profiles1.md#savepresets-item-s-name-ass-values--s-uid)
- <a id="deletepresets-preset"></a>[`DeletePreset(s preset)`](dbus/io-github-reiwand-profiles1.md#deletepresets-preset)
- <a id="iogithubreiwandproviders1"></a>[`io.github.reiwand.Providers1`](dbus/io-github-reiwand-providers1.md#iogithubreiwandproviders1)
- <a id="list--s-json-2"></a>[`List() → (s json)`](dbus/io-github-reiwand-providers1.md#list--s-json)
- <a id="adds-id-s-kind-ass-options-s-secret--s-source"></a>[`Add(s id, s kind, a{ss} options, s secret) → (s source)`](dbus/io-github-reiwand-providers1.md#adds-id-s-kind-ass-options-s-secret--s-source)
- <a id="sets-id-s-key-s-value"></a>[`Set(s id, s key, s value)`](dbus/io-github-reiwand-providers1.md#sets-id-s-key-s-value)
- <a id="updates-id-ass-options"></a>[`Update(s id, a{ss} options)`](dbus/io-github-reiwand-providers1.md#updates-id-ass-options)
- <a id="removes-id-b-purge"></a>[`Remove(s id, b purge)`](dbus/io-github-reiwand-providers1.md#removes-id-b-purge)
- <a id="clearcaches-id"></a>[`ClearCache(s id)`](dbus/io-github-reiwand-providers1-clearcache-s-id.md#clearcaches-id)
- <a id="syncs-id--t-job"></a>[`Sync(s id) → (t job)`](dbus/io-github-reiwand-providers1-clearcache-s-id.md#syncs-id--t-job)
- <a id="collectionss-id-s-owner--s-json"></a>[`Collections(s id, s owner) → (s json)`](dbus/io-github-reiwand-providers1-clearcache-s-id.md#collectionss-id-s-owner--s-json)
- <a id="reseeds-id--t-job"></a>[`Reseed(s id) → (t job)`](dbus/io-github-reiwand-providers1-clearcache-s-id.md#reseeds-id--t-job)
- <a id="setsecrets-id-s-secret"></a>[`SetSecret(s id, s secret)`](dbus/io-github-reiwand-providers1-clearcache-s-id.md#setsecrets-id-s-secret)
- <a id="forgetsecrets-id"></a>[`ForgetSecret(s id)`](dbus/io-github-reiwand-providers1-clearcache-s-id.md#forgetsecrets-id)
- <a id="setsecretstores-store"></a>[`SetSecretStore(s store)`](dbus/io-github-reiwand-providers1-clearcache-s-id.md#setsecretstores-store)
- <a id="inspectplugins-path--s-json"></a>[`InspectPlugin(s path) → (s json)`](dbus/io-github-reiwand-providers1-clearcache-s-id.md#inspectplugins-path--s-json)
- <a id="installplugins-path-s-token--s-kind"></a>[`InstallPlugin(s path, s token) → (s kind)`](dbus/io-github-reiwand-providers1-clearcache-s-id.md#installplugins-path-s-token--s-kind)
- <a id="listplugins--s-json"></a>[`ListPlugins() → (s json)`](dbus/io-github-reiwand-providers1-clearcache-s-id.md#listplugins--s-json)
- <a id="removeplugins-id"></a>[`RemovePlugin(s id)`](dbus/io-github-reiwand-providers1-clearcache-s-id.md#removeplugins-id)
- <a id="iogithubreiwandrepos1"></a>[`io.github.reiwand.Repos1`](dbus/io-github-reiwand-repos1.md#iogithubreiwandrepos1)
- <a id="list--s-json-3"></a>[`List() → (s json)`](dbus/io-github-reiwand-repos1.md#list--s-json)
- <a id="adds-url--t-job"></a>[`Add(s url) → (t job)`](dbus/io-github-reiwand-repos1.md#adds-url--t-job)
- <a id="addtrustings-url-s-certificate--t-job"></a>[`AddTrusting(s url, s certificate) → (t job)`](dbus/io-github-reiwand-repos1.md#addtrustings-url-s-certificate--t-job)
- <a id="addexamples--t-job"></a>[`AddExamples() → (t job)`](dbus/io-github-reiwand-repos1.md#addexamples--t-job)
- <a id="refreshs-repo--t-job"></a>[`Refresh(s repo) → (t job)`](dbus/io-github-reiwand-repos1.md#refreshs-repo--t-job)
- <a id="removes-repo-b-uninstall"></a>[`Remove(s repo, b uninstall)`](dbus/io-github-reiwand-repos1.md#removes-repo-b-uninstall)
- <a id="browses-query-u-offset-u-limit--s-json"></a>[`Browse(s query, u offset, u limit) → (s json)`](dbus/io-github-reiwand-repos1.md#browses-query-u-offset-u-limit--s-json)
- <a id="reveals-spec"></a>[`Reveal(s spec)`](dbus/io-github-reiwand-repos1.md#reveals-spec)
- <a id="requestpreviewsas-specs--ass-paths"></a>[`RequestPreviews(as specs) → (a{ss} paths)`](dbus/io-github-reiwand-repos1.md#requestpreviewsas-specs--ass-paths)
- <a id="requestanimatedpreviewsas-specs--ass-paths"></a>[`RequestAnimatedPreviews(as specs) → (a{ss} paths)`](dbus/io-github-reiwand-repos1.md#requestanimatedpreviewsas-specs--ass-paths)
- <a id="installs-spec-b-allow_stale-b-allow_downgrade--t-job"></a>[`Install(s spec, b allow_stale, b allow_downgrade) → (t job)`](dbus/io-github-reiwand-repos1.md#installs-spec-b-allow_stale-b-allow_downgrade--t-job)
- <a id="uninstalls-id-s-version"></a>[`Uninstall(s id, s version)`](dbus/io-github-reiwand-repos1.md#uninstalls-id-s-version)
- <a id="listinstalled--s-json"></a>[`ListInstalled() → (s json)`](dbus/io-github-reiwand-repos1.md#listinstalled--s-json)
- <a id="signal-previewreadys-spec-s-path"></a>[Signal `PreviewReady(s spec, s path)`](dbus/io-github-reiwand-repos1.md#signal-previewreadys-spec-s-path)
- <a id="iogithubreiwandinputs1"></a>[`io.github.reiwand.Inputs1`](dbus/io-github-reiwand-inputs1.md#iogithubreiwandinputs1)
- <a id="grants-scene-s-input"></a>[`Grant(s scene, s input)`](dbus/io-github-reiwand-inputs1.md#grants-scene-s-input)
- <a id="grantpackages-scene-s-key-s-input"></a>[`GrantPackage(s scene, s key, s input)`](dbus/io-github-reiwand-inputs1.md#grantpackages-scene-s-key-s-input)
- <a id="decides-scene-s-key-as-allow-as-deny"></a>[`Decide(s scene, s key, as allow, as deny)`](dbus/io-github-reiwand-inputs1-decide-s-scene-s-key-as-.md#decides-scene-s-key-as-allow-as-deny)
- <a id="revokes-scene-s-input"></a>[`Revoke(s scene, s input)`](dbus/io-github-reiwand-inputs1-decide-s-scene-s-key-as-.md#revokes-scene-s-input)
- <a id="getinputs--s-json"></a>[`GetInputs() → (s json)`](dbus/io-github-reiwand-inputs1-decide-s-scene-s-key-as-.md#getinputs--s-json)
- <a id="setclickss-output-b-on"></a>[`SetClicks(s output, b on)`](dbus/io-github-reiwand-inputs1-decide-s-scene-s-key-as-.md#setclickss-output-b-on)
- <a id="simulatemedias-output-s-track-ay-cover"></a>[`SimulateMedia(s output, s track, ay cover)`](dbus/io-github-reiwand-inputs1-decide-s-scene-s-key-as-.md#simulatemedias-output-s-track-ay-cover)
- <a id="simulatetouchs-output-d-x-d-y-u-buttons"></a>[`SimulateTouch(s output, d x, d y, u buttons)`](dbus/io-github-reiwand-inputs1-decide-s-scene-s-key-as-.md#simulatetouchs-output-d-x-d-y-u-buttons)
- <a id="simulatetriggers-output-s-trigger"></a>[`SimulateTrigger(s output, s trigger)`](dbus/io-github-reiwand-inputs1-decide-s-scene-s-key-as-.md#simulatetriggers-output-s-trigger)
- <a id="bandss-output--s-json"></a>[`Bands(s output) → (s json)`](dbus/io-github-reiwand-inputs1-decide-s-scene-s-key-as-.md#bandss-output--s-json)
- <a id="touchs-output-d-x-d-y-u-buttons"></a>[`Touch(s output, d x, d y, u buttons)`](dbus/io-github-reiwand-inputs1-touch-s-output-d-x-d-y-u.md#touchs-output-d-x-d-y-u-buttons)
- <a id="setpointeri-x-i-y"></a>[`SetPointer(i x, i y)`](dbus/io-github-reiwand-inputs1-touch-s-output-d-x-d-y-u.md#setpointeri-x-i-y)
- <a id="iogithubreiwandrgb1"></a>[`io.github.reiwand.Rgb1`](dbus/io-github-reiwand-rgb1.md#iogithubreiwandrgb1)
- <a id="getstatus--s-json"></a>[`GetStatus() → (s json)`](dbus/io-github-reiwand-rgb1.md#getstatus--s-json)
- <a id="connect"></a>[`Connect()`](dbus/io-github-reiwand-rgb1.md#connect)
- <a id="testi-device"></a>[`Test(i device)`](dbus/io-github-reiwand-rgb1.md#testi-device)
- <a id="getstrips-output--u-width-u-height-ay-rgb"></a>[`GetStrip(s output) → (u width, u height, ay rgb)`](dbus/io-github-reiwand-rgb1.md#getstrips-output--u-width-u-height-ay-rgb)
- <a id="iogithubreiwandpublish1"></a>[`io.github.reiwand.Publish1`](dbus/io-github-reiwand-publish1.md#iogithubreiwandpublish1)
- <a id="uploadass-repo-s-path-s-visibility--t-job"></a>[`UploadAs(s repo, s path, s visibility) → (t job)`](dbus/io-github-reiwand-publish1.md#uploadass-repo-s-path-s-visibility--t-job)
- <a id="listpackagess-repo--s-json"></a>[`ListPackages(s repo) → (s json)`](dbus/io-github-reiwand-publish1.md#listpackagess-repo--s-json)
- <a id="setvisibilitys-repo-s-id-s-version-s-visibility"></a>[`SetVisibility(s repo, s id, s version, s visibility)`](dbus/io-github-reiwand-publish1.md#setvisibilitys-repo-s-id-s-version-s-visibility)
- <a id="withdrawpackages-repo-s-id"></a>[`WithdrawPackage(s repo, s id)`](dbus/io-github-reiwand-publish1.md#withdrawpackages-repo-s-id)
- <a id="listauthorkeyss-repo--s-json"></a>[`ListAuthorKeys(s repo) → (s json)`](dbus/io-github-reiwand-publish1.md#listauthorkeyss-repo--s-json)
- <a id="addauthorkeys-repo-s-public_key--s-fingerprint"></a>[`AddAuthorKey(s repo, s public_key) → (s fingerprint)`](dbus/io-github-reiwand-publish1.md#addauthorkeys-repo-s-public_key--s-fingerprint)
- <a id="rotateauthorkeys-repo-s-from-s-public_key-s-signature--s-fingerprint"></a>[`RotateAuthorKey(s repo, s from, s public_key, s signature) → (s fingerprint)`](dbus/io-github-reiwand-publish1.md#rotateauthorkeys-repo-s-from-s-public_key-s-signature--s-fingerprint)
- <a id="removeauthorkeys-repo-s-fingerprint"></a>[`RemoveAuthorKey(s repo, s fingerprint)`](dbus/io-github-reiwand-publish1.md#removeauthorkeys-repo-s-fingerprint)
- <a id="listuploadss-repo--s-json"></a>[`ListUploads(s repo) → (s json)`](dbus/io-github-reiwand-publish1.md#listuploadss-repo--s-json)
- <a id="canceluploads-repo-s-id"></a>[`CancelUpload(s repo, s id)`](dbus/io-github-reiwand-publish1.md#canceluploads-repo-s-id)
- <a id="votes-id-s-vote--s-json"></a>[`Vote(s id, s vote) → (s json)`](dbus/io-github-reiwand-publish1.md#votes-id-s-vote--s-json)
- <a id="getvotesas-ids--s-json"></a>[`GetVotes(as ids) → (s json)`](dbus/io-github-reiwand-publish1.md#getvotesas-ids--s-json)
- <a id="iogithubreiwandcarry1"></a>[`io.github.reiwand.Carry1`](dbus/io-github-reiwand-publish1.md#iogithubreiwandcarry1)
- <a id="exports-path-s-options_json--t-job"></a>[`Export(s path, s options_json) → (t job)`](dbus/io-github-reiwand-publish1.md#exports-path-s-options_json--t-job)
- <a id="previewimports-path-s-password--s-json"></a>[`PreviewImport(s path, s password) → (s json)`](dbus/io-github-reiwand-publish1.md#previewimports-path-s-password--s-json)
- <a id="importchosens-path-s-password-s-choices_json--t-job"></a>[`ImportChosen(s path, s password, s choices_json) → (t job)`](dbus/io-github-reiwand-publish1.md#importchosens-path-s-password-s-choices_json--t-job)
- <a id="iogithubreiwandaccounts1"></a>[`io.github.reiwand.Accounts1`](dbus/io-github-reiwand-publish1.md#iogithubreiwandaccounts1)
- <a id="list--s-json-4"></a>[`List() → (s json)`](dbus/io-github-reiwand-publish1.md#list--s-json)
- <a id="signins-repo_url--s-user_code-s-verification_uri-t-job"></a>[`SignIn(s repo_url) → (s user_code, s verification_uri, t job)`](dbus/io-github-reiwand-publish1.md#signins-repo_url--s-user_code-s-verification_uri-t-job)
- <a id="uses-repo-s-account"></a>[`Use(s repo, s account)`](dbus/io-github-reiwand-publish1.md#uses-repo-s-account)
- <a id="unlinks-repo"></a>[`Unlink(s repo)`](dbus/io-github-reiwand-publish1.md#unlinks-repo)
- <a id="signouts-account"></a>[`SignOut(s account)`](dbus/io-github-reiwand-publish1.md#signouts-account)
- <a id="removes-account"></a>[`Remove(s account)`](dbus/io-github-reiwand-publish1.md#removes-account)
- <a id="the-app-iogithubreiwand"></a>[The app: `io.github.reiwand`](dbus/the-app-io-github-reiwand.md#the-app-iogithubreiwand)
- <a id="edits-and-spanning"></a>[Edits and spanning](dbus/edits-and-spanning.md#edits-and-spanning)
- <a id="scenes"></a>[Scenes](dbus/scenes.md#scenes)
- <a id="scene-sound"></a>[Scene sound](dbus/scenes.md#scene-sound)
- <a id="change-notification"></a>[Change notification](dbus/scenes.md#change-notification)
- <a id="jobs"></a>[Jobs](dbus/jobs.md#jobs)
- <a id="thumbnails"></a>[Thumbnails](dbus/jobs.md#thumbnails)
- <a id="json-model"></a>[JSON model](dbus/json-model.md#json-model)
- <a id="configuration-file"></a>[Configuration file](dbus/configuration-file.md#configuration-file)
