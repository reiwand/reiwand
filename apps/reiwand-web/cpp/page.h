#pragma once
#include "dmabuf.h"
#include "world.h"
#include <QElapsedTimer>
#include <QJsonArray>
#include <QPointer>
#include <QQuickItem>
#include <QQuickRenderControl>
#include <QQuickWindow>
#include <QTimer>
#include <memory>

constexpr int kTextures = 3;
constexpr GLenum kFormat = 0x93A1;
constexpr GLenum kReadFormat = 0x80E1;

class Page : public QObject {
public:
    Page(uint32_t id, QSize size, double scale);
    ~Page() override;
    bool ready() const { return item != nullptr && failed.isEmpty(); }
    QString failure() const { return failed; }
    void resize(QSize size, double scale);
    void show(bool on);
    void setFps(uint32_t value);
    void released(uint32_t buffer);
    void shot();
    void hover(QPointF at);
    void leave();
    void install(const QJsonArray &list);
    void call(const char *method, const QVariant &argument);
    QObject *web() const;
    QQuickWindow *surface() const { return window; }
    bool visible = true;

private:
    void changed();
    void polish();
    void schedule();
    void render();
    bool build();
    void drop();
    bool announce(int index);
    void send(int index);
    void readback(int index);
    void convert(int index);

    uint32_t id;
    QSize logical;
    double scale;
    uint32_t fps = 30;
    QQuickRenderControl *control = nullptr;
    QQuickWindow *window = nullptr;
    QQuickItem *item = nullptr;
    QObject *root = nullptr;
    QTimer timer;
    QElapsedTimer since;
    bool dirty = true;
    bool pending = false;
    bool inFrame = false;
    bool polishQueued = false;
    bool first = true;
    bool unsent = false;
    bool broken = false;
    QString failed;
    GLuint textures[kTextures] = {};
    bool known[kTextures] = {};
    bool held[kTextures] = {};
    GLuint fbo = 0;
    GLuint scratch = 0;
    GLuint source = 0;
    QPointer<QQuickItem> input;
    QPointF last;
    int latest = 0;
    QSize pixels;
};

QString capScript(uint32_t fps, const QString &key);
Page *pageOf(uint32_t id);
std::vector<Page *> allPages();
