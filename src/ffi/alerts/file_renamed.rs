#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type file_renamed_alert = crate::ffi::ffi::file_renamed_alert;
    }
    unsafe extern "C++" {
        include!("cpp/alerts/file_renamed.h");

        unsafe fn file_renamed_alert_get_old_name<'a>(alert: *mut file_renamed_alert) -> &'a str;
        unsafe fn file_renamed_alert_get_new_name<'a>(alert: *mut file_renamed_alert) -> &'a str;
    }
}
