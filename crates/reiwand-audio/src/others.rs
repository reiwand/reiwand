use pipewire as pw;
use pw::node::{Node, NodeChangeMask, NodeListener, NodeState};
use pw::types::ObjectType;
use std::cell::{Cell, RefCell};
use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;
use std::sync::mpsc;
use std::thread::JoinHandle;

const STREAM: &str = "Stream/Output/Audio";

/// The property every sound stream of this process carries, and its value.
pub type Owner = (&'static str, &'static str);

pub struct Others {
    quit: pw::channel::Sender<()>,
    thread: Option<JoinHandle<()>>,
}

struct Seen {
    nodes: HashMap<u32, (Node, NodeListener)>,
    playing: BTreeSet<u32>,
    told: Option<bool>,
    tell: Box<dyn Fn(bool)>,
}

impl Seen {
    fn set(&mut self, id: u32, playing: bool) {
        match playing {
            true => self.playing.insert(id),
            false => self.playing.remove(&id),
        };
        let now = !self.playing.is_empty();
        if self.told.replace(now) != Some(now) {
            (self.tell)(now);
        }
    }
}

/// Whether a PipeWire node is a sound another application plays: a playback stream that neither
/// carries this process's own mark (`owner`, a property and its value) nor comes from a process `ours`
/// claims. Streams made through PipeWire itself carry no process id, so the mark is what tells ours.
fn theirs<'a>(value: impl Fn(&str) -> Option<&'a str>, ours: &dyn Fn(u32) -> bool, owner: Owner) -> bool {
    let pid = value("application.process.id").and_then(|p| p.parse().ok());
    value("media.class") == Some(STREAM) && value(owner.0) != Some(owner.1) && !pid.is_some_and(ours)
}

impl Others {
    pub fn watch(
        ours: impl Fn(u32) -> bool + Send + 'static,
        owner: Owner,
        tell: impl Fn(bool) + Send + 'static,
    ) -> Result<Others, String> {
        let ((quit, rx), (ready_tx, ready)) = (pw::channel::channel(), mpsc::channel());
        let thread = std::thread::Builder::new()
            .name("reiwand-others".into())
            .spawn(move || {
                if let Err(e) = run(rx, Box::new(ours), owner, Box::new(tell), &ready_tx) {
                    let _ = ready_tx.send(Err(e.to_string()));
                }
            })
            .map_err(|e| e.to_string())?;
        match ready.recv() {
            Ok(Ok(())) => Ok(Others { quit, thread: Some(thread) }),
            Ok(Err(e)) => Err(e),
            Err(_) => Err("the playback watcher stopped".into()),
        }
    }
}

impl Drop for Others {
    fn drop(&mut self) {
        let _ = self.quit.send(());
        self.thread.take().map(JoinHandle::join);
    }
}

fn run(
    rx: pw::channel::Receiver<()>,
    ours: Box<dyn Fn(u32) -> bool>,
    owner: Owner,
    tell: Box<dyn Fn(bool)>,
    ready: &mpsc::Sender<Result<(), String>>,
) -> Result<(), pw::Error> {
    pw::init();
    let mainloop = pw::main_loop::MainLoopRc::new(None)?;
    let context = pw::context::ContextRc::new(&mainloop, None)?;
    let core = context.connect_rc(None)?;
    let registry = core.get_registry_rc()?;
    let seen =
        Rc::new(RefCell::new(Seen { nodes: HashMap::new(), playing: BTreeSet::new(), told: None, tell }));
    let running = Rc::new(Cell::new(true));
    let (added, removed, bound, lost) =
        (Rc::downgrade(&seen), seen.clone(), registry.clone(), running.clone());
    let _core = core
        .add_listener_local()
        .error(move |id, _, _, _| lost.set(lost.get() && id != pw::core::PW_ID_CORE))
        .register();
    let ours: Rc<dyn Fn(u32) -> bool> = Rc::from(ours);
    let _listener = registry
        .add_listener_local()
        .global(move |global| {
            // The registry only carries a few properties of a node (its class, not the mark or the
            // process id): which node is whose is read from the node's own info below.
            let class = global.props.and_then(|p| p.get("media.class"));
            if global.type_ != ObjectType::Node || class != Some(STREAM) {
                return;
            }
            let Ok(node) = bound.bind::<Node, _>(global) else { return };
            let (id, state, ours) = (global.id, added.clone(), ours.clone());
            let counted = Cell::new(false);
            let listener = node
                .add_listener_local()
                .info(move |info| {
                    // Only an info that says its properties changed carries them.
                    if let Some(props) =
                        info.props().filter(|_| info.change_mask().contains(NodeChangeMask::PROPS))
                    {
                        counted.set(theirs(|key| props.get(key), &*ours, owner));
                    }
                    let running = counted.get() && matches!(info.state(), NodeState::Running);
                    let seen = state.upgrade();
                    seen.iter().filter_map(|s| s.try_borrow_mut().ok()).for_each(|mut s| s.set(id, running));
                })
                .register();
            if let Some(mut seen) = added.upgrade().as_ref().and_then(|s| s.try_borrow_mut().ok()) {
                seen.nodes.insert(id, (node, listener));
            }
        })
        .global_remove(move |id| {
            if let Ok(mut seen) = removed.try_borrow_mut() {
                seen.nodes.remove(&id);
                seen.set(id, false);
            }
        })
        .register();
    let keep = running.clone();
    let _quit = rx.attach(mainloop.loop_(), move |()| keep.set(false));
    let _ = ready.send(Ok(()));
    while running.get() {
        mainloop.loop_().iterate(pw::loop_::Timeout::Infinite);
    }
    if let Ok(mut seen) = seen.try_borrow_mut() {
        seen.playing.clear();
        seen.set(0, false);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stream<'a>(props: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<&'a str> + 'a {
        move |key| props.iter().find(|(k, _)| *k == key).map(|(_, v)| *v)
    }

    #[test]
    fn a_stream_of_this_process_is_not_another_application_even_without_a_process_id() {
        let owner = ("reiwand.owner", "7-123");
        let ours = |pid: u32| pid == 7;
        let played = [("media.class", STREAM), ("reiwand.owner", "7-123")];
        assert!(!theirs(stream(&played), &ours, owner), "a PipeWire stream of ours carries no process id");
        let sandboxed = [("media.class", STREAM), ("application.process.id", "2")];
        assert!(theirs(stream(&sandboxed), &ours, owner), "another application, whatever its process id");
        let elsewhere = [("media.class", STREAM), ("reiwand.owner", "9-456")];
        assert!(theirs(stream(&elsewhere), &ours, owner), "another Reiwand is another application");
        let helper = [("media.class", STREAM), ("application.process.id", "7")];
        assert!(!theirs(stream(&helper), &ours, owner), "a process of ours without the mark");
        assert!(!theirs(stream(&[("media.class", "Audio/Sink")]), &ours, owner), "not a playback stream");
    }
}
