#include "page.h"
#include <QQuickGraphicsDevice>
#include <QJsonArray>
#include <QJsonObject>
#include <QJSValue>
#include <QQuickRenderTarget>
#include <QWebEngineScript>
#include <unistd.h>

static QSize scaled(QSize size, double scale) {
    return {std::max(1, int(size.width() * scale + 0.5)), std::max(1, int(size.height() * scale + 0.5))};
}

Page::Page(uint32_t id, QSize size, double scale) : id(id), logical(size), scale(scale) {
    World &w = world();
    w.context->makeCurrent(w.surface);
    control = new QQuickRenderControl;
    window = new QQuickWindow(control);
    window->setGraphicsDevice(QQuickGraphicsDevice::fromOpenGLContext(w.context));
    QVariantMap initial{{"viewId", id}};
    root = w.component->createWithInitialProperties(initial);
    item = qobject_cast<QQuickItem *>(root);
    if (!item) return;
    item->setParentItem(window->contentItem());
    item->setSize(logical);
    window->setGeometry(0, 0, logical.width(), logical.height());
    control->initialize();
    timer.setSingleShot(true);
    timer.setTimerType(Qt::PreciseTimer);
    connect(&timer, &QTimer::timeout, this, &Page::render);
    connect(control, &QQuickRenderControl::renderRequested, this, &Page::changed);
    connect(control, &QQuickRenderControl::sceneChanged, this, &Page::changed);
    pixels = scaled(logical, scale);
    schedule();
}

Page::~Page() {
    World &w = world();
    w.context->makeCurrent(w.surface);
    delete root;
    delete window;
    delete control;
    drop();
}

QObject *Page::web() const { return root ? root->property("web").value<QObject *>() : nullptr; }

void Page::call(const char *method, const QVariant &argument) {
    if (argument.isValid())
        QMetaObject::invokeMethod(root, method, Q_ARG(QVariant, argument));
    else
        QMetaObject::invokeMethod(root, method);
}

void Page::changed() {
    dirty = true;
    schedule();
}

void Page::schedule() {
    if (!visible || !dirty || timer.isActive() || pending) return;
    const qint64 period = 1000 / std::max<uint32_t>(fps, 1);
    const qint64 gone = since.isValid() ? since.elapsed() : period;
    timer.start(int(std::max<qint64>(0, period - gone)));
}

void Page::setFps(uint32_t value) { fps = std::clamp<uint32_t>(value, 1, 240); }

void Page::show(bool on) {
    if (visible == on) return;
    visible = on;
    root->setProperty("shown", on);
    if (on) changed();
}

void Page::drop() {
    World &w = world();
    if (w.context->currentContext() != w.context) w.context->makeCurrent(w.surface);
    if (fbo) glDeleteFramebuffers(1, &fbo);
    if (source) glDeleteFramebuffers(1, &source);
    if (scratch) glDeleteTextures(1, &scratch);
    fbo = source = scratch = 0;
    for (int i = 0; i < kTextures; i++) {
        if (textures[i]) glDeleteTextures(1, &textures[i]);
        textures[i] = 0;
        known[i] = held[i] = false;
    }
}

void Page::resize(QSize size, double next) {
    logical = size;
    scale = next;
    item->setSize(size);
    window->setGeometry(0, 0, size.width(), size.height());
    pixels = scaled(size, next);
    drop();
    world().host->forget(id);
    pending = false;
    changed();
}

void Page::build() {
    if (textures[0]) return;
    glGenTextures(kTextures, textures);
    for (int i = 0; i < kTextures; i++) {
        glBindTexture(GL_TEXTURE_2D, textures[i]);
        glTexStorage2D(GL_TEXTURE_2D, 1, kFormat, pixels.width(), pixels.height());
    }
    glGenFramebuffers(1, &fbo);
    glGenFramebuffers(1, &source);
    glGenTextures(1, &scratch);
    glBindTexture(GL_TEXTURE_2D, scratch);
    glTexStorage2D(GL_TEXTURE_2D, 1, GL_RGBA8, pixels.width(), pixels.height());
    glBindFramebuffer(GL_FRAMEBUFFER, source);
    glFramebufferTexture2D(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, scratch, 0);
    glBindFramebuffer(GL_FRAMEBUFFER, 0);
}

