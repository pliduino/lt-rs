#include "./create_torrent.h"
#include <libtorrent/create_torrent.hpp>
#include <libtorrent/bencode.hpp>
#include <libtorrent/entry.hpp>
#include <stdexcept>

namespace ltrs {
    std::unique_ptr<FileStorageWrapper> lt_file_storage_new() {
        return std::make_unique<FileStorageWrapper>();
    }

    void lt_add_files(FileStorageWrapper &fs, rust::Str path) {
        lt::add_files(fs.inner, std::string(path));
    }

    int32_t lt_file_storage_num_files(const FileStorageWrapper &fs) { return fs.inner.num_files(); }
    int64_t lt_file_storage_total_size(const FileStorageWrapper &fs) { return fs.inner.total_size(); }

    std::unique_ptr<CreateTorrentWrapper> lt_create_torrent_new(FileStorageWrapper &fs, int32_t piece_size) {
        return std::make_unique<CreateTorrentWrapper>(fs.inner, piece_size);
    }

    void lt_create_torrent_add_tracker(CreateTorrentWrapper &ct, rust::Str url, int32_t tier) {
        ct.ct->add_tracker(std::string(url), tier);
    }
    void lt_create_torrent_set_comment(CreateTorrentWrapper &ct, rust::Str comment) {
        ct.ct->set_comment(std::string(comment).c_str());
    }
    void lt_create_torrent_set_creator(CreateTorrentWrapper &ct, rust::Str creator) {
        ct.ct->set_creator(std::string(creator).c_str());
    }
    void lt_create_torrent_set_priv(CreateTorrentWrapper &ct, bool priv_flag) {
        ct.ct->set_priv(priv_flag);
    }

    rust::String lt_set_piece_hashes(CreateTorrentWrapper &ct, rust::Str base_path) {
        lt::error_code ec;
        lt::set_piece_hashes(*ct.ct, std::string(base_path), ec);
        if (ec) return rust::String(ec.message());
        return rust::String();
    }

    rust::Vec<uint8_t> lt_create_torrent_generate(CreateTorrentWrapper &ct) {
        lt::entry e = ct.ct->generate();
        std::vector<char> buf;
        lt::bencode(std::back_inserter(buf), e);
        rust::Vec<uint8_t> out;
        out.reserve(buf.size());
        for (unsigned char c : buf) out.push_back(c);
        return out;
    }
}
