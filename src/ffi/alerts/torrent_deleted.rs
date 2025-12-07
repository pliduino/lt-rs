#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type torrent_deleted_alert = crate::ffi::ffi::torrent_deleted_alert;
    }

    unsafe extern "C++" {
        include!("cpp/alerts/torrent_deleted.h");
        type InfoHashCpp = crate::ffi::ffi::InfoHashCpp;
        unsafe fn torrent_deleted_alert_get_info_hashes(
            alert: *mut torrent_deleted_alert,
        ) -> InfoHashCpp;
    }
}
