#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type block_timeout_alert = crate::ffi::ffi::block_timeout_alert;
    }

    unsafe extern "C++" {
        include!("cpp/alerts/block_timeout.h");
        type PieceIndex = crate::ffi::ffi::PieceIndex;
        unsafe fn block_timeout_alert_get_block_index(alert: *mut block_timeout_alert) -> i32;
        unsafe fn block_timeout_alert_get_piece_index(
            alert: *mut block_timeout_alert,
        ) -> PieceIndex;
    }
}
