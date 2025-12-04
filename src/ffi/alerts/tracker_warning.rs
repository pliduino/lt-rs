#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type tracker_warning_alert = crate::ffi::ffi::tracker_warning_alert;
    }
    unsafe extern "C++" {
        include!("cpp/alerts/tracker_warning.h");

        unsafe fn tracker_warning_alert_get_warning_message<'a>(
            alert: *mut tracker_warning_alert,
        ) -> &'a str;

        unsafe fn tracker_warning_alert_get_version(alert: *mut tracker_warning_alert) -> u8;
    }
}
