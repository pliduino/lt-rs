#pragma once
#include "rust/cxx.h"
#include <libtorrent/create_torrent.hpp>
#include <libtorrent/file_storage.hpp>
#include <memory>

namespace ltrs {
    struct FileStorageWrapper { lt::file_storage inner; };

    // Owns a copy of file_storage and the create_torrent that references it.
    // Non-copyable/moveable to keep the internal reference stable.
    struct CreateTorrentWrapper {
        lt::file_storage fs;
        std::unique_ptr<lt::create_torrent> ct;
        CreateTorrentWrapper(const lt::file_storage &src, int piece_size)
            : fs(src) { ct = std::make_unique<lt::create_torrent>(this->fs, piece_size); }
        CreateTorrentWrapper(const CreateTorrentWrapper&) = delete;
        CreateTorrentWrapper& operator=(const CreateTorrentWrapper&) = delete;
    };

    std::unique_ptr<FileStorageWrapper> lt_file_storage_new();
    void    lt_add_files(FileStorageWrapper &fs, rust::Str path);
    int32_t lt_file_storage_num_files(const FileStorageWrapper &fs);
    int64_t lt_file_storage_total_size(const FileStorageWrapper &fs);

    std::unique_ptr<CreateTorrentWrapper> lt_create_torrent_new(FileStorageWrapper &fs, int32_t piece_size);
    void lt_create_torrent_add_tracker(CreateTorrentWrapper &ct, rust::Str url, int32_t tier);
    void lt_create_torrent_set_comment(CreateTorrentWrapper &ct, rust::Str comment);
    void lt_create_torrent_set_creator(CreateTorrentWrapper &ct, rust::Str creator);
    void lt_create_torrent_set_priv(CreateTorrentWrapper &ct, bool priv_flag);
    rust::String lt_set_piece_hashes(CreateTorrentWrapper &ct, rust::Str base_path);
    rust::Vec<uint8_t> lt_create_torrent_generate(CreateTorrentWrapper &ct);
}
