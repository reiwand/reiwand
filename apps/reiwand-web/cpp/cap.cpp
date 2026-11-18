#include "page.h"
#include <QJsonArray>
#include <QJsonDocument>

QString capScript(uint32_t fps, const QString &key) {
    static const char *const source = R"JS((() => {
  const raw = window.requestAnimationFrame.bind(window);
  const stop = window.cancelAnimationFrame.bind(window);
  const pending = new Map();
  let count = 0;
  let cap = %1;
  const key = %2;
  let last = -1e9;
  const issue = (callback, id) => {
    pending.set(id, raw((t) => {
      if (t !== last && t - last < 1000 / cap - 2) return issue(callback, id);
      last = t;
      pending.delete(id);
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
  Object.defineProperty(window, "__rafCap", { value: (given, fps) => { if (given === key) cap = fps; } });
})();)JS";
    return QString::fromLatin1(source).arg(fps).arg(QString::fromUtf8(QJsonDocument(QJsonArray{key}).toJson(QJsonDocument::Compact)).mid(1).chopped(1));
}
