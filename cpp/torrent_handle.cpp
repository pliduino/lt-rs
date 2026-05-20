#include "./torrent_handle.h"

namespace ltrs {
    bool torrent_handle_in_session(const lt::torrent_handle &handle) {
      return handle.in_session();
    }

    InfoHashCpp torrent_handle_info_hashes(const lt::torrent_handle &handle) {
      const auto hash = handle.info_hashes();
      return info_hash_t_to_info_hash_cpp(hash);
    }

    std::unique_ptr<lt::torrent_status>
    torrent_handle_status(const lt::torrent_handle &handle) {
      return std::make_unique<lt::torrent_status>(handle.status());
    }

    rust::string torrent_handle_make_magnet_uri(const lt::torrent_handle &handle) {
        std::string magnet = make_magnet_uri(handle);
        return rust::string(magnet);
    }

    void torrent_handle_save_resume_data(const lt::torrent_handle &handle,
                                            uint8_t flags) {
      handle.save_resume_data((lt::resume_data_flags_t)flags);
    }

    void torrent_handle_read_piece(const lt::torrent_handle &handle, PieceIndex piece) {
       handle.read_piece(lt::piece_index_t(piece.inner));
    }

    std::unique_ptr<lt::torrent_handle> torrent_handle_clone(const lt::torrent_handle &handle) {
        auto new_handle = std::make_unique<lt::torrent_handle>(lt::torrent_handle(handle));
        return new_handle;
    }

    void torrent_handle_pause(const lt::torrent_handle &handle, uint32_t flags) {
        handle.pause(static_cast<lt::pause_flags_t>(flags));
    }

    void torrent_handle_resume(const lt::torrent_handle &handle) {
        handle.resume();
    }

    void torrent_handle_force_recheck(const lt::torrent_handle &handle) {
        handle.force_recheck();
    }

    void torrent_handle_move_storage(const lt::torrent_handle &handle, rust::Str save_path, uint32_t flags) {
        handle.move_storage(std::string(save_path), static_cast<lt::move_flags_t>(flags));
    }

    void torrent_handle_set_upload_limit(const lt::torrent_handle &handle, int limit) {
        handle.set_upload_limit(limit);
    }

    void torrent_handle_set_download_limit(const lt::torrent_handle &handle, int limit) {
        handle.set_download_limit(limit);
    }

    int torrent_handle_upload_limit(const lt::torrent_handle &handle) {
        return handle.upload_limit();
    }

    int torrent_handle_download_limit(const lt::torrent_handle &handle) {
        return handle.download_limit();
    }

    void torrent_handle_set_flags(const lt::torrent_handle &handle, uint64_t flags) {
        handle.set_flags(lt::torrent_flags_t(flags));
    }

    void torrent_handle_unset_flags(const lt::torrent_handle &handle, uint64_t flags) {
        handle.unset_flags(lt::torrent_flags_t(flags));
    }

    uint64_t torrent_handle_get_flags(const lt::torrent_handle &handle) {
        return static_cast<uint64_t>(handle.flags());
    }

    void torrent_handle_prioritize_files(const lt::torrent_handle &handle, rust::Slice<const uint8_t> priorities) {
        std::vector<lt::download_priority_t> prios;
        prios.reserve(priorities.size());
        for (auto p : priorities) prios.push_back(lt::download_priority_t(p));
        handle.prioritize_files(prios);
    }

    rust::Vec<uint8_t> torrent_handle_file_priorities(const lt::torrent_handle &handle) {
        auto prios = handle.get_file_priorities();
        rust::Vec<uint8_t> out;
        out.reserve(prios.size());
        for (auto p : prios) out.push_back(static_cast<uint8_t>(p));
        return out;
    }

    void torrent_handle_add_tracker(const lt::torrent_handle &handle, rust::Str url, int tier) {
        std::string url_str(url);  // avoid most-vexing-parse
        lt::announce_entry ae{url_str};
        ae.tier = static_cast<uint8_t>(tier);
        handle.add_tracker(ae);
    }

