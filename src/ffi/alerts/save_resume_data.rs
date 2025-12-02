#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type save_resume_data_alert = crate::ffi::ffi::save_resume_data_alert;
        type add_torrent_params = crate::ffi::ffi::add_torrent_params;
    }
    unsafe extern "C++" {
        include!("cpp/alerts/save_resume_data.h");

        unsafe fn save_resume_data_alert_get_params(
            alert: *mut save_resume_data_alert,
        ) -> *mut add_torrent_params;
    }
}
