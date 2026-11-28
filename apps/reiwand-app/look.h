#pragma once

#include <QtCore/QDir>
#include <QtCore/QStringList>
#include <QtGui/QColor>
#include <QtGui/QFont>
#include <QtGui/QFontDatabase>
#include <QtGui/QFontInfo>
#include <QtGui/QGuiApplication>
#include <QtGui/QIcon>
#include <QtGui/QPalette>
#include <QtQuickControls2/QQuickStyle>

inline void installLook(const QString& style, const QString& theme)
{
    for (auto file : { ":/reiwand/fonts/Onest.ttf", ":/reiwand/fonts/RedHatDisplay.ttf" })
        QFontDatabase::addApplicationFont(QString::fromLatin1(file));
    QIcon::setThemeSearchPaths({ QStringLiteral(":/icons") });
    QIcon::setFallbackSearchPaths({});
    QIcon::setFallbackThemeName(theme);
    QIcon::setThemeName(theme);
    QGuiApplication::setWindowIcon(QIcon(QStringLiteral(":/reiwand/pictures/icon.svg")));
    QQuickStyle::setStyle(style);
}

inline void dressLook(const QString& family, int pixels)
{
    QFont font(family);
    font.setPixelSize(pixels);
    QGuiApplication::setFont(font);
}

inline void paintLook(const QString& joined)
{
    const auto tones = joined.split(QLatin1Char(','));
    if (tones.size() < 7)
        return;
    const QColor night(tones[0]), high(tones[1]), line(tones[2]), frost(tones[3]), mist(tones[4]), low(tones[5]), moon(tones[6]);
    QPalette palette;
    const auto set = [&palette](QPalette::ColorGroup group, std::initializer_list<std::pair<QPalette::ColorRole, QColor>> roles) {
        for (const auto& [role, color] : roles)
            palette.setColor(group, role, color);
    };
    set(QPalette::All,
        { { QPalette::Window, night }, { QPalette::WindowText, frost }, { QPalette::Base, high },
            { QPalette::AlternateBase, night }, { QPalette::Text, frost }, { QPalette::Button, high },
            { QPalette::ButtonText, frost }, { QPalette::Highlight, moon }, { QPalette::HighlightedText, night },
            { QPalette::BrightText, night }, { QPalette::ToolTipBase, high }, { QPalette::ToolTipText, frost },
            { QPalette::PlaceholderText, mist }, { QPalette::Link, moon }, { QPalette::LinkVisited, mist },
            { QPalette::Light, line }, { QPalette::Midlight, high }, { QPalette::Mid, line },
            { QPalette::Dark, mist }, { QPalette::Shadow, night } });
#if QT_VERSION >= QT_VERSION_CHECK(6, 6, 0)
    palette.setColor(QPalette::All, QPalette::Accent, moon);
#endif
    set(QPalette::Disabled,
        { { QPalette::WindowText, low }, { QPalette::Text, low }, { QPalette::ButtonText, low },
            { QPalette::Highlight, line }, { QPalette::HighlightedText, mist }, { QPalette::Light, high },
            { QPalette::Dark, line } });
    QGuiApplication::setPalette(palette);
}

inline QString lookStyle()
{
    return QQuickStyle::name();
}

inline QString lookFamily()
{
    return QFontInfo(QGuiApplication::font()).family();
}

inline bool lookHasFamily(const QString& family)
{
    return QFontDatabase::families().contains(family);
}

inline bool lookHasIcon(const QString& name)
{
    return QIcon::hasThemeIcon(name);
}

inline QString lookStyleFiles()
{
    return QDir(QStringLiteral(":/qt/qml/io/github/reiwand/style")).entryList({ QStringLiteral("*.qml") }, QDir::Files, QDir::Name).join(QLatin1Char(','));
}
