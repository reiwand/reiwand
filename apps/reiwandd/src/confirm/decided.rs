use super::tests::{connect, emit, hub, notifications, private_bus, show};
use super::*;

fn package(scene: &str) -> (String, String, String) {
    (scene.to_owned(), "sha256:dd".to_owned(), "Decided".to_owned())
}

fn refused(r: &Result<()>) -> bool {
    matches!(r, Err(Error::Fdo(fdo::Error::AccessDenied(m))) if m.contains("network or lights"))
}

#[tokio::test]
async fn an_unknown_caller_gets_one_question_for_what_it_allows() {
    let Some(bus) = private_bus() else { return };
    let ((server, mut asked), client) = (notifications(&bus.1).await, connect(&bus.1).await);
    let dir = tempfile::tempdir().unwrap();
    let (scene, hub, path) = ("org.example.decided", hub(), dir.path().join("inputs.json"));
    show(&hub, scene, "DP-1", "sha256:dd");
    let both = vec![Input::Pointer, Input::Audio];
    let answer = decide(&client, hub.clone(), path.clone(), package(scene), both).await;
    assert!(matches!(answer, Err(Error::Named(name, _)) if name == UNCONFIRMED), "the call did not wait");
    asked.recv().await.unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert!(asked.try_recv().is_err(), "the user got more than one notification");
    assert_eq!(questions().0.get(scene).map(|s| s.len()), Some(2), "the question does not name both");
    emit(&server, "ActionInvoked", &(7, "allow")).await;
    let start = Instant::now();
    while questions().0.contains_key(scene) {
        assert!(start.elapsed() < Duration::from_secs(10), "the answer was not applied");
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    let granted = lock(&hub.control).granted(scene, "sha256:dd");
    assert!(granted.contains(&Input::Audio) && granted.contains(&Input::Pointer), "the answer was lost");
}

#[tokio::test]
async fn an_unknown_caller_never_gives_the_network() {
    let Some(bus) = private_bus() else { return };
    let ((_server, mut asked), client) = (notifications(&bus.1).await, connect(&bus.1).await);
    let dir = tempfile::tempdir().unwrap();
    let (scene, hub, path) = ("org.example.networked", hub(), dir.path().join("inputs.json"));
    show(&hub, scene, "DP-1", "sha256:dd");
    let alone = decide(&client, hub.clone(), path.clone(), package(scene), vec![Input::MediaNetwork]).await;
    assert!(refused(&alone), "the network was not refused");
    assert!(asked.try_recv().is_err() && !questions().0.contains_key(scene), "the network was asked for");
    let mixed = vec![Input::Network, Input::Audio];
    let mixed = decide(&client, hub.clone(), path, package(scene), mixed).await;
    assert!(refused(&mixed), "a mix hid the refusal of the network");
    asked.recv().await.expect("the input that may be asked was not asked");
    assert_eq!(questions().0.get(scene).map(|s| s.len()), Some(1), "the network went into the question");
    assert!(lock(&hub.control).grants.is_empty());
}
