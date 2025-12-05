#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type scrape_failed_alert = crate::ffi::ffi::scrape_failed_alert;
    }

    unsafe extern "C++" {
        include!("cpp/alerts/scrape_failed.h");
        type Error = crate::ffi::error::ffi::Error;
        unsafe fn scrape_failed_alert_get_error_message<'a>(
            alert: *mut scrape_failed_alert,
        ) -> &'a str;
        unsafe fn scrape_failed_alert_get_error(alert: *mut scrape_failed_alert) -> Error;
        unsafe fn scrape_failed_alert_get_version(alert: *mut scrape_failed_alert) -> u8;
    }
}
