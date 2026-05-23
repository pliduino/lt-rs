#pragma once
#include "rust/cxx.h"
#include <libtorrent/torrent_handle.hpp>

namespace ltrs {
    rust::String          lt_torrent_handle_name(const lt::torrent_handle &h);
    int64_t               lt_torrent_handle_total_size(const lt::torrent_handle &h);
    int32_t               lt_torrent_handle_num_files(const lt::torrent_handle &h);
    int32_t               lt_torrent_handle_num_pieces(const lt::torrent_handle &h);
    int32_t               lt_torrent_handle_piece_length(const lt::torrent_handle &h);
    rust::Vec<rust::String> lt_torrent_handle_file_paths(const lt::torrent_handle &h);
    rust::Vec<int64_t>    lt_torrent_handle_file_sizes(const lt::torrent_handle &h);
}
