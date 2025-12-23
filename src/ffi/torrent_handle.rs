#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type torrent_handle;
        type torrent_status = crate::ffi::ffi::torrent_status;
    }

    // We need this otherwise the macro doesn't instantiate UniquePtrTarget
    impl UniquePtr<torrent_handle> {}

    unsafe extern "C++" {
        include!("cpp/torrent_handle.h");
        include!("lt-rs/src/ffi/mod.rs.h");
        type InfoHashCpp = crate::ffi::ffi::InfoHashCpp;
        type PieceIndex = crate::ffi::ffi::PieceIndex;

        fn torrent_handle_in_session(handle: &torrent_handle) -> bool;
        fn torrent_handle_read_piece(handle: &torrent_handle, piece: PieceIndex);
        fn torrent_handle_status(handle: &torrent_handle) -> UniquePtr<torrent_status>;
        fn torrent_handle_save_resume_data(handle: &torrent_handle, flags: u8);

        fn torrent_handle_info_hashes(handle: &torrent_handle) -> InfoHashCpp;
    }
}
