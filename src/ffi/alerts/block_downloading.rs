#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type block_downloading_alert = crate::ffi::ffi::block_downloading_alert;
    }

    unsafe extern "C++" {
        include!("cpp/alerts/block_downloading.h");
        type PieceIndex = crate::ffi::ffi::PieceIndex;
        unsafe fn block_downloading_alert_get_block_index(
            alert: *mut block_downloading_alert,
        ) -> i32;
        unsafe fn block_downloading_alert_get_piece_index(
            alert: *mut block_downloading_alert,
        ) -> PieceIndex;
    }
}
