#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type dht_reply_alert = crate::ffi::ffi::dht_reply_alert;
    }

    unsafe extern "C++" {
        include!("cpp/alerts/dht_reply.h");
        unsafe fn dht_reply_alert_get_num_peers(alert: *mut dht_reply_alert) -> i32;
    }
}
