%global debug_package %{nil}

Name:           reiwand
Version:        %{reiwand_core}
Release:        %{reiwand_release}%{?dist}
Summary:        Animated and static wallpapers for KDE Plasma
License:        GPL-3.0-or-later
URL:            https://github.com/reiwand/reiwand
Source0:        reiwand-%{reiwand_version}-x86_64-linux.tar.gz
ExclusiveArch:  x86_64
BuildRequires:  systemd-rpm-macros
Requires:       pipewire
Requires:       kpipewire
Requires:       kf6-kwindowsystem
Requires:       plasma-workspace
Requires:       gstreamer1 >= 1.24
Requires:       gstreamer1-plugins-base >= 1.24
Requires:       gstreamer1-plugins-good >= 1.24
Requires:       pipewire-gstreamer
Requires:       kf6-kirigami
Recommends:     wpewebkit >= 2.54
Recommends:     %{name}-examples

%description
Images and looping videos as wallpapers, with transitions, one wallpaper per screen and
automatic pausing. A daemon renders each screen on the GPU and KDE Plasma shows it through a
wallpaper plugin; the Reiwand app, reiwand-ctl and the system tray icon manage it.

%package examples
Summary:        Example scenes for Reiwand
License:        CC0-1.0
BuildArch:      noarch
Requires:       %{name} = %{version}-%{release}

%description examples
Small scenes made for Reiwand and dedicated to the public domain: layers and blend modes,
particles, effects, a scene that follows the sound, one that follows the pointer and a scripted
one. They appear in the library, can be removed from it, and leave it with this package.

%prep
%autosetup -n reiwand-%{reiwand_version}-x86_64-linux

%install
install -Dm755 -t %{buildroot}%{_bindir} bin/reiwandd bin/reiwand-ctl bin/reiwand-app bin/reiwand-web bin/reiwand-upgrade
mkdir -p %{buildroot}%{_userunitdir} %{buildroot}%{_datadir}/dbus-1/services %{buildroot}%{_datadir}/plasma/wallpapers %{buildroot}%{_datadir}/plasma/plasmoids %{buildroot}%{_datadir}/applications
sed 's|@BINDIR@|%{_bindir}|g' packaging/systemd/reiwandd.service > %{buildroot}%{_userunitdir}/reiwandd.service
sed 's|@BINDIR@|%{_bindir}|g' packaging/dbus/io.github.reiwand.Daemon.service > %{buildroot}%{_datadir}/dbus-1/services/io.github.reiwand.Daemon.service
sed 's|@BINDIR@|%{_bindir}|g' packaging/dbus/io.github.reiwand.service > %{buildroot}%{_datadir}/dbus-1/services/io.github.reiwand.service
sed 's|@BINDIR@|%{_bindir}|g' packaging/desktop/io.github.reiwand.desktop > %{buildroot}%{_datadir}/applications/io.github.reiwand.desktop
install -Dm644 packaging/mime/io.github.reiwand.xml %{buildroot}%{_datadir}/mime/packages/io.github.reiwand.xml
install -Dm644 packaging/icons/io.github.reiwand.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/io.github.reiwand.svg
for name in io.github.reiwand-symbolic io.github.reiwand-paused-symbolic; do
  install -Dm644 packaging/icons/$name.svg %{buildroot}%{_datadir}/icons/hicolor/symbolic/apps/$name.svg
done
for png in packaging/icons/hicolor/*/apps/io.github.reiwand.png; do
  size=$(basename "$(dirname "$(dirname "$png")")")
  install -Dm644 "$png" "%{buildroot}%{_datadir}/icons/hicolor/$size/apps/io.github.reiwand.png"
done
cp -r plasma-wallpaper %{buildroot}%{_datadir}/plasma/wallpapers/io.github.reiwand
cp -r plasma-tray %{buildroot}%{_datadir}/plasma/plasmoids/io.github.reiwand.tray
mkdir -p %{buildroot}%{_datadir}/reiwand
cp -r kwin-pointer %{buildroot}%{_datadir}/reiwand/kwin-pointer
install -Dm644 -t %{buildroot}%{_datadir}/reiwand/examples examples/*.wpak

%files
%license LICENSE THIRD-PARTY-NOTICES
%doc README.md
%{_bindir}/reiwandd
%{_bindir}/reiwand-ctl
%{_bindir}/reiwand-upgrade
%{_bindir}/reiwand-app
%{_bindir}/reiwand-web
%{_userunitdir}/reiwandd.service
%{_datadir}/dbus-1/services/io.github.reiwand.Daemon.service
%{_datadir}/dbus-1/services/io.github.reiwand.service
%{_datadir}/applications/io.github.reiwand.desktop
%{_datadir}/mime/packages/io.github.reiwand.xml
%{_datadir}/icons/hicolor/scalable/apps/io.github.reiwand.svg
%{_datadir}/icons/hicolor/symbolic/apps/io.github.reiwand-*symbolic.svg
%{_datadir}/icons/hicolor/*/apps/io.github.reiwand.png
%{_datadir}/plasma/wallpapers/io.github.reiwand
%{_datadir}/plasma/plasmoids/io.github.reiwand.tray
%dir %{_datadir}/reiwand
%{_datadir}/reiwand/kwin-pointer

%files examples
%license examples/LICENSE
%{_datadir}/reiwand/examples
