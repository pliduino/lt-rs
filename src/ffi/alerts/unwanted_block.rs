#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type unwanted_block_alert = crate::ffi::ffi::unwanted_block_alert;
    }

    unsafe extern "C++" {
        include!("cpp/alerts/unwanted_block.h");
        type PieceIndex = crate::ffi::ffi::PieceIndex;
        unsafe fn unwanted_block_alert_get_block_index(alert: *mut unwanted_block_alert) -> i32;
        unsafe fn unwanted_block_alert_get_piece_index(
            alert: *mut unwanted_block_alert,
        ) -> PieceIndex;
    }
}
