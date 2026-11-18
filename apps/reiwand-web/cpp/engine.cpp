#include "guards.h"
#include "page.h"
#include <QGuiApplication>
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QQmlContext>
#include <QSocketNotifier>
#include <QSurfaceFormat>
#include <QtWebEngineQuick/QQuickWebEngineDownloadRequest>
#include <QtWebEngineQuick/QQuickWebEngineProfile>
#include <QtWebEngineQuick/qtwebenginequickglobal.h>
#include <map>

static World global;
static std::map<uint32_t, std::unique_ptr<Page>> pages;

World &world() { return global; }

Page *pageOf(uint32_t id) {
    auto found = pages.find(id);
    return found == pages.end() ? nullptr : found->second.get();
}

std::vector<Page *> allPages() {
    std::vector<Page *> out;
    for (auto &entry : pages) out.push_back(entry.second.get());
    return out;
}

void openPage(uint32_t id, QSize size, double scale) {
    auto page = std::make_unique<Page>(id, size, scale);
    if (page->ready()) pages[id] = std::move(page);
}

void closePage(uint32_t id) { pages.erase(id); }

bool openContext(World &w, QString &why) {
    QSurfaceFormat format;
    format.setRenderableType(QSurfaceFormat::OpenGLES);
    format.setVersion(3, 0);
    QSurfaceFormat::setDefaultFormat(format);
    w.context = new QOpenGLContext;
    w.context->setFormat(format);
    w.context->setShareContext(QOpenGLContext::globalShareContext());
    if (!w.context->create()) {
        why = "the OpenGL ES context cannot be created";
        return false;
    }
    w.surface = new QOffscreenSurface;
    w.surface->setFormat(w.context->format());
    w.surface->create();
    if (!w.context->makeCurrent(w.surface)) {
        why = "the OpenGL ES context cannot be used";
        return false;
    }
    return true;
}

bool hasDisplay(QString &why) {
    if (qEnvironmentVariableIsSet("QT_QPA_PLATFORM") || qEnvironmentVariableIsSet("WAYLAND_DISPLAY") ||
        qEnvironmentVariableIsSet("DISPLAY"))
        return true;
    why = "there is no graphical session to draw the page in (no Wayland or X display)";
    return false;
}

void prepare() {
    registerSchemes();
    qputenv("QSG_RHI_BACKEND", "opengl");
    qputenv("QT_XCB_GL_INTEGRATION", "xcb_egl");
    QtWebEngineQuick::initialize();
    QCoreApplication::setAttribute(Qt::AA_ShareOpenGLContexts);
}

static QQuickWebEngineProfile *makeProfile(const QJsonObject &c, QObject *parent) {
    const QString storage = c["storage"].toString();
    QQuickWebEngineProfile *profile =
        storage.isEmpty() ? new QQuickWebEngineProfile(parent) : new QQuickWebEngineProfile("page", parent);
    if (!storage.isEmpty()) {
        profile->setPersistentStoragePath(storage + "/data");
        profile->setCachePath(storage + "/cache");
        profile->setPersistentCookiesPolicy(QQuickWebEngineProfile::ForcePersistentCookies);
    }
    QStringList languages;
    for (const QJsonValue &l : c["languages"].toArray()) languages << l.toString();
    profile->setHttpAcceptLanguage(languages.join(','));
    QStringList origins;
    for (const QJsonValue &o : c["network"].toArray()) origins << o.toString();
    profile->setUrlRequestInterceptor(new Interceptor(origins));
    const QByteArray policy = c["policy"].toString().toUtf8();
    for (const char *name : {"wpak", "wpak-user"}) profile->installUrlSchemeHandler(name, new Scheme(global.host, policy));
    QObject::connect(profile, &QQuickWebEngineProfile::downloadRequested,
                     [](QQuickWebEngineDownloadRequest *download) { download->cancel(); });
    return profile;
}

int rw_run(const char *config, const RwHost *host, int socket) {
    const QJsonObject c = QJsonDocument::fromJson(config).object();
    QString why;
    if (!hasDisplay(why)) {
        host->said(why.toUtf8().constData());
        return 2;
    }
    prepare();
    int argc = 1;
    char name[] = "reiwand-web";
    char *argv[] = {name, nullptr};
    QGuiApplication app(argc, argv);
    World &w = global;
    w.host = host;
    if (!openContext(w, why)) {
        host->said(why.toUtf8().constData());
        return 2;
    }
    for (const QJsonValue &m : c["modifiers"].toArray()) w.modifiers.push_back(m.toString().toULongLong());
    const QJsonArray rgb = c["background"].toArray();
    w.background = QColor(rgb[0].toInt(), rgb[1].toInt(), rgb[2].toInt());
    w.copy = c["copy"].toBool();
    auto *profile = makeProfile(c, &app);
    w.engine = new QQmlEngine(&app);
    w.bridge = new Bridge(host, profile, c["own"].toString(), c["prefix"].toString().toUtf8(), w.background);
    w.engine->rootContext()->setContextProperty("bridge", w.bridge);
    w.component = new QQmlComponent(w.engine);
    static const char view[] =
#include "view.inc"
        ;
    w.component->setData(QByteArray(view), QUrl());
    if (w.component->isError()) {
        host->said(w.component->errorString().toUtf8().constData());
        return 2;
    }
    QSocketNotifier notifier(socket, QSocketNotifier::Read);
    QObject::connect(&notifier, &QSocketNotifier::activated, [host] { host->readable(); });
    host->ready(qVersion());
    const int code = app.exec();
    pages.clear();
    QCoreApplication::sendPostedEvents(nullptr, QEvent::DeferredDelete);
    delete w.component;
    delete w.engine;
    delete profile;
    return code;
}

void rw_quit() { QCoreApplication::quit(); }
