#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type read_piece_alert = crate::ffi::ffi::read_piece_alert;
    }
    unsafe extern "C++" {
        include!("cpp/alerts/read_piece.h");
        include!("lt-rs/src/ffi/error.rs.h");
        type Error = crate::ffi::error::ffi::Error;

        unsafe fn read_piece_alert_get_size(alert: *mut read_piece_alert) -> i32;

        unsafe fn read_piece_alert_get_error(alert: *mut read_piece_alert) -> Error;
    }
}
