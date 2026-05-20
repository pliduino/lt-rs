#include "./lt.h"
#include <libtorrent/torrent_info.hpp>

namespace ltrs {
    // Returns torrent metadata as parallel vectors — avoids cxx struct circular deps.
    // Rust side zips them back into TorrentInfoView.
    rust::String          lt_torrent_handle_name(const lt::torrent_handle &h) {
        auto ti = h.torrent_file(); return ti ? rust::String(ti->name()) : rust::String("");
    }
    int64_t lt_torrent_handle_total_size(const lt::torrent_handle &h) {
        auto ti = h.torrent_file(); return ti ? ti->total_size() : 0;
    }
    int32_t lt_torrent_handle_num_files(const lt::torrent_handle &h) {
        auto ti = h.torrent_file(); return ti ? ti->num_files() : 0;
    }
    int32_t lt_torrent_handle_num_pieces(const lt::torrent_handle &h) {
        auto ti = h.torrent_file(); return ti ? ti->num_pieces() : 0;
    }
    int32_t lt_torrent_handle_piece_length(const lt::torrent_handle &h) {
        auto ti = h.torrent_file(); return ti ? ti->piece_length() : 0;
    }
    rust::Vec<rust::String> lt_torrent_handle_file_paths(const lt::torrent_handle &h) {
        auto ti = h.torrent_file();
        rust::Vec<rust::String> out;
        if (!ti) return out;
        const auto &fs = ti->files();
        out.reserve(ti->num_files());
        for (lt::file_index_t i(0); i < lt::file_index_t(ti->num_files()); ++i)
            out.push_back(rust::String(fs.file_path(i)));
        return out;
    }
    rust::Vec<int64_t> lt_torrent_handle_file_sizes(const lt::torrent_handle &h) {
        auto ti = h.torrent_file();
        rust::Vec<int64_t> out;
        if (!ti) return out;
        const auto &fs = ti->files();
        out.reserve(ti->num_files());
        for (lt::file_index_t i(0); i < lt::file_index_t(ti->num_files()); ++i)
            out.push_back(fs.file_size(i));
        return out;
    }
}
