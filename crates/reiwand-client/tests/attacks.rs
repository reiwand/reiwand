mod mock;

use mock::Bus;
use reiwand_client::Client;
use reiwand_core::dbus::{BUS_NAME, OBJECT_PATH};
use std::time::{Duration, Instant};

const QUICK: &str = "<limit name=\"reply_timeout\">800</limit>";

async fn within<T>(work: impl Future<Output = T>) -> T {
    tokio::time::timeout(Duration::from_secs(15), work).await.expect("the call must not hang")
}

struct Stall;

#[zbus::interface(name = "io.github.reiwand.Daemon1")]
impl Stall {
    #[zbus(property(emits_changed_signal = "const"))]
    async fn model(&self) -> u32 {
        tokio::time::sleep(Duration::from_secs(3600)).await;
        1
    }
}

#[tokio::test]
async fn without_the_service_the_error_says_it_is_not_running() {
    let bus = Bus::start();
    let started = Instant::now();
    let error = within(Client::new(bus.connect().await)).await.unwrap_err();
    assert!(format!("{error:#}").contains("not running"), "{error:#}");
    assert!(started.elapsed() < Duration::from_secs(5));
}

#[tokio::test]
async fn a_service_that_never_answers_ends_in_an_error_that_names_the_service() {
    let bus = Bus::start_with(QUICK);
    let _stalled = zbus::connection::Builder::address(bus.address())
        .name(BUS_NAME)
        .serve_at(OBJECT_PATH, Stall)
        .build()
        .await
        .unwrap();
    let error = within(Client::new(bus.connect().await)).await.unwrap_err();
    let text = format!("{error:#}");
    assert!(text.contains("reiwandd"), "the message does not say which program did not answer: {text}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_service_that_dies_during_a_call_gives_an_error_and_the_next_client_works() {
    let bus = Bus::start();
    let address = bus.address().to_string();
    let (ready, started) = std::sync::mpsc::channel();
    let (stop, stopped) = std::sync::mpsc::channel::<()>();
    let service = std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let held = runtime.block_on(async {
            zbus::connection::Builder::address(address.as_str())
                .name(BUS_NAME)
                .serve_at(OBJECT_PATH, Stall)
                .build()
                .await
                .unwrap()
        });
        ready.send(()).unwrap();
        stopped.recv().ok();
        drop(held);
        runtime.shutdown_background();
    });
    started.recv().unwrap();
    let connection = bus.connect().await;
    let call = tokio::spawn(async move { Client::new(connection).await });
    tokio::time::sleep(Duration::from_millis(300)).await;
    stop.send(()).unwrap();
    service.join().unwrap();
    let error = within(call).await.unwrap().unwrap_err();
    let text = format!("{error:#}");
    assert!(text.contains("reiwandd"), "the message does not say which program went away: {text}");
    let _back = bus.serve(Some(1)).await;
    within(Client::new(bus.connect().await)).await.unwrap();
}
