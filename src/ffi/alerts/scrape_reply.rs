#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type scrape_reply_alert = crate::ffi::ffi::scrape_reply_alert;
    }
    unsafe extern "C++" {
        include!("cpp/alerts/scrape_reply.h");

        unsafe fn scrape_reply_alert_get_version(alert: *mut scrape_reply_alert) -> u8;
        unsafe fn scrape_reply_alert_get_incomplete(alert: *mut scrape_reply_alert) -> i32;
        unsafe fn scrape_reply_alert_get_complete(alert: *mut scrape_reply_alert) -> i32;
    }
}
