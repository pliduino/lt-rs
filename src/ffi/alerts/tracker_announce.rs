#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type tracker_announce_alert = crate::ffi::ffi::tracker_announce_alert;
    }

    unsafe extern "C++" {
        include!("cpp/alerts/tracker_announce.h");
        unsafe fn tracker_announce_alert_get_event(alert: *mut tracker_announce_alert) -> u8;
        unsafe fn tracker_announce_alert_get_version(alert: *mut tracker_announce_alert) -> u8;
    }
}
