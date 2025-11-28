#[cxx::bridge(namespace = "libtorrent")]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("cpp/alerts/torrent_alert.h");
        type torrent_alert = crate::ffi::ffi::torrent_alert;
        type torrent_handle = crate::ffi::ffi::torrent_handle;

        unsafe fn lt_torrent_alert_handle(alert: *mut torrent_alert) -> *mut torrent_handle;
        unsafe fn lt_torrent_alert_message(alert: *const torrent_alert) -> String;
        unsafe fn lt_torrent_alert_torrent_name<'a>(alert: *const torrent_alert) -> &'a str;
    }
}
