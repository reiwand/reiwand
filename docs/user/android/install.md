# Installing on Android

Reiwand for Android is an APK on the same *Releases* page as the desktop packages. It needs
Android 12 or later (64-bit).

## Choose the APK

There is one APK per architecture, plus one with both:

| File | For | Size |
| --- | --- | --- |
| `reiwand-android-arm64-v8a.apk` | phones | about 22 MB |
| `reiwand-android-x86_64.apk` | emulators and some tablets | about 23 MB |
| `reiwand-android.apk` | both architectures, if you do not know which one is yours | about 39 MB |

Before, a single 83 MB APK carried both architectures, ONNX Runtime inside and the Kotlin code
unshrunk.

All APKs are always signed with the same key and carry the same version. So each version installs
over the previous one, also if you switch from one APK to another, and keeps the chosen wallpaper.

## Install it

- **On the phone:** open the APK. Android asks you to allow installation from that app.
- **From a computer:**

```sh
adb install -r reiwand-android-arm64-v8a.apk
```

## Next

To set a wallpaper, open the **reiwand** app: see [Library](library.md).
