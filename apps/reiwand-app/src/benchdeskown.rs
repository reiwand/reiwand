use super::{Chosen, Desk, Own};
use crate::tr::tr;

/// The layouts the author saves by a name: saving, renaming and removing one.
impl Desk {
    /// Why `name` cannot name a layout of the author (`except` is the one being renamed), if it
    /// cannot.
    pub fn name_fault(&self, name: &str, except: Option<usize>) -> Option<String> {
        if name.trim().is_empty() {
            return Some(tr("a layout has a name"));
        }
        if name.trim() != name {
            return Some(tr("a name does not start or end with a space"));
        }
        if self.own_at(name).is_some_and(|at| Some(at) != except) {
            return Some(tr("another layout of yours has that name"));
        }
        None
    }

    pub fn save_as(&self, name: &str) -> Option<Desk> {
        if self.name_fault(name, None).is_some() {
            return None;
        }
        let mut next = self.clone();
        let view = self.view().clone();
        next.own.push(Own { name: name.to_owned(), saved: view.clone(), view });
        next.chosen = Chosen::Own(name.to_owned());
        Some(next)
    }

    pub fn rename(&self, at: usize, name: &str) -> Option<Desk> {
        if at >= self.own.len() || self.name_fault(name, Some(at)).is_some() {
            return None;
        }
        let mut next = self.clone();
        let old = std::mem::replace(&mut next.own[at].name, name.to_owned());
        if next.chosen == Chosen::Own(old) {
            next.chosen = Chosen::Own(name.to_owned());
        }
        Some(next)
    }
}
