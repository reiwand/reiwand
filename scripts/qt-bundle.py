import os
import re
import shutil
import subprocess
import sys

PREFIX, APPDIR = (os.path.realpath(p) for p in sys.argv[1:3])
QT = os.path.join(APPDIR, "qt")
BINARIES = [os.path.join(APPDIR, "bin", name) for name in ("reiwand-app", "reiwand-web")]
PLUGINS = ["iconengines", "imageformats", "networkinformation", "platforminputcontexts", "platforms",
           "platformthemes", "tls", "wayland-decoration-client", "wayland-graphics-integration-client",
           "wayland-shell-integration", "xcbglintegrations"]
TREES = [f"lib/qt6/plugins/{p}" for p in PLUGINS] + [
    "lib/qt6/qml", "share/qt6/resources", "share/qt6/translations/qtwebengine_locales", "share/licenses"]
LEFT_OUT = ["lib/qt6/qml/Qt/test", "lib/qt6/qml/org/kde/pipewire/record"]
BUILD_FILES = shutil.ignore_patterns("*.o", "*.a", "*.prl", "objects-*")
HOST = re.compile(r"^(ld-linux.*|lib(c|m|dl|rt|pthread|resolv|util|GL|EGL|GLX|OpenGL|GLdispatch|GLESv2|gbm|drm|"
                  r"vulkan|asound|fontconfig|dbus-1|va|va-drm|epoxy|pipewire-0\.3)\.so(\..*)?)$")
CONF = """[Paths]
Prefix={prefix}
Libraries=lib
Plugins=lib/qt6/plugins
QmlImports=lib/qt6/qml
ArchData=lib/qt6
LibraryExecutables=lib/qt6
Data=share/qt6
Translations=share/qt6/translations
"""


def elf(path):
    with open(path, "rb") as f:
        head = f.read(18)
    return head[:4] == b"\x7fELF" and head[16] in (2, 3)


def needed(path):
    env = dict(os.environ, LD_LIBRARY_PATH=os.path.join(PREFIX, "lib"))
    out = subprocess.run(["ldd", path], env=env, capture_output=True, text=True, check=True).stdout
    for line in out.splitlines():
        parts = line.split()
        if len(parts) >= 3 and parts[1] == "=>":
            if parts[2] == "not" and not HOST.match(parts[0]):
                sys.exit(f"{path} needs {parts[0]}, which neither the Qt prefix nor this system has")
            yield parts[0], parts[2]


def rpath(path, target):
    rel = os.path.relpath(target, os.path.dirname(path))
    value = "$ORIGIN" if rel == "." else f"$ORIGIN/{rel}"
    subprocess.run(["patchelf", "--force-rpath", "--set-rpath", value, path], check=True)


for tree in TREES:
    shutil.copytree(os.path.join(PREFIX, tree), os.path.join(QT, tree), ignore=BUILD_FILES)
for tree in LEFT_OUT:
    shutil.rmtree(os.path.join(QT, tree))
os.makedirs(os.path.join(QT, "lib"), exist_ok=True)
shutil.copy2(os.path.join(PREFIX, "lib/qt6/QtWebEngineProcess"), os.path.join(QT, "lib/qt6"))

files = sorted(os.path.join(d, f) for d, _, fs in os.walk(QT) for f in fs)
elves = BINARIES + [f for f in files if not os.path.islink(f) and elf(f)]
libs = {}
for path in elves:
    for soname, found in needed(path):
        if not HOST.match(soname) and found != "not" and os.path.realpath(found).startswith(PREFIX + os.sep):
            libs[soname] = os.path.realpath(found)
for soname, found in sorted(libs.items()):
    shutil.copyfile(found, os.path.join(QT, "lib", soname))
    os.chmod(os.path.join(QT, "lib", soname), 0o755)
    elves.append(os.path.join(QT, "lib", soname))
for path in elves:
    rpath(path, os.path.join(QT, "lib"))
with open(os.path.join(APPDIR, "bin", "qt.conf"), "w") as f:
    f.write(CONF.format(prefix="../qt"))
with open(os.path.join(QT, "lib/qt6", "qt.conf"), "w") as f:
    f.write(CONF.format(prefix="../.."))
print(f"bundled {len(libs)} libraries from {PREFIX}")
