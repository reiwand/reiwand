use super::Descriptor;
use crate::value::MAX_LISTENS;

impl Descriptor {
    pub(crate) fn listening(&self) -> Vec<String> {
        let mut found = Vec::new();
        if self.audio.len() > MAX_LISTENS {
            found.push(format!("more than {MAX_LISTENS} audio inputs"));
        }
        for (i, l) in self.audio.iter().enumerate() {
            found.extend(l.problems(&format!("audio/{i}")).into_iter().map(|p| p.to_string()));
        }
        found
    }
}
