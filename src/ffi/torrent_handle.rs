#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type torrent_handle;
        type torrent_status = crate::ffi::ffi::torrent_status;
    }

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
        fn torrent_handle_make_magnet_uri(handle: &torrent_handle) -> String;
        fn torrent_handle_clone(handle: &torrent_handle) -> UniquePtr<torrent_handle>;

        // pause / resume / recheck
        fn torrent_handle_pause(handle: &torrent_handle, flags: u32);
        fn torrent_handle_resume(handle: &torrent_handle);
        fn torrent_handle_force_recheck(handle: &torrent_handle);

        // storage
        fn torrent_handle_move_storage(handle: &torrent_handle, save_path: &str, flags: u32);
        fn torrent_handle_save_path(handle: &torrent_handle) -> String;
        fn torrent_handle_name(handle: &torrent_handle) -> String;

        // limits
        fn torrent_handle_set_upload_limit(handle: &torrent_handle, limit: i32);
        fn torrent_handle_set_download_limit(handle: &torrent_handle, limit: i32);
        fn torrent_handle_upload_limit(handle: &torrent_handle) -> i32;
        fn torrent_handle_download_limit(handle: &torrent_handle) -> i32;

        // flags
        fn torrent_handle_set_flags(handle: &torrent_handle, flags: u64);
        fn torrent_handle_unset_flags(handle: &torrent_handle, flags: u64);
        fn torrent_handle_get_flags(handle: &torrent_handle) -> u64;

        // file priorities
        fn torrent_handle_prioritize_files(handle: &torrent_handle, priorities: &[u8]);
        fn torrent_handle_file_priorities(handle: &torrent_handle) -> Vec<u8>;

        // tracker
        fn torrent_handle_add_tracker(handle: &torrent_handle, url: &str, tier: i32);
        fn torrent_handle_trackers_url(handle: &torrent_handle) -> Vec<String>;
        fn torrent_handle_trackers_tier(handle: &torrent_handle) -> Vec<u8>;
        fn torrent_handle_trackers_fails(handle: &torrent_handle) -> Vec<u8>;
        fn torrent_handle_trackers_working(handle: &torrent_handle) -> Vec<bool>;
        fn torrent_handle_trackers_scrape_complete(handle: &torrent_handle) -> Vec<i32>;
        fn torrent_handle_trackers_scrape_incomplete(handle: &torrent_handle) -> Vec<i32>;
        fn torrent_handle_trackers_scrape_downloaded(handle: &torrent_handle) -> Vec<i32>;
        fn torrent_handle_replace_trackers(handle: &torrent_handle, urls: &[&str]);
        fn torrent_handle_scrape_tracker(handle: &torrent_handle, index: i32);

        // torrent info (parallel vectors, no cxx struct needed)
        fn lt_torrent_handle_name(h: &torrent_handle) -> String;
        fn lt_torrent_handle_total_size(h: &torrent_handle) -> i64;
        fn lt_torrent_handle_num_files(h: &torrent_handle) -> i32;
        fn lt_torrent_handle_num_pieces(h: &torrent_handle) -> i32;
        fn lt_torrent_handle_piece_length(h: &torrent_handle) -> i32;
        fn lt_torrent_handle_file_paths(h: &torrent_handle) -> Vec<String>;
        fn lt_torrent_handle_file_sizes(h: &torrent_handle) -> Vec<i64>;
    }
}
