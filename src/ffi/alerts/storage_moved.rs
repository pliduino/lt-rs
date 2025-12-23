#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type storage_moved_alert = crate::ffi::ffi::storage_moved_alert;
    }

    unsafe extern "C++" {
        include!("cpp/alerts/storage_moved.h");
        unsafe fn storage_moved_alert_get_storage_path<'a>(
            alert: *mut storage_moved_alert,
        ) -> &'a str;
        unsafe fn storage_moved_alert_get_old_path<'a>(alert: *mut storage_moved_alert) -> &'a str;
    }
}
