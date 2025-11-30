#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type state_changed_alert = crate::ffi::ffi::state_changed_alert;
    }
    unsafe extern "C++" {
        include!("cpp/alerts/state_changed.h");

        unsafe fn state_changed_alert_get_state(alert: *mut state_changed_alert) -> u8;
        unsafe fn state_changed_alert_get_prev_state(alert: *mut state_changed_alert) -> u8;
    }
}
