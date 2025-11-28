#[cxx::bridge(namespace = "libtorrent")]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("cpp/alerts/torrent_removed_alert.h");
        include!("lt-rs/src/ffi/mod.rs.h");

        type torrent_removed_alert = crate::ffi::ffi::torrent_removed_alert;
        type InfoHashCpp = crate::ffi::ffi::InfoHashCpp;

        unsafe fn torrent_removed_alert_get_info_hashes(
            a: *mut torrent_removed_alert,
        ) -> InfoHashCpp;
    }
}
