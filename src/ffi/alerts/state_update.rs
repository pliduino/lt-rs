#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {

    #[namespace = "libtorrent"]
    extern "C++" {
        type state_update_alert = crate::ffi::ffi::state_update_alert;
        type torrent_status = crate::ffi::ffi::torrent_status;
    }
    unsafe extern "C++" {
        include!("cpp/alerts/state_update.h");

        unsafe fn state_update_alert_get_status(
            alert: *mut state_update_alert,
        ) -> *mut CxxVector<torrent_status>;
    }
}
