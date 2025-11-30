#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type performance_alert = crate::ffi::ffi::performance_alert;
    }
    unsafe extern "C++" {
        include!("cpp/alerts/performance.h");

        unsafe fn performance_alert_get_warning_code(alert: *mut performance_alert) -> u8;
    }
}
