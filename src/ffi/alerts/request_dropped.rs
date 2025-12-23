#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type request_dropped_alert = crate::ffi::ffi::request_dropped_alert;
    }

    unsafe extern "C++" {
        include!("cpp/alerts/request_dropped.h");
        type PieceIndex = crate::ffi::ffi::PieceIndex;
        unsafe fn request_dropped_alert_get_block_index(alert: *mut request_dropped_alert) -> i32;
        unsafe fn request_dropped_alert_get_piece_index(
            alert: *mut request_dropped_alert,
        ) -> PieceIndex;
    }
}
