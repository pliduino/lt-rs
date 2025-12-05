#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type peer_connect_alert = crate::ffi::ffi::peer_connect_alert;
    }

    unsafe extern "C++" {
        include!("cpp/alerts/peer_connect.h");
        unsafe fn peer_connect_alert_get_direction(alert: *mut peer_connect_alert) -> u8;
        unsafe fn peer_connect_alert_get_socket_type(alert: *mut peer_connect_alert) -> u8;
    }
}
