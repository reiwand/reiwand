use super::*;
use std::net::TcpListener;
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use std::sync::{Arc, Mutex};

static ONE: Mutex<()> = Mutex::new(());

fn counted() -> (String, Arc<AtomicUsize>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let (address, count) = (listener.local_addr().unwrap().to_string(), Arc::new(AtomicUsize::new(0)));
    let seen = count.clone();
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            seen.fetch_add(1, SeqCst);
            drop(stream);
        }
    });
    (address, count)
}

#[test]
fn private_mode_refuses_a_raw_connection_before_it_starts() {
    let _one = ONE.lock().unwrap();
    let (address, count) = counted();
    set_private(true);
    let error = tcp(&address, Duration::from_secs(2)).unwrap_err();
    assert!(refused(&error) && error.to_string() == REFUSED, "{error}");
    assert!(tcp("name.invalid:80", Duration::from_secs(2)).is_err_and(|e| refused(&e)));
    set_private(false);
    assert!(tcp(&address, Duration::from_secs(2)).is_ok());
    std::thread::sleep(Duration::from_millis(100));
    assert_eq!(count.load(SeqCst), 1, "only the connection made with private mode off arrives");
}

#[cfg(feature = "http")]
#[test]
fn private_mode_refuses_every_request_of_a_gated_client() {
    let _one = ONE.lock().unwrap();
    let (address, count) = counted();
    let client = blocking().build().unwrap();
    set_private(true);
    let error = client.get(format!("http://{address}/")).send().unwrap_err();
    assert!(refused(&error), "{error:?}");
    set_private(false);
    let _ = client.get(format!("http://{address}/")).send();
    std::thread::sleep(Duration::from_millis(100));
    assert_eq!(count.load(SeqCst), 1, "the request made with private mode off did not arrive");
}

#[test]
fn the_setting_is_an_option_off_by_default() {
    let mut config = crate::Config::default();
    assert!(!config.network.private && config.options().contains_key(KEY));
    config.set_option(KEY, "true").unwrap();
    assert!(config.network.private);
}

#[test]
fn the_update_check_is_an_option_on_without_its_block() {
    let mut config = crate::Config::default();
    assert!(config.auto_updates() && config.updates.is_none());
    assert_eq!(config.options().get(super::UPDATES), Some(&toml::Value::Boolean(true)));
    config.set_option(super::UPDATES, "false").unwrap();
    assert!(!config.auto_updates());
    let text = toml::to_string_pretty(&config).unwrap();
    assert!(text.contains("[updates]\nauto = false"), "{text}");
    assert!(!crate::Config::parse(&text).unwrap().auto_updates());
    let plain = toml::to_string_pretty(&crate::Config::default()).unwrap();
    assert!(!plain.contains("[updates]") && crate::Config::parse(&plain).unwrap().auto_updates());
}
