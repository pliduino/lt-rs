#pragma once
#include "lt-rs/src/ffi/mod.rs.h"

#include <libtorrent/torrent_handle.hpp>

namespace ltrs {
    bool torrent_handle_in_session(const lt::torrent_handle &handle);

    void torrent_handle_read_piece(const lt::torrent_handle &handle, PieceIndex piece);

    std::unique_ptr<lt::torrent_status>
    torrent_handle_status(const lt::torrent_handle &handle);

    void torrent_handle_save_resume_data(const lt::torrent_handle &handle, uint8_t flags);

    rust::string torrent_handle_make_magnet_uri(const lt::torrent_handle &handle);

    InfoHashCpp torrent_handle_info_hashes(const lt::torrent_handle &handle);

    std::unique_ptr<lt::torrent_handle> torrent_handle_clone(const lt::torrent_handle &handle);


    void torrent_handle_pause(const lt::torrent_handle &handle, uint32_t flags);
    void torrent_handle_resume(const lt::torrent_handle &handle);
    void torrent_handle_force_recheck(const lt::torrent_handle &handle);
    void torrent_handle_move_storage(const lt::torrent_handle &handle, rust::Str save_path, uint32_t flags);
    void torrent_handle_set_upload_limit(const lt::torrent_handle &handle, int limit);
    void torrent_handle_set_download_limit(const lt::torrent_handle &handle, int limit);
    int  torrent_handle_upload_limit(const lt::torrent_handle &handle);
    int  torrent_handle_download_limit(const lt::torrent_handle &handle);
    void torrent_handle_set_flags(const lt::torrent_handle &handle, uint64_t flags);
    void torrent_handle_unset_flags(const lt::torrent_handle &handle, uint64_t flags);
    uint64_t torrent_handle_get_flags(const lt::torrent_handle &handle);
    void torrent_handle_prioritize_files(const lt::torrent_handle &handle, rust::Slice<const uint8_t> priorities);
    rust::Vec<uint8_t> torrent_handle_file_priorities(const lt::torrent_handle &handle);
    void torrent_handle_add_tracker(const lt::torrent_handle &handle, rust::Str url, int tier);
    rust::String torrent_handle_save_path(const lt::torrent_handle &handle);
    rust::String torrent_handle_name(const lt::torrent_handle &handle);

    // trackers
    rust::Vec<rust::String> torrent_handle_trackers_url(const lt::torrent_handle &handle);
    rust::Vec<uint8_t>      torrent_handle_trackers_tier(const lt::torrent_handle &handle);
    rust::Vec<uint8_t>      torrent_handle_trackers_fails(const lt::torrent_handle &handle);
    rust::Vec<bool>         torrent_handle_trackers_working(const lt::torrent_handle &handle);
    rust::Vec<int32_t>      torrent_handle_trackers_scrape_complete(const lt::torrent_handle &handle);
    rust::Vec<int32_t>      torrent_handle_trackers_scrape_incomplete(const lt::torrent_handle &handle);
    rust::Vec<int32_t>      torrent_handle_trackers_scrape_downloaded(const lt::torrent_handle &handle);
    void torrent_handle_replace_trackers(const lt::torrent_handle &handle, rust::Slice<const rust::Str> urls);
    void torrent_handle_scrape_tracker(const lt::torrent_handle &handle, int32_t index);
}
