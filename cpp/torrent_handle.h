#pragma once

#include <libtorrent/torrent_handle.hpp>
#include <cpp/lt.h>

namespace ltrs {
    bool torrent_handle_in_session(const lt::torrent_handle &handle);

    void torrent_handle_read_piece(const lt::torrent_handle &handle, int piece);

    std::unique_ptr<lt::torrent_status>
    torrent_handle_status(const lt::torrent_handle &handle);

    void torrent_handle_save_resume_data(const lt::torrent_handle &handle,
                                            uint8_t flags);

    InfoHashCpp torrent_handle_info_hashes(const lt::torrent_handle &handle);
}
