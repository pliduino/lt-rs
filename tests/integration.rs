/// Integration tests for libtorrent-rs bindings.
///
/// Each test mirrors a libtorrent C++ test in vendor/libtorrent/test/.
/// Run with: cargo test

use libtorrent_rs::{
    add_torrent_params::AddTorrentParams,
    alerts::{Alert, TorrentAlert},
    create_torrent::{CreateTorrent, FileStorage},
    ip_filter::{self, IpFilter},
    session::LtSession,
    torrent_handle::{MoveFlags, PauseFlags, TorrentFlags, TorrentHandle},
    torrent_status::TorrentStatus,
};
use std::{fs, path::PathBuf, time::Duration, thread};

fn tmp_dir(name: &str) -> PathBuf {
    let p = std::env::temp_dir().join(format!("lt_rs_test_{}", name));
    let _ = fs::remove_dir_all(&p);
    fs::create_dir_all(&p).unwrap();
    p
}

/// Add a magnet torrent and wait for its handle via AddTorrentAlert.
fn add_and_get_handle(session: &mut LtSession, magnet: &str, save_path: &str) -> Option<TorrentHandle> {
    let mut params = AddTorrentParams::parse_magnet_uri(magnet).unwrap();
    params.set_path(save_path);
    session.async_add_torrent(&params);

    let deadline = std::time::Instant::now() + Duration::from_secs(2);
    while std::time::Instant::now() < deadline {
        session.pop_alerts();
        for alert in session.alerts() {
            if let Alert::TorrentAlert(TorrentAlert::AddTorrent(a)) = alert {
                if a.error().is_ok() {
                    return Some(a.handle());
                }
            }
        }
        thread::sleep(Duration::from_millis(20));
    }
    None
}

// ─── Session ──────────────────────────────────────────────────────────────────

/// Ref: vendor/libtorrent/test/test_session.cpp
#[test]
fn session_creates_and_drops() {
    let _s = LtSession::new();
}

#[test]
fn session_get_torrent_hashes_empty() {
    let mut s = LtSession::new();
    assert_eq!(s.get_torrent_hashes().len(), 0);
}

// ─── AddTorrentParams ─────────────────────────────────────────────────────────

/// Ref: vendor/libtorrent/test/test_magnet.cpp
#[test]
fn parse_magnet_uri_roundtrip() {
    let magnet = "magnet:?xt=urn:btih:aabbccddeeff00112233445566778899aabbccdd&dn=test";
    let params = AddTorrentParams::parse_magnet_uri(magnet).expect("parse failed");
    let out = params.make_magnet_uri().expect("make_magnet_uri failed");
    assert!(out.contains("xt=urn:btih:"), "output: {}", out);
}

/// Ref: vendor/libtorrent/test/test_resume.cpp
#[test]
fn resume_data_roundtrip() {
    let magnet = "magnet:?xt=urn:btih:aabbccddeeff00112233445566778899aabbccdd&dn=resume_test";
    let params = AddTorrentParams::parse_magnet_uri(magnet).unwrap();
    let bytes = params.write_resume_data_buf();
    assert!(!bytes.is_empty(), "resume data should not be empty");
    let _params2 = AddTorrentParams::load_resume_data(&bytes);
}

/// Verify set_total_uploaded/downloaded seed all-time stats into the session.
#[test]
fn add_torrent_params_total_stats() {
    let dir = tmp_dir("total_stats");
    let mut params = AddTorrentParams::parse_magnet_uri(
        "magnet:?xt=urn:btih:aabbccddeeff00112233445566778899aabbccdd"
    ).unwrap();
    params.set_path(dir.to_str().unwrap());
    params.set_total_uploaded(1_000_000);
    params.set_total_downloaded(500_000);

    let mut s = LtSession::new();
    s.async_add_torrent(&params);
    thread::sleep(Duration::from_millis(300));
    s.pop_alerts();
    let hashes = s.get_torrent_hashes();
    assert!(!hashes.is_empty(), "torrent should be in session");
    let h = s.find_torrent(&hashes[0].as_hex()).expect("handle");
    let st = h.status();
    assert_eq!(st.all_time_upload,   1_000_000);
    assert_eq!(st.all_time_download, 500_000);
}

// ─── CreateTorrent ────────────────────────────────────────────────────────────

/// Ref: vendor/libtorrent/test/test_create_torrent.cpp
#[test]
fn create_torrent_single_file() {
    let dir = tmp_dir("create_single");
    let file = dir.join("hello.txt");
    fs::write(&file, b"hello world from libtorrent-rs").unwrap();

    let mut fs_obj = FileStorage::new();
    fs_obj.add_files(file.to_str().unwrap());
    assert_eq!(fs_obj.num_files(), 1);
    assert!(fs_obj.total_size() > 0);

    let mut ct = CreateTorrent::new(&mut fs_obj, 0);
    ct.set_creator("libtorrent-rs test");
    ct.set_comment("integration test");
    ct.set_piece_hashes(dir.to_str().unwrap());

    let bytes = ct.generate();
    assert!(!bytes.is_empty());
    assert_eq!(bytes[0], b'd', "torrent must be a bencoded dict");
}

