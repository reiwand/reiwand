#include "bridge.h"

void Bridge::mute(bool on) {
    if (muted == on) return;
    muted = on;
    emit mutedChanged();
}

void Bridge::loaded(int view) { host->loaded(view); }

void Bridge::terminated(int view, int status) { host->crashed(view, status); }

void Bridge::answered(int view) { host->answered(view); }

void Bridge::console(int view, const QString &message) {
    QByteArray text = message.toUtf8();
    if (text.startsWith(prefix)) host->message(view, text.constData() + prefix.size());
}
