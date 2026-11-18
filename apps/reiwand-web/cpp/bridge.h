#pragma once
#include "host.h"
#include <QByteArray>
#include <QColor>
#include <QObject>
#include <QString>

class Bridge : public QObject {
    Q_OBJECT
    Q_PROPERTY(QObject *profile MEMBER profile CONSTANT)
    Q_PROPERTY(QString own MEMBER own CONSTANT)
    Q_PROPERTY(QColor background MEMBER background CONSTANT)
    Q_PROPERTY(bool muted MEMBER muted NOTIFY mutedChanged)

public:
    Bridge(const RwHost *host, QObject *profile, QString own, QByteArray prefix, QColor background)
        : host(host), profile(profile), own(std::move(own)), prefix(std::move(prefix)), background(background) {}
    void mute(bool on);

    Q_INVOKABLE void loaded(int view);
    Q_INVOKABLE void terminated(int view, int status);
    Q_INVOKABLE void console(int view, const QString &message);
    Q_INVOKABLE void answered(int view);

signals:
    void mutedChanged();

private:
    const RwHost *host;
    QObject *profile;
    QString own;
    QByteArray prefix;
    QColor background;
    bool muted = true;
};