#[test]
fn create_torrent_with_tracker() {
    let dir = tmp_dir("create_tracker");
    fs::write(dir.join("file.bin"), vec![0u8; 1024]).unwrap();

    let mut fs_obj = FileStorage::new();
    fs_obj.add_files(dir.join("file.bin").to_str().unwrap());

    let mut ct = CreateTorrent::new(&mut fs_obj, 0);
    ct.add_tracker("udp://tracker.example.com:6969/announce", 0);
    ct.set_piece_hashes(dir.to_str().unwrap());

    let bytes = ct.generate();
    assert!(!bytes.is_empty());
}

#[test]
fn create_torrent_private() {
    let dir = tmp_dir("create_private");
    fs::write(dir.join("f.txt"), b"data").unwrap();

    let mut fs_obj = FileStorage::new();
    fs_obj.add_files(dir.join("f.txt").to_str().unwrap());
    let mut ct = CreateTorrent::new(&mut fs_obj, 0);
    ct.set_priv(true);
    ct.set_piece_hashes(dir.to_str().unwrap());
    let bytes = ct.generate();
    assert!(!bytes.is_empty());
}

// ─── IpFilter ─────────────────────────────────────────────────────────────────

/// Ref: vendor/libtorrent/test/test_ip_filter.cpp
#[test]
fn ip_filter_block_range() {
    let mut f = IpFilter::new();
    f.add_rule("192.168.1.0", "192.168.1.255", ip_filter::access_flags::BLOCKED);
    assert_eq!(f.access("192.168.1.50"), ip_filter::access_flags::BLOCKED);
    assert_eq!(f.access("192.168.2.1"), 0);
    assert_eq!(f.access("10.0.0.1"), 0);
}

#[test]
fn ip_filter_allow_after_block() {
    let mut f = IpFilter::new();
    f.add_rule("10.0.0.0", "10.255.255.255", ip_filter::access_flags::BLOCKED);
    f.add_rule("10.0.1.0", "10.0.1.255", 0);
    assert_eq!(f.access("10.0.0.1"), ip_filter::access_flags::BLOCKED);
    assert_eq!(f.access("10.0.1.1"), 0);
}

#[test]
fn ip_filter_applied_to_session() {
    let mut f = IpFilter::new();
    f.add_rule("1.2.3.4", "1.2.3.4", ip_filter::access_flags::BLOCKED);
    let mut s = LtSession::new();
    s.set_ip_filter(&f);
    // get_ip_filter roundtrip: rule must survive set→get
    let f2 = s.get_ip_filter();
    assert_eq!(f2.access("1.2.3.4"), ip_filter::access_flags::BLOCKED);
    assert_eq!(f2.access("1.2.3.5"), 0);
}

// ─── Session: find_torrent + save/load state ──────────────────────────────────

/// Ref: vendor/libtorrent/test/test_session.cpp
#[test]
fn session_find_torrent_by_hash() {
    let dir = tmp_dir("find");
    let mut s = LtSession::new();
    let h = add_and_get_handle(&mut s, "magnet:?xt=urn:btih:aabbccddeeff00112233445566778899aabbccdd", dir.to_str().unwrap())
        .expect("handle");
    let hex = h.info_hashes().as_hex();
    let found = s.find_torrent(&hex);
    assert!(found.is_some(), "find_torrent should return handle for known hash");
    assert!(found.unwrap().in_session());
}

#[test]
fn session_save_load_state() {
    let mut s = LtSession::new();
    let state = s.save_state();
    assert!(!state.is_empty(), "save_state should return non-empty bytes");
    // Load into a fresh session — should not panic
    let mut s2 = LtSession::new();
    s2.load_state(&state);
}

// ─── TorrentHandle: add and get handle ───────────────────────────────────────

/// Verify TorrentStatus snapshot returns all fields without crashing.
/// Ref: vendor/libtorrent/test/test_torrent_info.cpp
#[test]
fn torrent_handle_status_snapshot() {
    let dir = tmp_dir("status");
    let mut s = LtSession::new();
    let h = add_and_get_handle(&mut s, "magnet:?xt=urn:btih:aabbccddeeff00112233445566778899aabbccdd", dir.to_str().unwrap())
        .expect("handle");
    let st: TorrentStatus = h.status();
    // Magnet without metadata: downloading metadata state, no peers yet
    assert!(!st.is_seeding);
    assert!(!st.is_finished);
    assert!(st.download_rate >= 0);
    assert!(st.upload_rate >= 0);
    assert!(st.progress >= 0.0 && st.progress <= 1.0);
}

/// Ref: vendor/libtorrent/test/test_checking.cpp
#[test]
fn torrent_handle_force_recheck() {
    let dir = tmp_dir("recheck");
    let mut s = LtSession::new();
    let h = add_and_get_handle(&mut s, "magnet:?xt=urn:btih:aabbccddeeff00112233445566778899aabbccdd", dir.to_str().unwrap())
        .expect("handle");
    // Should not panic — libtorrent queues a recheck even without data
    h.force_recheck();
    // Give it a moment to process
    thread::sleep(Duration::from_millis(100));
    s.pop_alerts(); // drain any resulting alerts
}

