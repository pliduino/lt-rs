#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type torrent_error_alert = crate::ffi::ffi::torrent_error_alert;
    }
    unsafe extern "C++" {
        include!("cpp/alerts/torrent_error.h");
        type Error = crate::ffi::error::ffi::Error;

        unsafe fn torrent_error_alert_get_filename<'a>(alert: *mut torrent_error_alert) -> &'a str;
        unsafe fn torrent_error_alert_get_error(alert: *mut torrent_error_alert) -> Error;
    }
}
