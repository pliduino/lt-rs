#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type file_completed_alert = crate::ffi::ffi::file_completed_alert;
    }
    unsafe extern "C++" {
        include!("cpp/alerts/file_completed.h");
    }
}