void Page::convert(int index) {
    glBindFramebuffer(GL_DRAW_FRAMEBUFFER, fbo);
    glFramebufferTexture2D(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, textures[index], 0);
    glBindFramebuffer(GL_READ_FRAMEBUFFER, source);
    glBlitFramebuffer(0, 0, pixels.width(), pixels.height(), 0, 0, pixels.width(), pixels.height(),
                      GL_COLOR_BUFFER_BIT, GL_NEAREST);
    glBindFramebuffer(GL_FRAMEBUFFER, 0);
}

bool Page::announce(int index) {
    Exported out;
    if (!dmabufExport(world().context, textures[index], out) || out.planes != 1) return false;
    const auto &mods = world().modifiers;
    if (std::find(mods.begin(), mods.end(), out.modifier) == mods.end()) {
        close(out.fd);
        return false;
    }
    world().host->buffer(id, index + 1, pixels.width(), pixels.height(), out.fourcc, out.modifier, out.offset,
                         out.stride, out.fd);
    known[index] = true;
    return true;
}

void Page::render() {
    World &w = world();
    if (!visible || !dirty) return;
    const bool share = !w.copy && !w.modifiers.empty();
    int index = 0;
    if (share) {
        index = -1;
        for (int i = 0; i < kTextures && index < 0; i++)
            if (!held[i]) index = i;
        if (index < 0) {
            pending = true;
            return;
        }
    }
    w.context->makeCurrent(w.surface);
    build();
    dirty = false;
    since.start();
    control->polishItems();
    QQuickRenderTarget target = QQuickRenderTarget::fromOpenGLTexture(scratch, GL_RGBA8, pixels);
    target.setDevicePixelRatio(scale);
    target.setMirrorVertically(true);
    window->setRenderTarget(target);
    control->beginFrame();
    control->sync();
    control->render();
    control->endFrame();
    convert(index);
    glFinish();
    latest = index;
    if (share && (known[index] || announce(index))) {
        held[index] = true;
        w.host->frame(id, index + 1);
    } else {
        if (share) w.copy = true;
        readback(index);
    }
    schedule();
}

void Page::readback(int index) {
    World &w = world();
    const uint32_t stride = pixels.width() * 4;
    uint32_t slot = 0;
    uint8_t *into = w.host->slot(id, pixels.width(), pixels.height(), stride, &slot);
    if (!into) {
        dirty = true;
        return;
    }
    glBindFramebuffer(GL_FRAMEBUFFER, fbo);
    glFramebufferTexture2D(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, textures[index], 0);
    glReadPixels(0, 0, pixels.width(), pixels.height(), kReadFormat, GL_UNSIGNED_BYTE, into);
    glBindFramebuffer(GL_FRAMEBUFFER, 0);
    w.host->pixels(id, slot, pixels.width(), pixels.height(), stride);
}

void Page::released(uint32_t buffer) {
    if (buffer >= 1 && buffer <= kTextures) held[buffer - 1] = false;
    pending = false;
    schedule();
}

void Page::shot() {
    if (!textures[0]) return;
    world().context->makeCurrent(world().surface);
    readback(latest);
}

void Page::install(const QJsonArray &list) {
    QObject *scripts = web() ? web()->property("userScripts").value<QObject *>() : nullptr;
    if (!scripts) return;
    QList<QWebEngineScript> items;
    for (const QJsonValue &entry : list) {
        QWebEngineScript script;
        script.setSourceCode(entry["source"].toString());
        script.setInjectionPoint(QWebEngineScript::DocumentCreation);
        script.setWorldId(QWebEngineScript::MainWorld);
        script.setRunsOnSubFrames(entry["frames"].toBool());
        items.append(script);
    }
    QWebEngineScript cap;
    cap.setSourceCode(capScript(fps, world().bridge->key()));
    cap.setInjectionPoint(QWebEngineScript::DocumentCreation);
    cap.setWorldId(QWebEngineScript::MainWorld);
    items.append(cap);
    QMetaObject::invokeMethod(scripts, "clear");
    QMetaObject::invokeMethod(scripts, "insert", Q_ARG(QList<QWebEngineScript>, items));
}
