#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type torrent_alert = crate::ffi::ffi::torrent_alert;
        type torrent_handle = crate::ffi::torrent_handle::ffi::torrent_handle;
    }
    unsafe extern "C++" {
        include!("cpp/alerts/torrent_alert.h");
        unsafe fn torrent_alert_handle(alert: *mut torrent_alert) -> UniquePtr<torrent_handle>;
        unsafe fn torrent_alert_message(alert: *const torrent_alert) -> String;
        unsafe fn torrent_alert_torrent_name<'a>(alert: *const torrent_alert) -> &'a str;
    }
}
