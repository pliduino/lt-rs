#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type peer_disconnected_alert = crate::ffi::ffi::peer_disconnected_alert;
    }

    unsafe extern "C++" {
        include!("cpp/alerts/peer_disconnected.h");
        type Error = crate::ffi::error::ffi::Error;

        unsafe fn peer_disconnected_alert_get_socket_type(
            alert: *mut peer_disconnected_alert,
        ) -> u8;
        unsafe fn peer_disconnected_alert_get_op(alert: *mut peer_disconnected_alert) -> u8;
        unsafe fn peer_disconnected_alert_get_error(alert: *mut peer_disconnected_alert) -> Error;
        unsafe fn peer_disconnected_alert_get_reason(alert: *mut peer_disconnected_alert) -> u16;
    }
}
