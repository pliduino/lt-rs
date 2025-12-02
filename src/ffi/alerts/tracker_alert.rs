#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type tracker_alert = crate::ffi::ffi::tracker_alert;
    }
    unsafe extern "C++" {
        include!("cpp/alerts/tracker_alert.h");

        unsafe fn tracker_alert_get_tracker_url<'a>(alert: *mut tracker_alert) -> &'a str;
    }
}
