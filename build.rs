use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("DOCS_RS").is_ok() { return Ok(()); }

    let out_dir  = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest = std::env::var("CARGO_MANIFEST_DIR")?;
    let lt_dir   = PathBuf::from(manifest.clone() + "/vendor/libtorrent");
    let lt_build = out_dir.join("libtorrent-build");

    // --- Prebuilt fast path ---
    // If prebuilt/libtorrent-rasterbar.a exists in the crate root, use it
    // directly and skip the ~15 min b2 compile entirely. Regenerate by running
    // `cargo build` after removing prebuilt/libtorrent-rasterbar.a.
    let prebuilt = PathBuf::from(&manifest).join("prebuilt").join("libtorrent-rasterbar.a");
    if prebuilt.exists() {
        println!("cargo:warning=Using prebuilt libtorrent-rasterbar.a");
        println!("cargo:rustc-link-search=native={}", prebuilt.parent().unwrap().display());
        println!("cargo:rustc-link-lib=static=torrent-rasterbar");
        println!("cargo:rustc-link-lib=ssl");
        println!("cargo:rustc-link-lib=crypto");
        println!("cargo:rerun-if-changed=prebuilt/libtorrent-rasterbar.a");
        return build_cxx_bridge(&manifest, &out_dir, &lt_dir);
    }

    // b2 places the .a in a deep platform-specific path under lt_build.
    // Find it after the build rather than assuming a fixed location.
    let lt_a = find_files(&lt_build, "a")
        .into_iter()
        .find(|p| p.file_name().map(|f| f == "libtorrent-rasterbar.a").unwrap_or(false));

    if lt_a.is_none() {
        println!("cargo:warning=Building libtorrent from source (one-time ~15 min)…");
        let mut b2 = Command::new("b2");
        b2.current_dir(&lt_dir).args([
            "link=static", "cxxstd=20", "variant=release",
            "define=BOOST_ASIO_HEADER_ONLY", "define=TORRENT_NO_DEPRECATE",
            "deprecated-functions=off",
            &format!("--build-dir={}", lt_build.display()),
            &format!("-j{}", std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4)),
        ]);
        if cfg!(feature = "lto") { b2.arg("lto"); }
        assert!(b2.status().expect("b2 not found — install boost-build").success(),
                "libtorrent build failed");
    }

    // Find the .a in b2's platform-specific output path
    let lt_a = find_files(&lt_build, "a")
        .into_iter()
        .find(|p| p.file_name().map(|f| f == "libtorrent-rasterbar.a").unwrap_or(false))
        .expect("libtorrent-rasterbar.a not found after b2 build");

    let lt_lib_dir = lt_a.parent().unwrap().to_path_buf();
    println!("cargo:rustc-link-search=native={}", lt_lib_dir.display());
    println!("cargo:rustc-link-lib=static=torrent-rasterbar");
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-lib=crypto");

    build_cxx_bridge(&manifest, &out_dir, &lt_dir)?;

    if cfg!(feature = "lt-tests") {
        build_lt_tests(&out_dir, &manifest)?;
    }

    Ok(())
}

fn build_cxx_bridge(manifest: &str, out_dir: &PathBuf, lt_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let try_signal = lt_dir.join("deps/try_signal");

    let rust_srcs = find_files(Path::new("src/ffi"), "rs");
    let cxx_srcs  = find_files(Path::new("cpp"), "cpp");
    let cxx_hdrs  = find_files(Path::new("cpp"), "h");

    let mut cxx = cxx_build::bridges(rust_srcs);
    if cfg!(feature = "lto") { cxx.flag("-flto=thin"); }

    cxx.compiler("clang++")
        .std("c++20")
        .include(manifest)
        .include(lt_dir.join("include"))
        .include(&try_signal)
        .define("TORRENT_NO_DEPRECATE", Some("1"))
        .define("BOOST_ASIO_HEADER_ONLY", Some("1"))
        .flag_if_supported("-O3");

    cxx.files(&cxx_srcs)
        .file(try_signal.join("signal_error_code.cpp"))
        .file(try_signal.join("try_signal.cpp"))
        .compile("ltbridge");

    for f in cxx_hdrs.iter().chain(cxx_srcs.iter()) {
        println!("cargo:rerun-if-changed={}", f.display());
    }
    println!("cargo:rerun-if-changed=src/ffi/mod.rs");

    Ok(())
}

const LT_TEST_TARGETS: &[&str] = &[
    "test_storage", "test_ip_filter", "test_flags", "test_create_torrent",
    "test_checking", "test_magnet", "test_resume", "test_add_torrent",
    "test_pause", "test_bandwidth_limiter", "test_file_progress",
    "test_torrent_info", "test_file_storage",
];

fn build_lt_tests(out_dir: &PathBuf, manifest: &str) -> Result<(), Box<dyn std::error::Error>> {
    let lt_src = PathBuf::from(manifest).join("vendor/libtorrent");
    let lt_test_build = out_dir.join("lt-test-build");
    std::fs::create_dir_all(&lt_test_build)?;

    let prefix = Command::new("pkg-config")
        .args(["--variable=prefix", "libtorrent-rasterbar"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "/usr".to_string());

    let status = Command::new("cmake")
        .args([
            "-S", lt_src.to_str().unwrap(),
            "-B", lt_test_build.to_str().unwrap(),
            "-DCMAKE_BUILD_TYPE=Release",
            "-DBUILD_SHARED_LIBS=ON",
            "-Dbuild_tests=ON",
            "-Dbuild_examples=OFF",
            "-Dbuild_tools=OFF",
            &format!("-DCMAKE_PREFIX_PATH={}", prefix),
        ])
        .status()?;
    assert!(status.success(), "cmake configure for lt-tests failed");

    let ncpu = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    let mut build_cmd = Command::new("cmake");
    build_cmd.args(["--build", lt_test_build.to_str().unwrap()])
        .arg("--parallel").arg(ncpu.to_string());
    for t in LT_TEST_TARGETS { build_cmd.arg("--target").arg(t); }
    let status = build_cmd.status()?;
    assert!(status.success(), "cmake build for lt-tests failed");

    println!("cargo:rustc-env=LT_TEST_BIN_DIR={}", lt_test_build.join("test").display());
    Ok(())
}

fn find_files(dir: &Path, ext: &str) -> Vec<PathBuf> {
    fn recurse(dir: &Path, ext: &str, out: &mut Vec<PathBuf>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for e in entries.flatten() {
                let p = e.path();
                if p.is_dir() { recurse(&p, ext, out); }
                else if p.extension().map(|e| e == ext).unwrap_or(false) { out.push(p); }
            }
        }
    }
    let mut out = Vec::new();
    recurse(dir, ext, &mut out);
    out
}