    rust::String torrent_handle_save_path(const lt::torrent_handle &handle) {
        return rust::String(handle.status(lt::torrent_handle::query_save_path).save_path);
    }

    rust::String torrent_handle_name(const lt::torrent_handle &handle) {
        return rust::String(handle.status(lt::torrent_handle::query_name).name);
    }

    rust::Vec<rust::String> torrent_handle_trackers_url(const lt::torrent_handle &handle) {
        auto t = handle.trackers(); rust::Vec<rust::String> out; out.reserve(t.size());
        for (auto &ae : t) out.push_back(rust::String(ae.url)); return out;
    }
    rust::Vec<uint8_t> torrent_handle_trackers_tier(const lt::torrent_handle &handle) {
        auto t = handle.trackers(); rust::Vec<uint8_t> out; out.reserve(t.size());
        for (auto &ae : t) out.push_back(ae.tier); return out;
    }
    // In libtorrent 2.0+, scrape/fail data is on announce_endpoint.info_hashes[V1].
    // Structure: announce_entry → endpoints[] → endpoint.info_hashes[protocol_version::V1]
    static const lt::announce_infohash* ep_ih(const lt::announce_entry &ae) {
        for (auto &ep : ae.endpoints) return &ep.info_hashes[lt::protocol_version::V1];
        return nullptr;
    }
    rust::Vec<uint8_t> torrent_handle_trackers_fails(const lt::torrent_handle &handle) {
        auto t = handle.trackers(); rust::Vec<uint8_t> out; out.reserve(t.size());
        for (auto &ae : t) { auto *i = ep_ih(ae); out.push_back(i ? i->fails : 0); }
        return out;
    }
    rust::Vec<bool> torrent_handle_trackers_working(const lt::torrent_handle &handle) {
        auto t = handle.trackers(); rust::Vec<bool> out; out.reserve(t.size());
        for (auto &ae : t) { auto *i = ep_ih(ae); out.push_back(i && i->fails == 0 && !i->updating); }
        return out;
    }
    rust::Vec<int32_t> torrent_handle_trackers_scrape_complete(const lt::torrent_handle &handle) {
        auto t = handle.trackers(); rust::Vec<int32_t> out; out.reserve(t.size());
        for (auto &ae : t) { auto *i = ep_ih(ae); out.push_back(i ? i->scrape_complete : -1); }
        return out;
    }
    rust::Vec<int32_t> torrent_handle_trackers_scrape_incomplete(const lt::torrent_handle &handle) {
        auto t = handle.trackers(); rust::Vec<int32_t> out; out.reserve(t.size());
        for (auto &ae : t) { auto *i = ep_ih(ae); out.push_back(i ? i->scrape_incomplete : -1); }
        return out;
    }
    rust::Vec<int32_t> torrent_handle_trackers_scrape_downloaded(const lt::torrent_handle &handle) {
        auto t = handle.trackers(); rust::Vec<int32_t> out; out.reserve(t.size());
        for (auto &ae : t) { auto *i = ep_ih(ae); out.push_back(i ? i->scrape_downloaded : -1); }
        return out;
    }

    void torrent_handle_replace_trackers(const lt::torrent_handle &handle, rust::Slice<const rust::Str> urls) {
        std::vector<lt::announce_entry> entries;
        entries.reserve(urls.size());
        for (size_t i = 0; i < urls.size(); ++i) {
            std::string url_str(urls[i]);
            lt::announce_entry ae{url_str};
            ae.tier = static_cast<uint8_t>(i);
            entries.push_back(ae);
        }
        handle.replace_trackers(entries);
    }

    void torrent_handle_scrape_tracker(const lt::torrent_handle &handle, int32_t index) {
        handle.scrape_tracker(index);
    }
}
