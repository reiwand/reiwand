//! Requests for covers against a library that answers late, fails and sends covers on its own,
//! while searches replace the rows: in any order, the covers of the rows shown all arrive once the
//! library is healthy again, without the shelf being moved.
use super::summary;
use crate::grid::Grid;
use std::collections::{HashMap, VecDeque};

struct Dice(u64);

impl Dice {
    fn roll(&mut self, sides: u64) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0 % sides
    }
}

enum Late {
    Answer(Vec<String>),
    Thumb(String),
}

struct Shelf {
    grid: Grid,
    shown: (usize, usize),
    late: VecDeque<Late>,
    made: Vec<String>,
}

fn path(uid: &str) -> String {
    format!("/covers/{uid}.png")
}

impl Shelf {
    /// What the app does when the rows shown change (`ItemModel::visible`).
    fn visible(&mut self) {
        if let Some(asked) = self.grid.request(self.shown.0, self.shown.1) {
            self.late.push_back(Late::Answer(asked));
        }
    }

    /// The library answers: covers it has now, the others later as `Thumb` events; or it fails.
    fn answer(&mut self, asked: Vec<String>, dice: &mut Dice, healthy: bool) {
        let found = (healthy || dice.roll(4) != 0).then(|| {
            let (ready, later): (Vec<_>, Vec<_>) =
                asked.into_iter().partition(|uid| healthy || self.made.contains(uid) || dice.roll(2) == 0);
            later.into_iter().for_each(|uid| self.late.push_back(Late::Thumb(uid)));
            ready.iter().map(|uid| (uid.clone(), path(uid))).collect::<HashMap<_, _>>()
        });
        let answer = self.grid.answered(found);
        if answer.again.is_some() || answer.retry.is_some() {
            self.visible();
        }
    }

    fn deliver(&mut self, dice: &mut Dice, healthy: bool) {
        match self.late.pop_front() {
            Some(Late::Answer(asked)) => self.answer(asked, dice, healthy),
            Some(Late::Thumb(uid)) => {
                self.made.push(uid.clone());
                self.grid.thumb(&uid, &path(&uid));
            }
            None => {}
        }
    }
}

#[test]
fn every_cover_of_the_rows_shown_arrives_whatever_the_order() {
    let base = summary("base");
    let item = |uid: &String| reiwand_core::model::ItemSummary { uid: uid.clone(), ..base.clone() };
    for seed in 1..=400u64 {
        let mut dice = Dice(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
        let mut shelf = Shelf {
            grid: Grid { size: 256, ..Grid::default() },
            shown: (0, 0),
            late: VecDeque::new(),
            made: Vec::new(),
        };
        let uids: Vec<String> = (0..40).map(|i| format!("U{i:02}")).collect();
        // A few searches typed and taken back ("s", "sd", "s"…): the same rows go and come back.
        let searches: Vec<Vec<String>> = (0..3)
            .map(|_| {
                let from = dice.roll(20) as usize;
                uids[from..from + 1 + dice.roll(20) as usize].to_vec()
            })
            .chain([Vec::new()])
            .collect();
        for _ in 0..60 {
            match dice.roll(5) {
                0 | 1 => {
                    let rows = searches[dice.roll(4) as usize].iter().map(item).collect();
                    shelf.grid.reset(rows, true);
                    shelf.shown = (0, shelf.grid.rows.len().saturating_sub(1).min(11));
                    shelf.visible();
                }
                2 => shelf.visible(),
                _ => shelf.deliver(&mut dice, false),
            }
        }
        for _ in 0..1000 {
            if shelf.late.is_empty() {
                break;
            }
            shelf.deliver(&mut dice, true);
        }
        let (first, last) = shelf.shown;
        let empty: Vec<&str> = shelf
            .grid
            .rows
            .iter()
            .skip(first)
            .take(last + 1 - first)
            .filter(|r| r.thumb.is_none())
            .map(|r| r.item.uid.as_str())
            .collect();
        assert!(empty.is_empty(), "seed {seed}: rows shown without a cover: {empty:?}");
    }
}
