#pragma once
#include "libtorrent-rs/src/ffi/mod.rs.h"

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
    void torrent_handle_force_reannounce(const lt::torrent_handle &handle);
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

    // rename
    void torrent_handle_rename_file(const lt::torrent_handle &handle, int32_t file_index, rust::Str new_name);

    // queue position
    int32_t torrent_handle_queue_position(const lt::torrent_handle &handle);
    void    torrent_handle_queue_position_up(const lt::torrent_handle &handle);
    void    torrent_handle_queue_position_down(const lt::torrent_handle &handle);
    void    torrent_handle_queue_position_top(const lt::torrent_handle &handle);
    void    torrent_handle_queue_position_bottom(const lt::torrent_handle &handle);

    // web seeds
    void torrent_handle_add_url_seed(const lt::torrent_handle &handle, rust::Str url);
    void torrent_handle_remove_url_seed(const lt::torrent_handle &handle, rust::Str url);
    rust::Vec<rust::String> torrent_handle_url_seeds(const lt::torrent_handle &handle);

    // piece priorities
    uint8_t              torrent_handle_piece_priority(const lt::torrent_handle &handle, int32_t piece);
    rust::Vec<uint8_t>   torrent_handle_piece_priorities(const lt::torrent_handle &handle);
    void                 torrent_handle_prioritize_pieces(const lt::torrent_handle &handle, rust::Slice<const uint8_t> priorities);
    void                 torrent_handle_set_piece_deadline(const lt::torrent_handle &handle, int32_t piece, int32_t deadline_ms);
    void                 torrent_handle_reset_piece_deadline(const lt::torrent_handle &handle, int32_t piece);
    void                 torrent_handle_clear_piece_deadlines(const lt::torrent_handle &handle);

    // peer info (async: posts a peer_info_alert)
    void torrent_handle_post_peer_info(const lt::torrent_handle &handle);
}
