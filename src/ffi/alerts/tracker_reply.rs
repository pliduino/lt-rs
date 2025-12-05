#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type tracker_reply_alert = crate::ffi::ffi::tracker_reply_alert;
    }

    unsafe extern "C++" {
        include!("cpp/alerts/tracker_reply.h");
        unsafe fn tracker_reply_alert_get_num_peers(alert: *mut tracker_reply_alert) -> i32;
        unsafe fn tracker_reply_alert_get_version(alert: *mut tracker_reply_alert) -> u8;
    }
}
