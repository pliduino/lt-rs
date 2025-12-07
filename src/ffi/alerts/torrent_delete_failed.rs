#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type torrent_delete_failed_alert = crate::ffi::ffi::torrent_delete_failed_alert;
    }

    unsafe extern "C++" {
        include!("cpp/alerts/torrent_delete_failed.h");
        type Error = crate::ffi::error::ffi::Error;
        type InfoHashCpp = crate::ffi::ffi::InfoHashCpp;

        unsafe fn torrent_delete_failed_alert_get_error(
            alert: *mut torrent_delete_failed_alert,
        ) -> Error;
        unsafe fn torrent_delete_failed_alert_get_info_hashes(
            alert: *mut torrent_delete_failed_alert,
        ) -> InfoHashCpp;
    }
}
