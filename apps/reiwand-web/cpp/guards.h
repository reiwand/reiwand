#pragma once
#include "host.h"
#include <QByteArray>
#include <QStringList>
#include <QWebEngineUrlRequestInterceptor>
#include <QWebEngineUrlSchemeHandler>

class Interceptor : public QWebEngineUrlRequestInterceptor {
public:
    Interceptor(QStringList origins) : origins(std::move(origins)) {}
    void interceptRequest(QWebEngineUrlRequestInfo &info) override;

private:
    QStringList origins;
};

class Scheme : public QWebEngineUrlSchemeHandler {
public:
    Scheme(const RwHost *host, QByteArray policy) : host(host), policy(std::move(policy)) {}
    void requestStarted(QWebEngineUrlRequestJob *job) override;

private:
    const RwHost *host;
    QByteArray policy;
};

void registerSchemes();
