#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type hash_failed_alert = crate::ffi::ffi::hash_failed_alert;
    }

    unsafe extern "C++" {
        include!("cpp/alerts/hash_failed.h");
        unsafe fn hash_failed_alert_get_piece_index(alert: *mut hash_failed_alert) -> i32;
    }
}
