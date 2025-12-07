#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type metadata_failed_alert = crate::ffi::ffi::metadata_failed_alert;
    }

    unsafe extern "C++" {
        include!("cpp/alerts/metadata_failed.h");
        type Error = crate::ffi::error::ffi::Error;
        unsafe fn metadata_failed_alert_get_error(alert: *mut metadata_failed_alert) -> Error;
    }
}
