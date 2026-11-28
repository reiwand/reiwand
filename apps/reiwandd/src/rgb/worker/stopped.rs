use super::restored::{server, told};
use super::tests::session;
use super::*;
use std::sync::mpsc::channel;

#[test]
fn nothing_takes_the_devices_again_once_the_lights_are_stopped() {
    let (address, heard) = server();
    let mut s = session("TEST-STOPPED");
    s.rgb = Rgb { enabled: true, server: address, ..Rgb::default() };
    let strip = Strip::new(1, 1, vec![255, 0, 0]).unwrap();
    s.paint(&strip, None);
    assert_eq!(told(&heard).len(), 2, "the device was not taken");
    let (asks, asked) = channel();
    let worker = std::thread::spawn(move || run(asked, s));
    let (stopped, done) = channel();
    asks.send(Ask::Stop(stopped)).unwrap();
    done.recv().unwrap();
    assert_eq!(told(&heard), [(1101, 0)], "stopping left the device in direct mode");
    asks.send(Ask::Frame("TEST-STOPPED".into(), strip)).unwrap();
    let (tested, answer) = channel();
    asks.send(Ask::Test(None, tested)).unwrap();
    assert!(answer.recv().unwrap().is_err(), "a test ran after the stop");
    let (connected, answer) = channel();
    asks.send(Ask::Connect(connected)).unwrap();
    assert!(answer.recv().unwrap().is_err(), "a connection opened after the stop");
    drop(asks);
    worker.join().unwrap();
    assert!(told(&heard).is_empty(), "a late frame took the device again");
}

#[test]
fn many_change_notices_rebuild_the_connection_once_per_retry() {
    let (address, _heard) = server();
    let mut s = session("TEST-STALE");
    s.rgb = Rgb { enabled: true, server: address, ..Rgb::default() };
    s.ready().unwrap();
    s.stale = true;
    s.rebuild();
    let first = s.rebuilt.expect("the first notice did not rebuild the connection");
    assert!(!s.stale && s.client.is_some());
    s.stale = true;
    s.rebuild();
    assert_eq!(s.rebuilt, Some(first), "a second notice rebuilt the connection at once");
    assert!(s.stale && s.client.is_some(), "the notice was lost");
    s.rebuilt = Instant::now().checked_sub(RETRY);
    s.rebuild();
    assert!(!s.stale && s.client.is_some() && s.rebuilt.is_some_and(|at| at.elapsed() < RETRY));
}
