((g, post, key, wall) => {
  // Frame clock of a rendered page: time stands still until the host steps it. Clocks, timers, animation frames
  // and the animations of the document follow it; the host draws the page once the step is painted.
  if (g.__reiwandStep) return;
  const RealDate = g.Date;
  const realTimeout = g.setTimeout.bind(g);
  const realFrame = g.requestAnimationFrame.bind(g);
  const timers = new Map();
  const frames = new Map();
  const started = new WeakMap();
  let now = 0;
  let ids = 0;
  let busy = false;
  const stamp = () => wall + now;
  g.Date = new Proxy(RealDate, {
    construct: (target, args, made) => Reflect.construct(target, args.length ? args : [stamp()], made),
    apply: () => new RealDate(stamp()).toString(),
    get: (target, name, self) => (name === "now" ? stamp : Reflect.get(target, name, self)),
  });
  Object.defineProperty(g.performance, "now", { value: () => now });
  const add = (callback, delay, args, every) => {
    const id = ++ids;
    const wait = Math.max(0, Number(delay) || 0);
    const run = typeof callback === "function" ? callback : () => (0, eval)(String(callback));
    timers.set(id, { at: now + wait, wait: every ? Math.max(wait, 1) : wait, run, args, every });
    return id;
  };
  g.setTimeout = (callback, delay, ...args) => add(callback, delay, args, false);
  g.setInterval = (callback, delay, ...args) => add(callback, delay, args, true);
  g.clearTimeout = g.clearInterval = (id) => void timers.delete(id);
  g.requestAnimationFrame = (callback) => {
    frames.set(++ids, callback);
    return ids;
  };
  g.cancelAnimationFrame = (id) => void frames.delete(id);
  const report = (e) => realTimeout(() => {
    throw e;
  });
  const due = (until) => {
    let first = null;
    for (const [id, t] of timers) if (t.at <= until && (!first || t.at < first[1].at)) first = [id, t];
    return first;
  };
  const step = async (until) => {
    for (let ran = 0, next = due(until); next && ran < 10000; ran++, next = due(until)) {
      const [id, t] = next;
      now = Math.max(now, t.at);
      if (t.every) t.at += t.wait;
      else timers.delete(id);
      try {
        t.run.apply(g, t.args);
      } catch (e) {
        report(e);
      }
      await new Promise((done) => realTimeout(done, 0));
    }
    now = Math.max(now, until);
    const called = [...frames.values()];
    frames.clear();
    for (const callback of called) {
      try {
        callback(now);
      } catch (e) {
        report(e);
      }
    }
    await new Promise((done) => realTimeout(done, 0));
    for (const a of g.document.getAnimations()) {
      if (!started.has(a)) started.set(a, now);
      a.pause();
      a.currentTime = now - started.get(a);
    }
    realFrame(() => realFrame(() => post({ stepped: until })));
  };
  Object.defineProperty(g, "__reiwandStep", {
    value: (given, until) => {
      if (given !== key || busy) return;
      busy = true;
      step(Number(until) || 0).finally(() => {
        busy = false;
      });
    },
  });
})
