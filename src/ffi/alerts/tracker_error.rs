#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type tracker_error_alert = crate::ffi::ffi::tracker_error_alert;
    }
    unsafe extern "C++" {
        include!("cpp/alerts/tracker_error.h");
        include!("cpp/error.h");
        type Error = crate::ffi::error::ffi::Error;

        unsafe fn tracker_error_alert_get_failure_reason<'a>(
            a: *mut tracker_error_alert,
        ) -> &'a str;
        unsafe fn tracker_error_alert_get_times_in_row(a: *mut tracker_error_alert) -> i32;
        unsafe fn tracker_error_alert_get_error(a: *mut tracker_error_alert) -> Error;
        unsafe fn tracker_error_alert_get_op(a: *mut tracker_error_alert) -> u8;
        unsafe fn tracker_error_alert_get_version(a: *mut tracker_error_alert) -> u8;
    }
}
