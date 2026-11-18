mod common;
use common::*;
use reiwand_carry::*;

#[test]
fn plain_round_trip_with_own_files() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("w.bin");
    let data: Vec<u8> = (0..200_000u32).map(|n| (n % 251) as u8).collect();
    std::fs::write(&file, &data).unwrap();
    let own = vec![(OwnFile { id: "w1".into(), name: "Mine".into() }, file)];
    let path = dir.path().join("a.reiwand");
    write(&path, &export(content(), own, None)).unwrap();
    let mut opened = open(&path, None, Limits::default()).unwrap();
    assert_eq!(opened.manifest.format, 2);
    assert_eq!(opened.content.repositories, content().repositories);
    assert_eq!(opened.content.own.len(), 1);
    assert!(opened.manifest.sections.contains(&"own".to_string()));
    let mut out = Vec::new();
    opened.extract_own("w1", &mut out).unwrap();
    assert_eq!(out, data);
}

#[test]
fn encrypted_round_trip_with_secrets() {
    let mut c = content();
    c.secrets = Some(Secrets {
        sessions: vec![Session {
            repo_id: "r1".into(),
            origin: "https://example.org".into(),
            account: "me".into(),
            token: "tok".into(),
        }],
        provider_keys: [("photos".to_string(), "key".to_string())].into(),
    });
    let bytes = write_bytes(&export(c.clone(), vec![], Some("hunter2"))).unwrap();
    assert!(is_sealed(&bytes));
    assert!(!String::from_utf8_lossy(&bytes).contains("tok"));
    let opened = open_bytes(bytes.clone(), Some("hunter2"), Limits::default()).unwrap();
    assert_eq!(opened.content, c);
    assert!(matches!(open_bytes(bytes.clone(), None, Limits::default()), Err(Error::NeedsPassword)));
    assert!(matches!(open_bytes(bytes, Some("wrong"), Limits::default()), Err(Error::WrongPassword)));
}

#[test]
fn secrets_need_a_password() {
    let mut c = content();
    c.secrets = Some(Secrets { sessions: Vec::new(), provider_keys: Default::default() });
    assert!(write_bytes(&export(c, vec![], None)).is_err());
}

#[test]
fn encrypted_files_with_many_segments_and_truncation() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("big.bin");
    std::fs::write(&file, vec![7u8; 300_000]).unwrap();
    let own = vec![(OwnFile { id: "big".into(), name: "Big".into() }, file)];
    let bytes = write_bytes(&export(content(), own, Some("pw"))).unwrap();
    let mut opened = open_bytes(bytes.clone(), Some("pw"), Limits::default()).unwrap();
    let mut out = Vec::new();
    opened.extract_own("big", &mut out).unwrap();
    assert_eq!(out.len(), 300_000);
    let cut = 52 + 2 * (65_536 + 16);
    assert!(matches!(
        open_bytes(bytes[..cut].to_vec(), Some("pw"), Limits::default()),
        Err(Error::Truncated)
    ));
    assert!(open_bytes(bytes[..bytes.len() - 5].to_vec(), Some("pw"), Limits::default()).is_err());
    assert!(matches!(open_bytes(bytes[..20].to_vec(), Some("pw"), Limits::default()), Err(Error::Truncated)));
    let mut flipped = bytes.clone();
    let at = flipped.len() / 2;
    flipped[at] ^= 1;
    assert!(open_bytes(flipped, Some("pw"), Limits::default()).is_err());
}

#[test]
fn truncated_plain_file_is_refused() {
    let bytes = write_bytes(&export(content(), vec![], None)).unwrap();
    for cut in [10, bytes.len() / 2, bytes.len() - 4] {
        assert!(open_bytes(bytes[..cut].to_vec(), None, Limits::default()).is_err());
    }
}

#[test]
fn sums_catch_a_changed_content() {
    let bytes = write_bytes(&export(content(), vec![], None)).unwrap();
    let mut changed = bytes.clone();
    let text = b"\"Calm\"";
    let at = changed.windows(text.len()).position(|w| w == text);
    if let Some(at) = at {
        changed[at + 1] = b'X';
        assert!(open_bytes(changed, None, Limits::default()).is_err());
    }
}

#[test]
fn device_local_settings_are_not_portable() {
    let all = content().settings;
    let kept = portable(&all);
    assert!(kept.contains_key("playback.fps"));
    assert!(!kept.contains_key("playback.volume"));
    assert!(is_device_local("outputs.HDMI-1.wallpaper"));
    assert!(is_device_local("playback.software_parallax"));
    assert!(is_device_local("playback.hdr"));
    assert!(is_device_local("playback.hdr.DP-1"));
    assert!(is_device_local("layout.bezel_mm"));
    assert!(is_device_local("rgb.host"));
}
