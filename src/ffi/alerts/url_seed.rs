#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type url_seed_alert = crate::ffi::ffi::url_seed_alert;
    }

    unsafe extern "C++" {
        include!("cpp/alerts/url_seed.h");
        type Error = crate::ffi::error::ffi::Error;
        unsafe fn url_seed_alert_get_server_url<'a>(alert: *mut url_seed_alert) -> &'a str;
        unsafe fn url_seed_alert_get_error_message<'a>(alert: *mut url_seed_alert) -> &'a str;
        unsafe fn url_seed_alert_get_error(alert: *mut url_seed_alert) -> Error;
    }
}
