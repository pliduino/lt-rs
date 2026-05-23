/// libtorrent's own C++ test suite run through Rust via ctest.
///
/// ctest sets the correct working directory (test build dir) so fixture files
/// like .gz torrents, mutable_test_torrents/, ssl/ etc. are found correctly.
///
/// Run with: cargo test --features lt-tests
///
/// First run builds the C++ test binaries via cmake (~5 min one-time).
/// Subsequent runs use the cached binaries.

#[cfg(feature = "lt-tests")]
mod lt {
    use std::process::Command;
    use std::path::PathBuf;

    fn test_build_dir() -> PathBuf {
        PathBuf::from(env!("LT_TEST_BIN_DIR"))
            .parent().unwrap().to_path_buf()  // go up from test/ to build root
    }

    /// Run a named libtorrent ctest. ctest sets working dir to the build root
    /// where fixture files live. Returns (passed, failed, output).
    fn run_lt_ctest(name: &str) -> (u32, u32, String) {
        let build_dir = test_build_dir();
        let out = Command::new("ctest")
            .current_dir(&build_dir)
            .args(["-R", name, "--output-on-failure"])
            .output()
            .unwrap_or_else(|e| panic!("ctest not found: {}", e));
        let combined = String::from_utf8_lossy(&out.stdout).to_string()
            + &String::from_utf8_lossy(&out.stderr);
        let passed = combined.matches("***PASS***").count() as u32;
        let failed = combined.matches("***FAIL***").count() as u32
                   + combined.matches("***** ").count() as u32;
        (passed, failed, combined)
    }

    macro_rules! lt_test {
        ($name:ident) => {
            #[test]
            fn $name() {
                let (passed, failed, output) = run_lt_ctest(stringify!($name));
                assert!(
                    failed == 0 && passed > 0,
                    "libtorrent {} - {} passed, {} failed:\n{}",
                    stringify!($name), passed, failed,
                    // Only show last 30 lines of output on failure
                    output.lines().rev().take(30).collect::<Vec<_>>()
                           .into_iter().rev().collect::<Vec<_>>().join("\n")
                );
            }
        };
    }

    // ── C++ tests mirroring our Rust bindings ────────────────────────────────
    lt_test!(test_ip_filter);        // IpFilter block/allow/access + set/get session filter
    lt_test!(test_magnet);           // AddTorrentParams magnet parse/make
    lt_test!(test_resume);           // Resume data read/write roundtrip (50+ cases)
    lt_test!(test_create_torrent);   // FileStorage + CreateTorrent + set_piece_hashes
    lt_test!(test_flags);            // TorrentHandle set/unset/get flags (11 cases)
    lt_test!(test_file_storage);     // FileStorage file listing and metadata
    lt_test!(test_torrent_info);     // TorrentInfo snapshot fields
    lt_test!(test_bandwidth_limiter); // set/get upload + download rate limits
    lt_test!(test_checking);         // force_recheck + piece verification
    lt_test!(test_add_torrent);      // session async_add_torrent + AddTorrentAlert
    lt_test!(test_storage);          // move_storage + StorageMovedAlert
    lt_test!(test_file_progress);    // prioritize_files / file_priorities
}
