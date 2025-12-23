#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type file_error_alert = crate::ffi::ffi::file_error_alert;
    }

    unsafe extern "C++" {
        include!("cpp/alerts/file_error.h");
        type Error = crate::ffi::error::ffi::Error;

        unsafe fn file_error_alert_get_filename<'a>(alert: *mut file_error_alert) -> &'a str;
        unsafe fn file_error_alert_get_error(alert: *mut file_error_alert) -> Error;
        unsafe fn file_error_alert_get_op(alert: *mut file_error_alert) -> u8;
    }
}
