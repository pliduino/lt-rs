#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type peer_alert = crate::ffi::ffi::peer_alert;
    }
    unsafe extern "C++" {
        include!("cpp/alerts/peer_alert.h");
        unsafe fn peer_alert_get_pid(alert: *mut peer_alert) -> [u8; 20];
    }
}
