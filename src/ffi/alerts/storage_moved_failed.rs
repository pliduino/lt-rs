#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type storage_moved_failed_alert = crate::ffi::ffi::storage_moved_failed_alert;
    }

    unsafe extern "C++" {
        include!("cpp/alerts/storage_moved_failed.h");
        type Error = crate::ffi::error::ffi::Error;
        unsafe fn storage_moved_failed_alert_file_path<'a>(
            alert: *mut storage_moved_failed_alert,
        ) -> &'a str;
        unsafe fn storage_moved_failed_alert_get_error(
            alert: *mut storage_moved_failed_alert,
        ) -> Error;
        unsafe fn storage_moved_failed_alert_get_op(alert: *mut storage_moved_failed_alert) -> u8;
    }
}
