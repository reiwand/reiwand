#include <QTimer>
#include "guards.h"
#include <QFile>
#include <QMultiMap>
#include <QWebEngineUrlRequestInfo>
#include <QWebEngineUrlRequestJob>
#include <QWebEngineUrlScheme>

static const char *const kept[] = {"wpak", "wpak-user", "data", "blob"};

void registerSchemes() {
    for (const char *name : {"wpak", "wpak-user"}) {
        QWebEngineUrlScheme scheme{QByteArray(name)};
        scheme.setSyntax(QWebEngineUrlScheme::Syntax::Host);
        scheme.setFlags(QWebEngineUrlScheme::SecureScheme | QWebEngineUrlScheme::CorsEnabled |
                        QWebEngineUrlScheme::FetchApiAllowed);
        QWebEngineUrlScheme::registerScheme(scheme);
    }
}

void Interceptor::interceptRequest(QWebEngineUrlRequestInfo &info) {
    const QUrl url = info.requestUrl();
    for (const char *name : kept) {
        if (url.scheme() == QLatin1String(name)) return;
    }
    const QString text = url.toString(QUrl::FullyEncoded);
    for (const QString &origin : origins) {
        if (text.startsWith(origin + QLatin1Char('/'))) return;
    }
    info.block(true);
}

void Scheme::requestStarted(QWebEngineUrlRequestJob *job) {
    if (job->requestMethod() != "GET") return job->fail(QWebEngineUrlRequestJob::RequestDenied);
    const QByteArray uri = job->requestUrl().toString(QUrl::FullyEncoded).toUtf8();
    uint64_t size = 0;
    char mime[128] = {0};
    const int fd = host->serve(uri.constData(), &size, mime, sizeof mime);
    if (fd < 0) return job->redirect(QUrl(QStringLiteral("http://absent.invalid/")));
    auto *file = new QFile(job);
    if (!file->open(fd, QIODevice::ReadOnly, QFile::AutoCloseHandle)) {
        delete file;
        return job->fail(QWebEngineUrlRequestJob::RequestFailed);
    }
    QMultiMap<QByteArray, QByteArray> headers;
    headers.insert("Access-Control-Allow-Origin", "*");
    headers.insert("Content-Security-Policy", policy);
    job->setAdditionalResponseHeaders(headers);
    job->reply(QByteArray(mime), file);
}
