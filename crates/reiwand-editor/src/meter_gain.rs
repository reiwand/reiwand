use reiwand_scene::{AudioFrame, BANDS, FLOOR_DB, MIN_PEAK, Usual, UsualKey, db01, follow_peak};
use std::sync::Mutex;
use std::time::Instant;

static REFERENCE: Mutex<(f32, Option<Instant>)> = Mutex::new((MIN_PEAK, None));

static USUAL: Mutex<Option<(UsualKey, Usual, Instant)>> = Mutex::new(None);

pub(crate) fn reset() {
    *REFERENCE.lock().unwrap_or_else(|e| e.into_inner()) = (MIN_PEAK, None);
    *USUAL.lock().unwrap_or_else(|e| e.into_inner()) = None;
}

pub(crate) fn usual(frame: &AudioFrame, key: UsualKey) -> Usual {
    let mut guard = USUAL.lock().unwrap_or_else(|e| e.into_inner());
    let now = Instant::now();
    let (_, mut usual, at) = guard.take().filter(|(k, ..)| *k == key).unwrap_or((key, Usual::at(frame), now));
    usual.advance(frame, (now - at).as_secs_f32().min(1.0), key);
    *guard = Some((key, usual, now));
    usual
}

pub(crate) fn follow(loud: &[[f32; BANDS]; 2], release: f32) -> [[f32; BANDS]; 2] {
    let mut guard = REFERENCE.lock().unwrap_or_else(|e| e.into_inner());
    let now = Instant::now();
    let dt = guard.1.replace(now).map_or(0.0, |at| (now - at).as_secs_f32().min(1.0));
    referenced(loud, release, &mut guard.0, dt)
}

pub(crate) fn referenced(
    loud: &[[f32; BANDS]; 2],
    release: f32,
    peak: &mut f32,
    dt: f32,
) -> [[f32; BANDS]; 2] {
    let power = loud.map(|c| c.map(|l| 10f32.powf((l * -FLOOR_DB + FLOOR_DB) / 10.0)));
    let loudest = power.iter().flatten().fold(0.0, |a: f32, &b| a.max(b));
    *peak = follow_peak(*peak, loudest, dt, release);
    power.map(|c| c.map(|p| db01(p / *peak, FLOOR_DB)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn level(db: f32) -> f32 {
        (db - FLOOR_DB) / -FLOOR_DB
    }

    #[test]
    fn a_short_release_lets_the_bands_dip_after_a_hit_and_recover() {
        let run = |release: f32| {
            let mut peak = MIN_PEAK;
            let mut seen = Vec::new();
            for tick in 0..100 {
                let db = if tick == 0 { 0.0 } else { -27.0 };
                let out = referenced(&[[level(db); BANDS]; 2], release, &mut peak, 0.01);
                seen.push(out[0][0]);
            }
            seen
        };
        let (quick, slow) = (run(0.4), run(5.0));
        assert!(quick[0] > 0.99 && quick[2] < 0.7 * quick[0], "{}", quick[2]);
        assert!(quick[99] > quick[2] + 0.04, "{} {}", quick[2], quick[99]);
        assert!((slow[2] - slow[99]).abs() < 0.03, "{} {}", slow[2], slow[99]);
    }
}
