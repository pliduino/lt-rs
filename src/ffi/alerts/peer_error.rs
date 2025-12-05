#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type peer_error_alert = crate::ffi::ffi::peer_error_alert;
    }

    unsafe extern "C++" {
        include!("cpp/alerts/peer_error.h");
        type Error = crate::ffi::error::ffi::Error;
        unsafe fn peer_error_alert_get_op(alert: *mut peer_error_alert) -> u8;
        unsafe fn peer_error_alert_get_error(alert: *mut peer_error_alert) -> Error;
    }
}
