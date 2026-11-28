#include "page.h"
#include <QJsonArray>
#include <QJsonDocument>
#include <QMouseEvent>
#include <QTimer>

static QSize sized(uint32_t width, uint32_t height) { return {int(std::max(width, 1u)), int(std::max(height, 1u))}; }

int rw_open(uint32_t view, uint32_t width, uint32_t height, double scale) {
    return openPage(view, sized(width, height), std::max(scale, 0.25)) ? 1 : 0;
}

void rw_close(uint32_t view) { closePage(view); }

void rw_resize(uint32_t view, uint32_t width, uint32_t height, double scale) {
    if (Page *page = pageOf(view)) page->resize(sized(width, height), std::max(scale, 0.25));
}

void rw_visible(uint32_t view, int visible) {
    if (Page *page = pageOf(view)) page->show(visible != 0);
}

void rw_fps(uint32_t view, uint32_t fps) {
    if (Page *page = pageOf(view)) page->setFps(fps);
}

void rw_scripts(uint32_t view, const char *json) {
    Page *page = pageOf(view);
    if (!page) return;
    const QJsonArray list = QJsonDocument::fromJson(json).array();
    page->install(list);
}

void rw_load(uint32_t view, const char *url) {
    if (Page *page = pageOf(view)) page->call("load", QString::fromUtf8(url));
}

void rw_eval(uint32_t view, const char *code) {
    if (Page *page = pageOf(view)) page->call("run", QString::fromUtf8(code));
}

int64_t rw_renderer(uint32_t view) {
    Page *page = pageOf(view);
    return page && page->web() ? page->web()->property("renderProcessPid").toLongLong() : 0;
}

void rw_ping(uint32_t view) {
    if (Page *page = pageOf(view)) page->call("ping", QVariant());
}

void rw_muted(int muted) { world().bridge->mute(muted != 0); }

void rw_copy() { world().copy = true; }

void rw_released(uint32_t view, uint32_t id) {
    if (Page *page = pageOf(view)) page->released(id);
}

void rw_shot(uint32_t view) {
    if (Page *page = pageOf(view)) page->shot();
}

void rw_pointer(uint32_t view, int present, double x, double y) {
    Page *page = pageOf(view);
    if (!page) return;
    if (!present) {
        page->leave();
        return;
    }
    page->hover(QPointF(x, y));
}

static Qt::MouseButton buttonOf(uint32_t button) {
    switch (button) {
    case 2: return Qt::MiddleButton;
    case 3: return Qt::RightButton;
    case 4: return Qt::BackButton;
    case 5: return Qt::ForwardButton;
    default: return Qt::LeftButton;
    }
}

void rw_press(uint32_t view, double x, double y, uint32_t button) {
    Page *page = pageOf(view);
    if (!page) return;
    const QPointF at(x, y);
    const Qt::MouseButton b = buttonOf(button);
    QMouseEvent down(QEvent::MouseButtonPress, at, at, at, b, b, Qt::NoModifier);
    QMouseEvent up(QEvent::MouseButtonRelease, at, at, at, b, Qt::NoButton, Qt::NoModifier);
    QCoreApplication::sendEvent(page->surface(), &down);
    QCoreApplication::sendEvent(page->surface(), &up);
}

void rw_later(uint32_t millis, void (*run)(void *), void *data) {
    QTimer::singleShot(millis, [run, data] { run(data); });
}
