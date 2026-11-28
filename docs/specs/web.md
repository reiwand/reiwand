# Web pages

A `web` package (`package.md`, "Web pages") is shown by `reiwand-web`, a helper process that the
daemon starts for each web package shown and stops as soon as no screen shows it. The helper runs
the page in Qt WebEngine (Chromium), hands every frame to the daemon, and the daemon draws it like any other
wallpaper: with the same transitions, fit, frame rate cap, pause rules and streams.

The specification, part by part:

- [Engine, Frames, Lifecycle and Session and the page's view](web/engine.md): engine, frames, lifecycle, session and the page's view.
- [Protocol, Frame clock, Console of a preview and Sandbox](web/protocol.md): protocol, frame clock, console of a preview, sandbox.
- [Privacy and The page interface](web/privacy.md): privacy, the page interface.
- [On the phone](web/on-the-phone.md): on the phone.

## Where the sections went

The sections of this page moved to the pages above; links to them still land here:

- <a id="engine"></a>[Engine](web/engine.md#engine)
- <a id="frames"></a>[Frames](web/engine.md#frames)
- <a id="lifecycle"></a>[Lifecycle](web/engine.md#lifecycle)
- <a id="session-and-the-pages-view"></a>[Session and the page's view](web/engine.md#session-and-the-pages-view)
- <a id="protocol"></a>[Protocol](web/protocol.md#protocol)
- <a id="frame-clock-scene-render"></a>[Frame clock (`scene render`)](web/protocol.md#frame-clock-scene-render)
- <a id="console-of-a-preview"></a>[Console of a preview](web/protocol.md#console-of-a-preview)
- <a id="sandbox"></a>[Sandbox](web/protocol.md#sandbox)
- <a id="privacy"></a>[Privacy](web/privacy.md#privacy)
- <a id="the-page-interface"></a>[The page interface](web/privacy.md#the-page-interface)
- <a id="on-the-phone"></a>[On the phone](web/on-the-phone.md#on-the-phone)
