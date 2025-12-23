use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;

    // #[cfg(feature = "static-boost")]
    // let boost_dir = PathBuf::from(manifest_dir.clone() + "/vendor/boost");
    #[cfg(feature = "static-libtorrent")]
    let libtorrent_dir = PathBuf::from(manifest_dir.clone() + "/vendor/libtorrent");

    // #[cfg(feature = "static-boost")]
    // let boost_build = out_dir.join("boost-build");
    #[cfg(feature = "static-libtorrent")]
    let libtorrent_build = out_dir.join("libtorrent-build");

    #[cfg(feature = "static-boost")]
    let boost_lib_dir = boost_build.join("lib");
    #[cfg(feature = "static-libtorrent")]
    let libtorrent_lib_dir = libtorrent_build.join("lib");

    // #[cfg(feature = "static-boost")]
    // if !std::fs::exists(&boost_build)? {
    //     std::fs::create_dir_all(&boost_build).unwrap();

    //     println!("cargo:warning=Building Boost (static)...");

    //     // bootstrap
    //     let bootstrap = if cfg!(windows) {
    //         "bootstrap.bat"
    //     } else {
    //         "./bootstrap.sh"
    //     };

    //     let status = Command::new(bootstrap)
    //         .current_dir(&boost_dir)
    //         .status()
    //         .expect("Failed to bootstrap Boost");
    //     assert!(status.success());

    //     // b2 build
    //     let mut b2 = Command::new("./b2");
    //     b2.current_dir(&boost_dir).args([
    //         "link=static",
    //         "threading=multi",
    //         "cxxflags=\"-std=c++20\"",
    //         "runtime-link=static",
    //         "variant=release",
    //         "--with-system",
    //         "--with-asio",
    //         "--with-filesystem",
    //         "--with-chrono",
    //         "--with-random",
    //         "--with-date_time",
    //         "--with-thread",
    //         &format!("--build-dir={}", boost_build.display()),
    //         &format!("--stagedir={}", boost_build.display()),
    //     ]);

    //     let status = b2.status().expect("Failed to build Boost");
    //     assert!(status.success(), "Boost build failed");
    // }

    // #[cfg(feature = "static-boost")]
    // println!("cargo:rustc-link-search=native={}", boost_lib_dir.display());

    #[cfg(feature = "static-libtorrent")]
    if !std::fs::exists(&libtorrent_build)? {
        let mut b2 = Command::new("b2");
        b2.current_dir(&libtorrent_dir).args([
            &format!("--prefix={}", libtorrent_lib_dir.display()),
            "link=static",
            "cxxstd=20",
            "runtime-link=static",
            "variant=release",
            "boost-link=static",
            "define=BOOST_ASIO_NO_DEPRECATED",
            "define=BOOST_ASIO_HEADER_ONLY",
            "define=TORRENT_NO_DEPRECATE",
            "deprecated-functions=off",
            &format!("location={}", libtorrent_lib_dir.display()),
            //"--layout=system",
            &format!("--build-dir={}", libtorrent_build.display()),
        ]);

        #[cfg(feature = "lto")]
        b2.arg("lto");

        let status = b2.status().expect("Failed to build libtorrent");
        assert!(status.success(), "libtorrent build failed");
    }

    #[cfg(feature = "static-libtorrent")]
    println!(
        "cargo:rustc-link-search=native={}",
        libtorrent_lib_dir.display()
    );

    let mut static_libs: Vec<&str> = Vec::new();
    let mut dynamic_libs: Vec<&str> = Vec::new();

    let boost_libs = [
        "boost_atomic",
        "boost_chrono",
        "boost_container",
        "boost_date_time",
        "boost_exception",
        "boost_filesystem",
        "boost_random",
        "boost_thread",
    ];

    cfg_if::cfg_if! {
        if #[cfg(feature = "static-boost")] {
            static_libs.extend(boost_libs.iter());
        } else {
            dynamic_libs.extend(boost_libs.iter());
        }
    }

    cfg_if::cfg_if! {
        if #[cfg(feature = "static-libtorrent")] {
            static_libs.push("torrent-rasterbar");
        } else {
            dynamic_libs.push("torrent-rasterbar");
        }
    }

    cfg_if::cfg_if! {
        if #[cfg(feature = "static-ssl")] {
            static_libs.push("ssl");
            static_libs.push("crypto");
        } else {
            dynamic_libs.push("ssl");
            dynamic_libs.push("crypto");
        }
    };

    for lib in static_libs {
        println!("cargo:rustc-link-lib=static={}", lib);
    }
    for lib in dynamic_libs {
        println!("cargo:rustc-link-lib={}", lib);
    }

    let rust_srcs = find_files(Path::new("src/ffi"), "rs");

    let mut cxx = cxx_build::bridges(rust_srcs);

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        cxx.define("_WIN32_WINNT", Some("0x0A00"));
        cxx.flag_if_supported("/EHsc");
    } else {
    }

    if cfg!(feature = "lto") {
        cxx.flag("-flto=thin");
    }
    cxx.compiler("clang++");
    cxx.define("TORRENT_NO_DEPRECATE", Some("1"));

    let cxx_src_files = find_files(Path::new("cpp"), "cpp");
    let cxx_header_files = find_files(Path::new("cpp"), "h");

    cxx.files(&cxx_src_files)
        // This is a hack until we find why the try_signal library is not being built.
        .file(libtorrent_dir.join("deps/try_signal/signal_error_code.cpp"))
        .file(libtorrent_dir.join("deps/try_signal/try_signal.cpp"))
        .include(libtorrent_dir.join("deps/try_signal"))
        .std("c++20")
        .include(&manifest_dir)
        .include(libtorrent_dir.join("include"))
        .define("BOOST_ASIO_HEADER_ONLY", Some("1"))
        .flag_if_supported("-O3")
        .compile("ltbridge");

    for header_file in &cxx_header_files {
        println!("cargo:rerun-if-changed={}", header_file.display());
    }
    for src_file in &cxx_src_files {
        println!("cargo:rerun-if-changed={}", src_file.display());
    }
    println!("cargo:rerun-if-changed=src/ffi/mod.rs");

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
