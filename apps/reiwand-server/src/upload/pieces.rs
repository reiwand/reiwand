use crate::error::{Problem, Res};
use std::collections::HashSet;
use std::sync::{Mutex, PoisonError};

#[derive(Default)]
pub struct Pieces(Mutex<HashSet<(i64, String)>>);

pub struct Turn<'a>(&'a Pieces, (i64, String));

impl Pieces {
    pub fn enter(&self, user: i64, id: &str) -> Res<Turn<'_>> {
        let mut busy = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        let key = (user, id.to_string());
        if busy.contains(&key) || busy.iter().filter(|(u, _)| *u == user).count() >= 4 {
            return Err(Problem::busy(5));
        }
        busy.insert(key.clone());
        Ok(Turn(self, key))
    }
}

impl Drop for Turn<'_> {
    fn drop(&mut self) {
        self.0.0.lock().unwrap_or_else(PoisonError::into_inner).remove(&self.1);
    }
}