#[test]
fn async_add_torrent_fires_alert() {
    let dir = tmp_dir("add_alert");
    let mut s = LtSession::new();
    let h = add_and_get_handle(&mut s, "magnet:?xt=urn:btih:aabbccddeeff00112233445566778899aabbccdd", dir.to_str().unwrap());
    assert!(h.is_some(), "AddTorrentAlert must fire within 2s");
    assert!(h.unwrap().in_session());
}

// ─── TorrentHandle: flags ─────────────────────────────────────────────────────

/// Ref: vendor/libtorrent/test/test_tracker_manager.cpp
#[test]
fn torrent_handle_trackers_announce_entry() {
    let dir = tmp_dir("trackers");
    let mut s = LtSession::new();
    let h = add_and_get_handle(&mut s,
        "magnet:?xt=urn:btih:aabbccddeeff00112233445566778899aabbccdd&tr=udp://tracker.example.com:6969/announce",
        dir.to_str().unwrap()).expect("handle");

    let entries = h.trackers();
    assert!(!entries.is_empty(), "magnet tracker should be visible");
    let e = &entries[0];
    assert!(e.url.contains("tracker.example.com"), "url: {}", e.url);
    assert_eq!(e.tier, 0);
    // Not yet scraped — counts are -1
    assert_eq!(e.scrape_complete, -1);
    assert_eq!(e.scrape_incomplete, -1);

    // add_tracker and verify it shows up
    h.add_tracker("udp://second.example.com:6969/announce", 1);
    let entries2 = h.trackers();
    assert_eq!(entries2.len(), 2);
    assert_eq!(entries2[1].tier, 1);
}

/// Ref: vendor/libtorrent/test/test_flags.cpp
#[test]
fn torrent_handle_flags() {
    let dir = tmp_dir("flags");
    let mut session = LtSession::new();
    let h = add_and_get_handle(&mut session, "magnet:?xt=urn:btih:aabbccddeeff00112233445566778899aabbccdd", dir.to_str().unwrap())
        .expect("handle not received");

    h.set_flags(TorrentFlags::Sequential);
    assert!(h.flags().contains(TorrentFlags::Sequential));

    h.unset_flags(TorrentFlags::Sequential);
    assert!(!h.flags().contains(TorrentFlags::Sequential));
}

// ─── TorrentHandle: rate limits ──────────────────────────────────────────────

/// Ref: vendor/libtorrent/test/test_bandwidth_limiter.cpp
#[test]
fn torrent_handle_rate_limits() {
    let dir = tmp_dir("limits");
    let mut session = LtSession::new();
    let h = add_and_get_handle(&mut session, "magnet:?xt=urn:btih:aabbccddeeff00112233445566778899aabbccdd", dir.to_str().unwrap())
        .expect("handle not received");

    h.set_upload_limit(100 * 1024);
    h.set_download_limit(200 * 1024);
    assert_eq!(h.upload_limit(),   100 * 1024);
    assert_eq!(h.download_limit(), 200 * 1024);

    h.set_upload_limit(0);
    assert_eq!(h.upload_limit(), 0);
}

// ─── TorrentHandle: pause / resume ───────────────────────────────────────────

/// Ref: vendor/libtorrent/test/test_pause.cpp
#[test]
fn torrent_handle_pause_resume() {
    let dir = tmp_dir("pause");
    let mut session = LtSession::new();
    let h = add_and_get_handle(&mut session, "magnet:?xt=urn:btih:aabbccddeeff00112233445566778899aabbccdd", dir.to_str().unwrap())
        .expect("handle not received");

    h.pause(PauseFlags::GracefulPause);
    assert!(h.flags().contains(TorrentFlags::Paused));

    h.resume();
    assert!(!h.flags().contains(TorrentFlags::Paused));
}

// ─── TorrentHandle: move_storage ─────────────────────────────────────────────

/// Ref: vendor/libtorrent/test/test_storage.cpp
#[test]
fn torrent_handle_move_storage_fires_alert() {
    let src = tmp_dir("move_src");
    let dst = tmp_dir("move_dst");

    let mut s = LtSession::new();
    let h = add_and_get_handle(&mut s, "magnet:?xt=urn:btih:aabbccddeeff00112233445566778899aabbccdd", src.to_str().unwrap())
        .expect("handle");

    h.move_storage(dst.to_str().unwrap(), MoveFlags::AlwaysReplaceFiles);

    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    let mut moved = false;
    let mut seen = Vec::new();
    while std::time::Instant::now() < deadline {
        s.pop_alerts();
        for alert in s.alerts() {
            seen.push(format!("{:?}", std::mem::discriminant(alert)));
            match alert {
                Alert::TorrentAlert(TorrentAlert::StorageMoved(_)) => { moved = true; }
                Alert::TorrentAlert(TorrentAlert::StorageMovedFailed(_)) => { panic!("StorageMovedFailed"); }
                _ => {}
            }
        }
        if moved { break; }
        thread::sleep(Duration::from_millis(20));
    }
    if !moved {
        panic!("StorageMovedAlert not received. Alerts seen: {:?}", seen);
    }
}
