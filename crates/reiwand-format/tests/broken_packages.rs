mod mutate;
mod mutated;

use mutate::{EXAMPLES, Rng, capped, example, files};
use mutated::{opened, sizes, zipped};

type Files = Vec<(String, Vec<u8>)>;

fn packages() -> Vec<(&'static str, Files)> {
    EXAMPLES.iter().map(|name| (*name, files(&example(name)))).collect()
}

fn survives(what: String, bytes: Vec<u8>) -> Result<usize, String> {
    capped(what, move || opened(bytes))
}

#[test]
fn every_example_reads_as_a_package_before_it_is_broken() {
    for (name, files) in packages() {
        survives(name.into(), zipped(&files)).unwrap_or_else(|e| panic!("{name}: {e}"));
    }
}

#[test]
fn a_package_cut_short_anywhere_is_refused() {
    for (name, files) in packages() {
        let whole = zipped(&files);
        let tail = whole.len().saturating_sub(300)..whole.len();
        let cuts = (0..whole.len()).step_by(97).chain(tail);
        for cut in cuts {
            let cut_short = survives(format!("{name} cut at {cut}"), whole[..cut].to_vec());
            assert!(cut_short.is_err(), "{name} cut at {cut} of {} was accepted", whole.len());
        }
    }
}

#[test]
fn wrong_sizes_in_the_entry_list_are_refused_or_read_back_whole() {
    for (name, files) in packages() {
        let whole = zipped(&files);
        for value in [0, 1, 0x7fff_ffff, u32::MAX] {
            for (index, broken) in sizes(&whole, value).into_iter().enumerate() {
                let _ = survives(format!("{name} size {value:#x} at field {index}"), broken);
            }
        }
    }
}

#[test]
fn random_damage_to_a_package_never_panics_or_hangs() {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    for (name, files) in packages() {
        let whole = zipped(&files);
        for round in 0..250 {
            let damaged = mutate::bytes(&mut rng, &whole);
            let _ = survives(format!("{name} damage {round}"), damaged);
        }
    }
}

#[test]
fn entries_that_are_not_what_the_list_says_are_refused_or_survived() {
    let mut rng = Rng(0x2545_f491_4f6c_dd1d);
    for (name, files) in packages() {
        for index in 0..files.len() {
            let other = rng.below(files.len());
            let half = files[index].1.len() / 2;
            let swaps = [files[other].1.clone(), files[index].1[..half].to_vec(), Vec::new(), vec![0xff; 64]];
            for (kind, data) in swaps.into_iter().enumerate() {
                let mut changed = files.clone();
                changed[index].1 = data;
                let what = format!("{name}: {} replaced ({kind})", files[index].0);
                let _ = survives(what, zipped(&changed));
            }
            let mut renamed = files.clone();
            renamed[index].0 = ["../x", "/abs", "a\\b", "", "dir/", "a//b"][rng.below(6)].into();
            if let Some(bytes) = written(&renamed) {
                let _ = survives(format!("{name}: {} renamed", files[index].0), bytes);
            }
            let mut twice = files.clone();
            twice.push(files[index].clone());
            if let Some(bytes) = written(&twice) {
                let _ = survives(format!("{name}: {} twice", files[index].0), bytes);
            }
        }
    }
}

fn written(files: &[(String, Vec<u8>)]) -> Option<Vec<u8>> {
    std::panic::catch_unwind(|| zipped(files)).ok()
}
