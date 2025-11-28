#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type torrent_removed_alert = crate::ffi::ffi::torrent_removed_alert;
    }
    unsafe extern "C++" {
        include!("cpp/alerts/torrent_removed.h");
        include!("lt-rs/src/ffi/mod.rs.h");
        type InfoHashCpp = crate::ffi::ffi::InfoHashCpp;

        unsafe fn torrent_removed_alert_get_info_hashes(
            a: *mut torrent_removed_alert,
        ) -> InfoHashCpp;
    }
}
