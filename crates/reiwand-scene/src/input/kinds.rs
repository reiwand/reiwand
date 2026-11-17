use super::{AudioFrame, BANDS};
use crate::signal::FLOOR_DB;
use crate::value::{AudioChannel, AudioKind, AudioUnit, Level};

const SURGE_FLOOR: f32 = 1e-6;
const SURGE_DB_STEP: f32 = 20.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Extra {
    pub sides: [[f32; BANDS]; 3],
    pub named: [f32; 4],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Usual {
    pub then: [[f32; BANDS]; 2],
    pub now: [[f32; BANDS]; 2],
    pub total: [f32; 2],
    pub all: [f32; 2],
}

pub type UsualKey = (u32, bool, bool);

impl Usual {
    pub fn at(frame: &AudioFrame) -> Usual {
        let total = frame.total_power();
        let all = frame.power.iter().flatten().sum::<f32>() / 2.0;
        Usual { then: frame.power, now: frame.power, total: [total; 2], all: [all; 2] }
    }

    pub fn advance(&mut self, frame: &AudioFrame, dt: f32, (window, _, hold): UsualKey) {
        let window = f32::from_bits(window);
        if hold && frame.silent {
            self.then = self.now;
            self.total[0] = self.total[1];
            self.all[0] = self.all[1];
            return;
        }
        let k = 1.0 - (-dt / window.max(1e-3)).exp();
        self.then = self.now;
        self.total[0] = self.total[1];
        for (c, now) in self.now.iter_mut().enumerate() {
            for (b, v) in now.iter_mut().enumerate() {
                *v += (frame.power[c][b] - *v) * k;
            }
        }
        self.total[1] += (frame.total_power() - self.total[1]) * k;
        self.all[0] = self.all[1];
        self.all[1] += (frame.power.iter().flatten().sum::<f32>() / 2.0 - self.all[1]) * k;
    }
}

impl Default for Extra {
    fn default() -> Self {
        Extra { sides: [[0.0; BANDS]; 3], named: [0.0; 4] }
    }
}

fn amplitude(power: f32) -> f32 {
    power.max(0.0).sqrt().min(1.0)
}

fn reference_amplitude(raw: f32) -> f32 {
    let floor = 10f32.powf(FLOOR_DB / 20.0);
    (10f32.powf((raw - 1.0) * -FLOOR_DB / 20.0) - floor).max(0.0) / (1.0 - floor)
}

pub fn usual_key(b: &crate::Binding<impl Sized>) -> UsualKey {
    (
        b.window.unwrap_or(1.0).to_bits(),
        b.reference == Some(crate::value::Reference::All),
        b.hold_silence == Some(true),
    )
}

pub fn surge(current: f32, usual: f32, unit: AudioUnit) -> f32 {
    let ratio = current.max(SURGE_FLOOR) / usual.max(SURGE_FLOOR);
    match unit {
        AudioUnit::Amplitude => ratio.sqrt(),
        AudioUnit::Db => (1.0 + 10.0 * ratio.log10() / SURGE_DB_STEP).max(0.0),
    }
}

impl AudioFrame {
    pub fn total(&self) -> [f32; 3] {
        let power = self.total_power();
        [amplitude(power), self.loudness, amplitude(power)]
    }

    pub fn total_power(&self) -> f32 {
        10f32.powf((self.loudness - 1.0) * -FLOOR_DB / 10.0)
    }

    pub fn sides(&self) -> [[[f32; BANDS]; 2]; 3] {
        [self.raw.map(|c| c.map(reference_amplitude)), self.loud, self.power.map(|c| c.map(amplitude))]
    }

    pub fn mean_power(&self) -> [f32; BANDS] {
        std::array::from_fn(|b| (self.power[0][b] + self.power[1][b]) / 2.0)
    }

    pub fn side(&self, kind: AudioKind, unit: AudioUnit, channel: AudioChannel) -> &[f32; BANDS] {
        let at = match channel {
            AudioChannel::Left => 0,
            AudioChannel::Right => 1,
            AudioChannel::Average => 2,
        };
        match self.table(kind, unit) {
            None => [&self.left, &self.right, &self.average][at],
            Some(i) => &self.extra[i].sides[at],
        }
    }

    pub fn named(&self, kind: AudioKind, unit: AudioUnit, level: Level) -> f32 {
        let i = match level {
            Level::Bass => 0,
            Level::Mid => 1,
            Level::Treble => 2,
            Level::Level => 3,
            Level::Beat => return f32::from(self.beat),
        };
        match self.table(kind, unit) {
            None => [self.bass, self.mid, self.treble, self.level][i],
            Some(k) => self.extra[k].named[i],
        }
    }

    fn table(&self, kind: AudioKind, unit: AudioUnit) -> Option<usize> {
        match (kind, unit) {
            (AudioKind::Relative, AudioUnit::Db) | (AudioKind::Surge, _) => None,
            (AudioKind::Relative, AudioUnit::Amplitude) => Some(0),
            (AudioKind::Absolute, AudioUnit::Db) => Some(1),
            (AudioKind::Absolute, AudioUnit::Amplitude) => Some(2),
        }
    }
}
