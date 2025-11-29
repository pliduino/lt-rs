#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type file_rename_failed_alert = crate::ffi::ffi::file_rename_failed_alert;
    }
    unsafe extern "C++" {
        include!("cpp/alerts/file_rename_failed.h");
        type Error = crate::ffi::error::ffi::Error;

        fn file_rename_failed_alert_get_error(alert: &file_rename_failed_alert) -> Error;
    }
}
