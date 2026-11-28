# Media, sound and touch

Some scenes and web pages react to what plays on the phone, to sound and to touch, and scenes can
play their own sound. Each of these is a permission turned on package by package. This page covers
what each one reads and when.

## Show what plays

*Show what plays* appears in the scenes and pages that declare `media`. It starts off.

Android only lets you see the players of other apps with notification access. When you turn it on,
the app explains what it uses it for and opens that setting.

With both permissions, the phone:

- follows the active media session through its notices, without polling it;
- does so only while a wallpaper that uses it is seen;
- gives it the same as the computer: texts, cover reduced to 256 pixels, colors, position, `media:`
  triggers and script events.

> **Note:** It never reads the notifications nor keeps, logs or sends what plays.

Below the switch it says whether it is being read now, how many players it found and whether there
is a track, whether it is playing and whether it has a cover, never which.

A page that can also reach the internet asks for a second permission, with its own dialog when
turning it on.

## React to sound, touch and taps

*React to sound*, *React to touch* and *React to taps* only appear in the scenes and web pages that
use them. They are turned on package by package and start off.

### Sound

Sound is read with Android's visualizer, which gives a low-resolution spectrum of what plays on
the phone.

- Android asks for the microphone permission the first time it is turned on, and shows its
  indicator while it listens.
- It only listens while a scene with that permission is seen, and only computes bands in memory.
- After two seconds of silence it stops reading until something starts to play. If a player stays
  active in silence, it looks every two seconds.

### Touch and taps

| Input | What it is |
| --- | --- |
| Touch | The position of the finger on the wallpaper on the home screen, which the scene follows like the pointer. |
| Taps | The phone's clicks. A tap on a `solid` layer starts its `click:` timelines and reaches its scripts. |

Only real taps count: on the home screen (the lock screen gives none) and on the preview on the
scene's page, and only in scenes with that permission. Without it, tapping the preview does not
reach the scene.

## Scene sound

The sound of scenes plays on the phone with the same rules as on the desktop:

| Setting | Effect |
| --- | --- |
| *Play the sound of videos and scenes* in *Settings* (`playback.muted`) | Scenes start silent until this is on. |
| `playback.scene_sound` set to `false` | Leaves scenes mute. |
| `playback.volume` | The volume is `playback.volume` times the scene's *Volume* setting (which appears in scenes with sound). |
| *Silence the wallpaper while another app plays sound* | Lowers it with the same fade. |

A sound started by an event or a script respects the silence and volume chosen in Settings just
the same.

### When it sounds

It only sounds on the home screen. It does not sound:

- on the lock screen or in the preview;
- when the wallpaper is not seen;
- with power saving, with low battery without a charger or with high heat (it goes quiet, like the
  scene's movement).

### How it plays

- All the layers of a scene are mixed into a single AAudio output. It is opened when something
  sounds and closed when everything has gone quiet and five seconds have passed.
- The sounds are decoded with Android's codecs the first time they are heard (long ones with
  `stream`, as they play).
- They are realigned with the scene once per second.
- They reach the speaker about 200 ms after the scene, so sounds started by a tap come somewhat
  later than on the desktop.

> **Note:** There is no network or new permissions. The sound output does not ask for the
> microphone (that is only *React to sound*), and what plays in a wallpaper does not leave the
> phone.
