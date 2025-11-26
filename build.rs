use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=vendor/boost");
    println!("cargo:rerun-if-changed=vendor/libtorrent");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;

    let boost_dir = PathBuf::from(manifest_dir.clone() + "/vendor/boost");
    let libtorrent_dir = PathBuf::from(manifest_dir.clone() + "/vendor/libtorrent");

    let boost_build = out_dir.join("boost-build");
    let libtorrent_build = out_dir.join("libtorrent-build");

    let boost_lib_dir = boost_build.join("lib");
    let libtorrent_lib_dir = libtorrent_build.join("torrent/gcc-14/release/cxxstd-14-iso/deprecated-functions-off/link-static/threading-multi/visibility-hidden");

    if !std::fs::exists(&boost_build)? {
        std::fs::create_dir_all(&boost_build).unwrap();

        println!("cargo:warning=Building Boost (static)...");

        // bootstrap
        let bootstrap = if cfg!(windows) {
            "bootstrap.bat"
        } else {
            "./bootstrap.sh"
        };

        let status = Command::new(bootstrap)
            .current_dir(&boost_dir)
            .status()
            .expect("Failed to bootstrap Boost");
        assert!(status.success());

        // b2 build
        let mut b2 = Command::new("./b2");
        b2.current_dir(&boost_dir).args([
            "link=static",
            "threading=multi",
            "cxxflags=\"-std=c++14\"",
            "runtime-link=static",
            "variant=release",
            "--with-system",
            "--with-asio",
            "--with-filesystem",
            "--with-chrono",
            "--with-random",
            "--with-date_time",
            "--with-thread",
            &format!("--build-dir={}", boost_build.display()),
            &format!("--stagedir={}", boost_build.display()),
        ]);

        let status = b2.status().expect("Failed to build Boost");
        assert!(status.success(), "Boost build failed");
    }

    println!("cargo:rustc-link-search=native={}", boost_lib_dir.display());
    for lib in [
        "boost_atomic",
        "boost_chrono",
        "boost_container",
        "boost_date_time",
        "boost_exception",
        "boost_filesystem",
        "boost_random",
        "boost_thread",
    ] {
        println!("cargo:rustc-link-lib=static={}", lib);
    }

    for lib in ["ssl", "crypto"] {
        println!("cargo:rustc-link-lib={}", lib);
    }

    if !std::fs::exists(&libtorrent_build)? {
        let mut b2 = Command::new("../boost/b2");
        b2.current_dir(&libtorrent_dir).args([
            "link=static",
            "threading=multi",
            "cxxflags=\"-std=c++14\"",
            "cxxstd=14",
            "runtime-link=static",
            "variant=release",
            "boost-link=static",
            "define=BOOST_ASIO_NO_DEPRECATED",
            "define=BOOST_ASIO_HEADER_ONLY",
            "deprecated-functions=off",
            &format!("--build-dir={}", libtorrent_build.display()),
            &format!("--stagedir={}", libtorrent_build.display()),
            &format!("include={}", boost_dir.display()),
        ]);

        let status = b2.status().expect("Failed to build libtorrent");
        assert!(status.success(), "libtorrent build failed");
    }

    println!(
        "cargo:rustc-link-search=native={}",
        libtorrent_lib_dir.display()
    );
    println!("cargo:rustc-link-lib=static=torrent-rasterbar");

    let mut cxx = cxx_build::bridge("src/ffi.rs");

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        cxx.define("_WIN32_WINNT", Some("0x0A00"));
        cxx.flag_if_supported("/EHsc");
    }

    cxx.define("TORRENT_NO_DEPRECATE", Some("1"));

    let cxx_src_files = find_files(Path::new("cpp"), "cpp");
    let cxx_header_files = find_files(Path::new("cpp"), "h");

    cxx.files(&cxx_src_files)
        // This is a hack until we find why the try_signal library is not being built.
        .file(libtorrent_dir.join("deps/try_signal/signal_error_code.cpp"))
        .file(libtorrent_dir.join("deps/try_signal/try_signal.cpp"))
        .include(libtorrent_dir.join("deps/try_signal"))
        .std("c++14")
        .include(&manifest_dir)
        .include(libtorrent_dir.join("include"))
        .include(boost_dir)
        .define("BOOST_ASIO_HEADER_ONLY", Some("1"))
        .flag_if_supported("-O3")
        .compile("ltbridge");

    for header_file in &cxx_header_files {
        println!("cargo:rerun-if-changed={}", header_file.display());
    }
    for src_file in &cxx_src_files {
        println!("cargo:rerun-if-changed={}", src_file.display());
    }
    println!("cargo:rerun-if-changed=src/ffi.rs");

    Ok(())
}

fn find_files(dir: &Path, ext: &str) -> Vec<PathBuf> {
    fn find_cpp_recursive(dir: &Path, ext: &str, out: &mut Vec<PathBuf>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    find_cpp_recursive(&path, ext, out);
                } else if path.extension().map(|e| e == ext).unwrap_or(false) {
                    out.push(path);
                }
            }
        }
    }

    let mut files = Vec::new();
    find_cpp_recursive(dir, ext, &mut files);

    files
}
