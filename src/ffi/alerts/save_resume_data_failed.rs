#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type save_resume_data_failed_alert = crate::ffi::ffi::save_resume_data_failed_alert;
    }
    unsafe extern "C++" {
        include!("cpp/alerts/save_resume_data_failed.h");
        type Error = crate::ffi::error::ffi::Error;

        unsafe fn save_resume_data_failed_alert_get_error(
            alert: *mut save_resume_data_failed_alert,
        ) -> Error;
    }
}
