#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type add_torrent_alert = crate::ffi::ffi::add_torrent_alert;
        type add_torrent_params = crate::ffi::ffi::add_torrent_params;
    }
    unsafe extern "C++" {
        include!("cpp/alerts/add_torrent.h");
        type Error = crate::ffi::error::ffi::Error;

        unsafe fn add_torrent_alert_get_error(alert: *mut add_torrent_alert) -> Error;
        unsafe fn add_torrent_alert_get_add_torrent_params(
            alert: *mut add_torrent_alert,
        ) -> *mut add_torrent_params;
    }
}
