(g, protections) => {
  // Each protection is `(window) => void`. The scripts of a frame run only once its document exists, which is after
  // the code that inserted it could already reach its window (`iframe.contentWindow` or `window[0]` right after
  // `append`): so every frame the page can reach is covered from here, synchronously, before it is handed out.
  const seen = new WeakSet();
  const cover = (w, inner) => {
    let doc;
    try {
      doc = w.document;
    } catch (e) {
      return;
    }
    if (!doc || seen.has(doc)) return;
    seen.add(doc);
    for (const protect of protections) {
      try {
        protect(w);
      } catch (e) {}
    }
    if (inner) follow(w);
    hook(w, doc);
  };
  const sweep = (w) => {
    let n = 0;
    try {
      n = w.length;
    } catch (e) {}
    for (let i = 0; i < n; i++) {
      let child;
      try {
        child = w[i];
      } catch (e) {
        continue;
      }
      if (!child) continue;
      cover(child, true);
      sweep(child);
    }
  };
  const all = () => sweep(g);
  // A frame paints at the pace of the page: its animation frames are those of its parent, which are capped.
  const follow = (w) => {
    const outer = w.parent;
    w.requestAnimationFrame = (callback) => outer.requestAnimationFrame(callback);
    w.cancelAnimationFrame = (key) => outer.cancelAnimationFrame(key);
  };
  const methods = ["appendChild", "insertBefore", "replaceChild", "append", "prepend", "before", "after", "replaceWith",
    "replaceChildren", "insertAdjacentElement", "insertAdjacentHTML", "insertNode", "surroundContents", "write",
    "writeln", "setHTML", "setHTMLUnsafe", "moveBefore", "execCommand"];
  const kinds = ["Node", "Element", "CharacterData", "DocumentType", "Document", "DocumentFragment", "ShadowRoot",
    "Range", "HTMLElement", "HTMLDocument"];
  const hook = (w, doc) => {
    for (const kind of kinds) {
      const proto = w[kind] && w[kind].prototype;
      if (!proto) continue;
      for (const name of methods) {
        const own = Object.getOwnPropertyDescriptor(proto, name);
        if (!own || typeof own.value !== "function") continue;
        const real = own.value;
        Object.defineProperty(proto, name, { ...own, value: { [name](...args) {
          try {
            return real.apply(this, args);
          } finally {
            all();
          }
        } }[name] });
      }
      for (const name of ["innerHTML", "outerHTML"]) {
        const own = Object.getOwnPropertyDescriptor(proto, name);
        if (!own || !own.set) continue;
        Object.defineProperty(proto, name, { ...own, set(value) {
          try {
            own.set.call(this, value);
          } finally {
            all();
          }
        } });
      }
    }
    for (const kind of ["HTMLIFrameElement", "HTMLFrameElement", "HTMLObjectElement", "HTMLEmbedElement"]) {
      const proto = w[kind] && w[kind].prototype;
      for (const name of ["contentWindow", "contentDocument"]) {
        const own = proto && Object.getOwnPropertyDescriptor(proto, name);
        if (!own || !own.get) continue;
        Object.defineProperty(proto, name, { ...own, get() {
          all();
          return own.get.call(this);
        } });
      }
    }
    // Frames written by the parser: the observer runs before the next script of the document does.
    try {
      new w.MutationObserver(all).observe(doc, { childList: true, subtree: true });
    } catch (e) {}
  };
  cover(g, false);
  all();
}
