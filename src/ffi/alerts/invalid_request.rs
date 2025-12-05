#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type invalid_request_alert = crate::ffi::ffi::invalid_request_alert;
    }

    unsafe extern "C++" {
        include!("cpp/alerts/invalid_request.h");
        type PeerRequest = crate::ffi::ffi::PeerRequest;

        unsafe fn invalid_request_alert_get_request(
            alert: *mut invalid_request_alert,
        ) -> PeerRequest;
        unsafe fn invalid_request_alert_get_we_have(alert: *mut invalid_request_alert) -> bool;
        unsafe fn invalid_request_alert_get_peer_interested(
            alert: *mut invalid_request_alert,
        ) -> bool;
        unsafe fn invalid_request_alert_get_withheld(alert: *mut invalid_request_alert) -> bool;
    }
}
