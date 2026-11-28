use crate::meter::channel;
use crate::meter_gain;
use reiwand_scene::{
    AudioChannel, AudioFrame, AudioKind, AudioUnit, BANDS, Binding, FLOOR_DB, Hearing, Level, Reference,
    Usual, levels, surge,
};
use serde_json::Value;

pub(crate) fn frame(reading: &Value, release: Option<f32>) -> AudioFrame {
    let loud = [channel(reading, "loud", 0), channel(reading, "loud", 1)];
    let relative = match release {
        Some(release) => meter_gain::follow(&loud, release),
        None => [channel(reading, "raw", 0), channel(reading, "raw", 1)],
    };
    let mut frame = AudioFrame {
        left: relative[0],
        right: relative[1],
        average: std::array::from_fn(|i| (relative[0][i] + relative[1][i]) / 2.0),
        raw: relative,
        loud,
        power: loud.map(|c| c.map(|l| 10f32.powf((l - 1.0) * -FLOOR_DB / 10.0))),
        loudness: reading["loudness"].as_f64().unwrap_or(0.0) as f32,
        beat: reading["beat"] == true,
        silent: reading["silent"] == true,
        ..AudioFrame::default()
    };
    [frame.bass, frame.mid, frame.treble] = levels(&frame.average);
    frame.level = frame.loudness;
    Hearing { attack: 0.001, release: 0.01, gain_release: 5.0 }.kinds(
        &AudioFrame::default(),
        &mut frame,
        1.0,
    );
    frame
}

pub(crate) fn side(
    frame: &AudioFrame,
    usual: Option<&Usual>,
    source: &Binding<f32>,
    kind: AudioKind,
    unit: AudioUnit,
) -> [f32; BANDS] {
    let channel = source.channel.unwrap_or_default();
    let Some(usual) = usual else { return *frame.side(kind, unit, channel) };
    let pick = |both: &[[f32; BANDS]; 2], b: usize| match channel {
        AudioChannel::Left => both[0][b],
        AudioChannel::Right => both[1][b],
        AudioChannel::Average => (both[0][b] + both[1][b]) / 2.0,
    };
    let own = source.reference != Some(Reference::All);
    std::array::from_fn(|b| {
        surge(pick(&frame.power, b), if own { pick(&usual.then, b) } else { usual.all[0] }, unit)
    })
}

pub(crate) fn surged(frame: &AudioFrame, usual: &Usual, level: Level, unit: AudioUnit) -> f32 {
    let mean = |both: &[[f32; BANDS]; 2]| -> [f32; BANDS] {
        std::array::from_fn(|b| (both[0][b] + both[1][b]) / 2.0)
    };
    let at = match level {
        Level::Bass => 0,
        Level::Mid => 1,
        Level::Treble => 2,
        _ => return surge(frame.total_power(), usual.total[0], unit),
    };
    surge(levels(&mean(&frame.power))[at], levels(&mean(&usual.then))[at], unit)
}
