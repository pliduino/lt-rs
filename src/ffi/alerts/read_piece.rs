#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("cpp/alerts/read_piece.h");
    }
}
