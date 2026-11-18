#include "page.h"

QString capScript(uint32_t fps) {
    static const char *const source = R"JS((() => {
  const raw = window.requestAnimationFrame.bind(window);
  const stop = window.cancelAnimationFrame.bind(window);
  const pending = new Map();
  let count = 0;
  let cap = %1;
  let last = -1e9;
  const issue = (callback, key) => {
    pending.set(key, raw((t) => {
      if (t !== last && t - last < 1000 / cap - 2) return issue(callback, key);
      last = t;
      pending.delete(key);
      callback(t);
    }));
  };
  window.requestAnimationFrame = (callback) => {
    count += 1;
    issue(callback, count);
    return count;
  };
  window.cancelAnimationFrame = (key) => {
    const inner = pending.get(key);
    if (inner !== undefined) {
      stop(inner);
      pending.delete(key);
    }
  };
  Object.defineProperty(window, "__rafCap", { value: (fps) => { cap = fps; } });
})();)JS";
    return QString::fromLatin1(source).arg(fps);
}
