use super::{AudioFrame, BANDS, Usual, surge};
use crate::Reduce;
use crate::signal::levels;
use crate::value::{AudioChannel, AudioKind, AudioUnit, Band, Binding, Level, Reference};

fn fold(bands: &[f32; BANDS], of: usize, reduce: Reduce, lo: u32, hi: u32) -> f32 {
    let a = (lo as usize * BANDS / of).min(BANDS - 1);
    let z = ((hi.max(lo) as usize + 1) * BANDS / of).clamp(a + 1, BANDS);
    let it = bands[a..z].iter().copied();
    match reduce {
        Reduce::Max => it.fold(0.0, f32::max),
        Reduce::Mean => it.sum::<f32>() / (z - a) as f32,
    }
}

fn pick(both: &[[f32; BANDS]; 2], channel: AudioChannel) -> [f32; BANDS] {
    match channel {
        AudioChannel::Left => both[0],
        AudioChannel::Right => both[1],
        AudioChannel::Average => std::array::from_fn(|b| (both[0][b] + both[1][b]) / 2.0),
    }
}

pub fn read(a: &AudioFrame, usual: Option<&Usual>, b: &Binding<impl Sized>, band: Band) -> f32 {
    let channel = b.channel.unwrap_or_default();
    let (kind, unit) = (b.kind.unwrap_or(AudioKind::Relative), b.unit.unwrap_or(AudioUnit::Db));
    let of = b.of.unwrap_or(BANDS as u32).clamp(1, BANDS as u32) as usize;
    let reduce = b.reduce.unwrap_or_default();
    let span = |band: Band| match band {
        Band::Index(i) => Some((i, i)),
        Band::Range([lo, hi]) => Some((lo, hi)),
        Band::Named(_) => None,
    };
    let Some(usual) = usual.filter(|_| kind == AudioKind::Surge) else {
        return match (span(band), band) {
            (Some((lo, hi)), _) => fold(a.side(kind, unit, channel), of, reduce, lo, hi),
            (None, Band::Named(level)) => a.named(kind, unit, level),
            _ => 0.0,
        };
    };
    let (now, then) = (pick(&a.power, channel), pick(&usual.then, channel));
    if b.reference == Some(Reference::All) && !matches!(band, Band::Named(Level::Level | Level::Beat)) {
        let sel = match band {
            Band::Named(named) => {
                let at = match named {
                    Level::Bass => 0,
                    Level::Mid => 1,
                    _ => 2,
                };
                levels(&now)[at]
            }
            _ => {
                let (lo, hi) = span(band).unwrap_or((0, 0));
                fold(&now, of, reduce, lo, hi)
            }
        };
        return surge(sel, usual.all[0], unit);
    }
    match band {
        Band::Named(Level::Level) => surge(a.total_power(), usual.total[0], unit),
        Band::Named(Level::Beat) => f32::from(a.beat),
        Band::Named(named) => {
            let at = match named {
                Level::Bass => 0,
                Level::Mid => 1,
                _ => 2,
            };
            surge(levels(&now)[at], levels(&then)[at], unit)
        }
        band => {
            let (lo, hi) = span(band).unwrap_or((0, 0));
            surge(fold(&now, of, reduce, lo, hi), fold(&then, of, reduce, lo, hi), unit)
        }
    }
}
