#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type piece_finished_alert = crate::ffi::ffi::piece_finished_alert;
    }

    unsafe extern "C++" {
        include!("cpp/alerts/piece_finished.h");
        type PieceIndex = crate::ffi::ffi::PieceIndex;
        unsafe fn piece_finished_alert_get_piece_index(
            alert: *mut piece_finished_alert,
        ) -> PieceIndex;
    }
}
