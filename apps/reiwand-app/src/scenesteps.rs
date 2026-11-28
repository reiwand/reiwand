use crate::scene::qobject::SceneEditor;
use cxx_qt::CxxQtType;
use reiwand_editor::Action;
use serde_json::Value;
use std::pin::Pin;

impl SceneEditor {
    pub(crate) fn steps(mut self: Pin<&mut Self>, args: Value) {
        let actions = match serde_json::from_value::<Vec<Action>>(args) {
            Ok(actions) => actions,
            Err(e) => return self.fail(e),
        };
        self.as_mut().scoped();
        let done = {
            let mut this = self.as_mut().rust_mut();
            let Some(project) = this.project.as_mut() else { return };
            project.act_all(&actions)
        };
        match done {
            Ok(_) => self.as_mut().changed(),
            Err(e) => self.as_mut().fail(e),
        }
    }
}
