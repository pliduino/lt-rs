#include "./lt.h"

#include "cpp/error.h"
#include "libtorrent/magnet_uri.hpp"
#include "libtorrent-rs/src/ffi/mod.rs.h"
#include <libtorrent/alert_types.hpp>
#include <libtorrent/peer_info.hpp>
#include <libtorrent/read_resume_data.hpp>
#include <libtorrent/settings_pack.hpp>
#include <libtorrent/torrent_handle.hpp>
#include <libtorrent/torrent_info.hpp>
#include <libtorrent/write_resume_data.hpp>

#include <sstream>
#include <string>
#include <vector>

namespace ltrs {
InfoHashCpp info_hash_t_to_info_hash_cpp(const lt::info_hash_t &hash) {
  std::array<std::uint8_t, 32> copied_hash{};
  if (hash.has_v2()) {
    std::memcpy(copied_hash.data(), hash.v2.data(), copied_hash.size());
    return InfoHashCpp{
        2,
        copied_hash,
    };
  } else if (hash.has_v1()) {
    std::memcpy(copied_hash.data(), hash.v1.data(), 20);
    return InfoHashCpp{
        1,
        copied_hash,
    };
  }

  return InfoHashCpp{1, copied_hash};
}

void set_add_torrent_params_storage_mode(lt::add_torrent_params &params, uint8_t mode) {
  params.storage_mode = static_cast<lt::storage_mode_t>(mode);
}
// ╔===========================================================================╗
// ║                                  Session                                  ║
// ╚===========================================================================╝

std::unique_ptr<lt::session> lt_create_session() {
  return std::make_unique<lt::session>();
}

std::unique_ptr<lt::session>
lt_create_session_with_settings(const lt::settings_pack &settings) {
  return std::make_unique<lt::session>(settings);
}

ParseMagnetUriResult lt_parse_magnet_uri(rust::Str magnet_uri) {
  std::string uri(magnet_uri);
  lt::error_code ec;

  auto params = lt::parse_magnet_uri(uri.c_str(), ec);

  if (ec) {
    Error e = error_code_to_error(ec);
    return ParseMagnetUriResult{
        .ok = nullptr,
        .error = e,
    };
  }

  return ParseMagnetUriResult{
      .ok = std::make_unique<lt::add_torrent_params>(std::move(params)),
      .error =
          Error{
              ErrorCategory::NoError,
              0,
          },
  };
}

std::unique_ptr<lt::torrent_handle>
lt_session_add_torrent(lt::session &session, lt::add_torrent_params *params) {
  lt::torrent_handle handle = session.add_torrent(*params);
  return std::make_unique<lt::torrent_handle>(std::move(handle));
}

void lt_session_delete_torrent(lt::session &session,
                               lt::torrent_handle const &handle,
                               uint32_t options) {
  session.remove_torrent(handle, (lt::remove_flags_t)options);
}

rust::string
lt_add_torrent_params_make_magnet_uri(const lt::add_torrent_params &params) {
  std::string magnet = make_magnet_uri(params);
  return rust::string(magnet);
}

rust::Vec<rust::String> lt_add_torrent_params_file_paths(lt::add_torrent_params *params) {
  rust::Vec<rust::String> out;
  if (params == nullptr || !params->ti) {
    return out;
  }
  const auto &fs = params->ti->files();
  out.reserve(params->ti->num_files());
  for (lt::file_index_t i(0); i < lt::file_index_t(params->ti->num_files()); ++i) {
    out.push_back(rust::String(fs.file_path(i)));
  }
  return out;
}

void lt_session_post_torrent_updates(lt::session &session, uint32_t flags) {
  session.post_torrent_updates((lt::status_flags_t)flags);
}

void lt_session_async_add_torrent(lt::session &session,
                                  lt::add_torrent_params *params) {
  session.async_add_torrent(*params);
}

AlertType map_alert_type(int type) {
  switch (type) {
#if TORRENT_ABI_VERSION == 1
  case lt::torrent_added_alert::alert_type:
    return AlertType::TorrentAdded;
#endif
  case lt::torrent_removed_alert::alert_type:
    return AlertType::TorrentRemoved;
  case lt::read_piece_alert::alert_type:
    return AlertType::ReadPiece;
  case lt::file_completed_alert::alert_type:
    return AlertType::FileCompleted;
  case lt::file_renamed_alert::alert_type:
    return AlertType::FileRenamed;
  case lt::file_rename_failed_alert::alert_type:
    return AlertType::FileRenameFailed;
  case lt::performance_alert::alert_type:
    return AlertType::Performance;
  case lt::state_changed_alert::alert_type:
    return AlertType::StateChanged;
  case lt::tracker_error_alert::alert_type:
    return AlertType::TrackerError;
  case lt::tracker_warning_alert::alert_type:
    return AlertType::TrackerWarning;
  case lt::scrape_reply_alert::alert_type:
    return AlertType::ScrapeReply;
  case lt::scrape_failed_alert::alert_type:
    return AlertType::ScrapeFailed;
  case lt::tracker_reply_alert::alert_type:
    return AlertType::TrackerReply;
  case lt::dht_reply_alert::alert_type:
    return AlertType::DhtReply;
  case lt::tracker_announce_alert::alert_type:
    return AlertType::TrackerAnnounce;
  case lt::hash_failed_alert::alert_type:
    return AlertType::HashFailed;
  case lt::peer_ban_alert::alert_type:
    return AlertType::PeerBan;
  case lt::peer_unsnubbed_alert::alert_type:
    return AlertType::PeerUnsnubbed;
  case lt::peer_snubbed_alert::alert_type:
    return AlertType::PeerSnubbed;
  case lt::peer_error_alert::alert_type:
    return AlertType::PeerError;
  case lt::peer_connect_alert::alert_type:
    return AlertType::PeerConnect;
  case lt::peer_disconnected_alert::alert_type:
    return AlertType::PeerDisconnected;
  case lt::invalid_request_alert::alert_type:
    return AlertType::InvalidRequest;
  case lt::torrent_finished_alert::alert_type:
    return AlertType::TorrentFinished;
  case lt::piece_finished_alert::alert_type:
    return AlertType::PieceFinished;
  case lt::request_dropped_alert::alert_type:
    return AlertType::RequestDropped;
  case lt::block_timeout_alert::alert_type:
    return AlertType::BlockTimeout;
  case lt::block_finished_alert::alert_type:
    return AlertType::BlockFinished;
  case lt::block_downloading_alert::alert_type:
    return AlertType::BlockDownloading;
  case lt::unwanted_block_alert::alert_type:
    return AlertType::UnwantedBlock;
  case lt::storage_moved_alert::alert_type:
    return AlertType::StorageMoved;
  case lt::storage_moved_failed_alert::alert_type:
    return AlertType::StorageMovedFailed;
  case lt::torrent_deleted_alert::alert_type:
    return AlertType::TorrentDeleted;
  case lt::torrent_delete_failed_alert::alert_type:
    return AlertType::TorrentDeleteFailed;
  case lt::save_resume_data_alert::alert_type:
    return AlertType::SaveResumeData;
  case lt::save_resume_data_failed_alert::alert_type:
    return AlertType::SaveResumeDataFailed;
  case lt::torrent_paused_alert::alert_type:
    return AlertType::TorrentPaused;
  case lt::torrent_resumed_alert::alert_type:
    return AlertType::TorrentResumed;
  case lt::torrent_checked_alert::alert_type:
    return AlertType::TorrentChecked;
  case lt::url_seed_alert::alert_type:
    return AlertType::UrlSeed;
  case lt::file_error_alert::alert_type:
    return AlertType::FileError;
  case lt::metadata_failed_alert::alert_type:
    return AlertType::MetadataFailed;
  case lt::metadata_received_alert::alert_type:
    return AlertType::MetadataReceived;
  case lt::udp_error_alert::alert_type:
    return AlertType::UdpError;
  case lt::external_ip_alert::alert_type:
    return AlertType::ExternalIp;
  case lt::listen_failed_alert::alert_type:
    return AlertType::ListenFailed;
  case lt::listen_succeeded_alert::alert_type:
    return AlertType::ListenSucceeded;
  case lt::portmap_error_alert::alert_type:
    return AlertType::PortmapError;
  case lt::portmap_alert::alert_type:
    return AlertType::Portmap;
  case lt::portmap_log_alert::alert_type:
    return AlertType::PortmapLog;
  case lt::fastresume_rejected_alert::alert_type:
    return AlertType::FastresumeRejected;
  case lt::peer_blocked_alert::alert_type:
    return AlertType::PeerBlocked;
  case lt::dht_announce_alert::alert_type:
    return AlertType::DhtAnnounce;
  case lt::dht_get_peers_alert::alert_type:
    return AlertType::DhtGetPeers;
#if TORRENT_ABI_VERSION <= 2
  case lt::stats_alert::alert_type:
    return AlertType::Stats;
#endif
  case lt::cache_flushed_alert::alert_type:
    return AlertType::CacheFlushed;
#if TORRENT_ABI_VERSION == 1
  case lt::anonymous_mode_alert::alert_type:
    return AlertType::AnonymousMode;
#endif
  case lt::lsd_peer_alert::alert_type:
    return AlertType::LsdPeer;
  case lt::trackerid_alert::alert_type:
    return AlertType::Trackerid;
  case lt::dht_bootstrap_alert::alert_type:
    return AlertType::DhtBootstrap;
  case lt::torrent_error_alert::alert_type:
    return AlertType::TorrentError;
  case lt::torrent_need_cert_alert::alert_type:
    return AlertType::TorrentNeedCert;
  case lt::incoming_connection_alert::alert_type:
    return AlertType::IncomingConnection;
  case lt::add_torrent_alert::alert_type:
    return AlertType::AddTorrent;
  case lt::state_update_alert::alert_type:
    return AlertType::StateUpdate;
#if TORRENT_ABI_VERSION == 1
  case lt::mmap_cache_alert::alert_type:
    return AlertType::MmapCache;
#endif
  case lt::session_stats_alert::alert_type:
    return AlertType::SessionStats;
  case lt::dht_error_alert::alert_type:
    return AlertType::DhtError;
  case lt::dht_immutable_item_alert::alert_type:
    return AlertType::DhtImmutableItem;
  case lt::dht_mutable_item_alert::alert_type:
    return AlertType::DhtMutableItem;
  case lt::dht_put_alert::alert_type:
    return AlertType::DhtPut;
  case lt::i2p_alert::alert_type:
    return AlertType::I2p;
  case lt::dht_outgoing_get_peers_alert::alert_type:
    return AlertType::DhtOutgoingGetPeers;
  case lt::log_alert::alert_type:
    return AlertType::Log;
  case lt::torrent_log_alert::alert_type:
    return AlertType::TorrentLog;
  case lt::peer_log_alert::alert_type:
    return AlertType::PeerLog;
  case lt::lsd_error_alert::alert_type:
    return AlertType::LsdError;
  case lt::dht_stats_alert::alert_type:
    return AlertType::DhtStats;
  case lt::incoming_request_alert::alert_type:
    return AlertType::IncomingRequest;
  case lt::dht_log_alert::alert_type:
    return AlertType::DhtLog;
  case lt::dht_pkt_alert::alert_type:
    return AlertType::DhtPkt;
  case lt::dht_get_peers_reply_alert::alert_type:
    return AlertType::DhtGetPeersReply;
  case lt::dht_direct_response_alert::alert_type:
    return AlertType::DhtDirectResponse;
  case lt::picker_log_alert::alert_type:
    return AlertType::PickerLog;
  case lt::session_error_alert::alert_type:
    return AlertType::SessionError;
  case lt::dht_live_nodes_alert::alert_type:
    return AlertType::DhtLiveNodes;
  case lt::session_stats_header_alert::alert_type:
    return AlertType::SessionStatsHeader;
  case lt::dht_sample_infohashes_alert::alert_type:
    return AlertType::DhtSampleInfohashes;
  case lt::block_uploaded_alert::alert_type:
    return AlertType::BlockUploaded;
  case lt::alerts_dropped_alert::alert_type:
    return AlertType::AlertsDropped;
  case lt::socks5_alert::alert_type:
    return AlertType::Socks5;
  case lt::file_prio_alert::alert_type:
    return AlertType::FilePrio;
  case lt::oversized_file_alert::alert_type:
    return AlertType::OversizedFile;
  case lt::torrent_conflict_alert::alert_type:
    return AlertType::TorrentConflict;
  case lt::peer_info_alert::alert_type:
    return AlertType::PeerInfo;
  case lt::file_progress_alert::alert_type:
    return AlertType::FileProgress;
  case lt::piece_info_alert::alert_type:
    return AlertType::PieceInfo;
  case lt::piece_availability_alert::alert_type:
    return AlertType::PieceAvailability;
  case lt::tracker_list_alert::alert_type:
    return AlertType::TrackerList;

  default:
    return AlertType::Unknown;
  }
}

rust::Vec<CastAlertRaw> lt_session_pop_alerts(lt::session &ses) {
  std::vector<lt::alert *> alerts;
  ses.pop_alerts(&alerts);
  rust::Vec<CastAlertRaw> cast_alerts;
  cast_alerts.reserve(alerts.size());
  for (auto alert : alerts) {
    cast_alerts.push_back(CastAlertRaw{
        .type_ = map_alert_type(alert->type()),
        .alert = alert,
    });
  }

  return cast_alerts;
}

// ╔===========================================================================╗
// ║                            Add Torrent Params                             ║
// ╚===========================================================================╝

void lt_set_add_torrent_params_path(lt::add_torrent_params *params,
                                    rust::Str path) {
  std::string path_str(path);
  params->save_path = path_str;
}

void lt_set_add_torrent_params_total_uploaded(lt::add_torrent_params *params, int64_t val) { params->total_uploaded = val; }
void lt_set_add_torrent_params_total_downloaded(lt::add_torrent_params *params, int64_t val) { params->total_downloaded = val; }

InfoHashCpp lt_add_torrent_params_info_hash(lt::add_torrent_params *params) {
  const auto hash = params->info_hashes;
  return info_hash_t_to_info_hash_cpp(hash);
}

rust::Vec<uint8_t> lt_write_resume_data_buf(lt::add_torrent_params *params) {
  auto buf = lt::write_resume_data_buf(*params);
  rust::Vec<uint8_t> buf_rust;

  buf_rust.reserve(buf.size());

  // It's so dumb how there's no function to just convert a vector to a
  // rust::Vec... Instead of just copying the pointer and len from buf we have
  // to manually loop like this And due to the files being auto generated you
  // can't simply add it
  // TODO: Eventually change this
  for (auto b : buf) {
    buf_rust.push_back(b);
  }

  return buf_rust;
}

std::unique_ptr<lt::add_torrent_params>
lt_read_resume_data(rust::Slice<const uint8_t> buf) {
  std::vector<char> buf_vec(buf.begin(), buf.end());
  return std::make_unique<lt::add_torrent_params>(
      lt::read_resume_data(buf_vec));
}

// ╔===========================================================================╗
// ║                               Settings Pack                               ║
// ╚===========================================================================╝

std::unique_ptr<lt::settings_pack> lt_create_settings_pack() {
  return std::make_unique<lt::settings_pack>();
}

void lt_set_alert_mask(lt::settings_pack &pack, uint32_t mask) {
  pack.set_int(lt::settings_pack::alert_mask, mask);
}

void lt_set_listen_interfaces(lt::settings_pack &pack, rust::Str value) {
  pack.set_str(lt::settings_pack::listen_interfaces, std::string(value));
}

void lt_set_outgoing_interfaces(lt::settings_pack &pack, rust::Str value) {
  pack.set_str(lt::settings_pack::outgoing_interfaces, std::string(value));
}

void lt_set_proxy_settings(lt::settings_pack &pack,
                           uint8_t proxy_kind,
                           rust::Str hostname,
                           uint16_t port,
                           rust::Str username,
                           rust::Str password,
                           bool authenticated) {
  int type = lt::settings_pack::none;
  if (proxy_kind == 0) {
    type = authenticated ? lt::settings_pack::socks5_pw : lt::settings_pack::socks5;
  } else if (proxy_kind == 1) {
    type = authenticated ? lt::settings_pack::http_pw : lt::settings_pack::http;
  }
  pack.set_int(lt::settings_pack::proxy_type, type);
  pack.set_int(lt::settings_pack::proxy_port, port);
  pack.set_str(lt::settings_pack::proxy_hostname, std::string(hostname));
  pack.set_str(lt::settings_pack::proxy_username, std::string(username));
  pack.set_str(lt::settings_pack::proxy_password, std::string(password));
  pack.set_bool(lt::settings_pack::proxy_hostnames, true);
  pack.set_bool(lt::settings_pack::proxy_peer_connections, true);
  pack.set_bool(lt::settings_pack::proxy_tracker_connections, true);
}

void lt_settings_set_upload_rate_limit(lt::settings_pack &pack, int32_t val) {
    pack.set_int(lt::settings_pack::upload_rate_limit, val);
}
void lt_settings_set_download_rate_limit(lt::settings_pack &pack, int32_t val) {
    pack.set_int(lt::settings_pack::download_rate_limit, val);
}
void lt_settings_set_connections_limit(lt::settings_pack &pack, int32_t val) {
    pack.set_int(lt::settings_pack::connections_limit, val);
}
void lt_settings_set_active_downloads(lt::settings_pack &pack, int32_t val) {
    pack.set_int(lt::settings_pack::active_downloads, val);
}
void lt_settings_set_active_seeds(lt::settings_pack &pack, int32_t val) {
    pack.set_int(lt::settings_pack::active_seeds, val);
}
void lt_settings_set_active_limit(lt::settings_pack &pack, int32_t val) {
    pack.set_int(lt::settings_pack::active_limit, val);
}
void lt_settings_set_seed_time_ratio_limit(lt::settings_pack &pack, int32_t val) {
    pack.set_int(lt::settings_pack::seed_time_ratio_limit, val);
}
void lt_settings_set_seed_time_limit(lt::settings_pack &pack, int32_t val) {
    pack.set_int(lt::settings_pack::seed_time_limit, val);
}
void lt_settings_set_share_ratio_limit(lt::settings_pack &pack, float val) {
    pack.set_int(lt::settings_pack::share_ratio_limit, static_cast<int>(val * 100));
}
void lt_settings_set_max_failcount(lt::settings_pack &pack, int32_t val) {
    pack.set_int(lt::settings_pack::max_failcount, val);
}
void lt_settings_set_dht_enabled(lt::settings_pack &pack, bool val) {
    pack.set_bool(lt::settings_pack::enable_dht, val);
}
void lt_settings_set_lsd_enabled(lt::settings_pack &pack, bool val) {
    pack.set_bool(lt::settings_pack::enable_lsd, val);
}
void lt_settings_set_upnp_enabled(lt::settings_pack &pack, bool val) {
    pack.set_bool(lt::settings_pack::enable_upnp, val);
}
void lt_settings_set_natpmp_enabled(lt::settings_pack &pack, bool val) {
    pack.set_bool(lt::settings_pack::enable_natpmp, val);
}

// ╔===========================================================================╗
// ║                              Torrent Handle                               ║
// ╚===========================================================================╝

bool lt_torrent_handle_in_session(const lt::torrent_handle &handle) {
  return handle.in_session();
}

InfoHashCpp lt_torrent_handle_info_hash(const lt::torrent_handle &handle) {
  const auto hash = handle.info_hashes();
  return info_hash_t_to_info_hash_cpp(hash);
}

std::unique_ptr<lt::torrent_status>
lt_torrent_handle_status(const lt::torrent_handle &handle) {
  return std::make_unique<lt::torrent_status>(handle.status());
}

void lt_torrent_handle_save_resume_data(const lt::torrent_handle &handle,
                                        uint8_t flags) {
  handle.save_resume_data((lt::resume_data_flags_t)flags);
}

void lt_torrent_handle_read_piece(const lt::torrent_handle &handle, int piece) {
  // TODO
  // handle.read_piece((lt::piece_index_t)piece);

  // Just so the compiler shuts up for now
  (void)piece;
  (void)handle;
}

// ╔===========================================================================╗
// ║                              Torrent Status                               ║
// ╚===========================================================================╝

std::unique_ptr<lt::torrent_handle>
lt_torrent_status_handle(lt::torrent_status *status) {
  return std::make_unique<lt::torrent_handle>(status->handle);
}

rust::Str lt_torrent_status_name(lt::torrent_status const &status) {
    return rust::Str(status.name);
}

rust::Str lt_torrent_status_save_path(lt::torrent_status const &status) {
    return rust::Str(status.save_path);
}

uint8_t lt_torrent_status_state(lt::torrent_status *status) {
  return status->state;
}

double lt_torrent_status_progress(lt::torrent_status *status) {
  return status->progress;
}

int64_t lt_torrent_status_all_time_download(lt::torrent_status const &status) {
  return status.all_time_download;
}

int64_t lt_torrent_status_all_time_upload(lt::torrent_status const &status) {
  return status.all_time_upload;
}

int64_t lt_torrent_status_total(lt::torrent_status const &status) {
  return status.total;
}

int32_t lt_torrent_status_download_rate(lt::torrent_status const &status) {
  return status.download_rate;
}

int32_t lt_torrent_status_upload_rate(lt::torrent_status const &status) {
  return status.upload_rate;
}

// ╔===========================================================================╗
// ║                                  Alerts                                   ║
// ╚===========================================================================╝

int lt_alert_type(lt::alert *alert) { return alert->type(); }

// ==========================  Torrent Finished  ===========================
lt::torrent_finished_alert *lt_alert_torrent_finished_cast(lt::alert *alert) {
  return lt::alert_cast<lt::torrent_finished_alert>(alert);
}

std::unique_ptr<lt::torrent_handle>
lt_alert_torrent_finished_handle(lt::torrent_finished_alert *alert) {
  return std::make_unique<lt::torrent_handle>(alert->handle);
}

// =============================  Add Torrent  =============================
lt::add_torrent_alert *lt_alert_add_torrent_cast(lt::alert *alert) {
  return lt::alert_cast<lt::add_torrent_alert>(alert);
}
std::unique_ptr<lt::torrent_handle>
lt_alert_add_torrent_handle(lt::add_torrent_alert *alert) {
  return std::make_unique<lt::torrent_handle>(alert->handle);
}
int lt_alert_add_torrent_error(lt::add_torrent_alert *alert) {
  if (alert->error)
    return alert->error.value();
  return 0;
}
std::unique_ptr<lt::add_torrent_params>
lt_alert_add_torrent_params(lt::add_torrent_alert *alert) {
  return std::make_unique<lt::add_torrent_params>(alert->params);
}

// ============================  State Changed  ============================
lt::state_changed_alert *lt_alert_state_changed_cast(lt::alert *alert) {
  return lt::alert_cast<lt::state_changed_alert>(alert);
}
std::unique_ptr<lt::torrent_handle>
lt_alert_state_changed_handle(lt::state_changed_alert *alert) {
  return std::make_unique<lt::torrent_handle>(alert->handle);
}
uint8_t lt_alert_state_changed_state(lt::state_changed_alert *alert) {
  return alert->state;
}
uint8_t lt_alert_state_changed_prev_state(lt::state_changed_alert *alert) {
  return alert->prev_state;
}

// ============================  State Update  =============================
lt::state_update_alert *lt_alert_state_update_cast(lt::alert *alert) {
  return lt::alert_cast<lt::state_update_alert>(alert);
}
std::unique_ptr<std::vector<lt::torrent_status>>
lt_alert_state_update_status(lt::state_update_alert *alert) {
  return std::make_unique<std::vector<lt::torrent_status>>(alert->status);
}

// ==========================  Save Resume Data  ===========================
lt::save_resume_data_alert *lt_alert_save_resume_data_cast(lt::alert *alert) {
  return lt::alert_cast<lt::save_resume_data_alert>(alert);
}
std::unique_ptr<lt::torrent_handle>
lt_alert_save_resume_data_handle(lt::save_resume_data_alert *alert) {
  return std::make_unique<lt::torrent_handle>(alert->handle);
}
std::unique_ptr<lt::add_torrent_params>
lt_alert_save_resume_data_params(lt::save_resume_data_alert *alert) {
  return std::make_unique<lt::add_torrent_params>(alert->params);
}

// =======================  Save Resume Data Failed  =======================
lt::save_resume_data_failed_alert *
lt_alert_save_resume_data_failed_cast(lt::alert *alert) {
  return lt::alert_cast<lt::save_resume_data_failed_alert>(alert);
}
std::unique_ptr<lt::torrent_handle> lt_alert_save_resume_data_failed_handle(
    lt::save_resume_data_failed_alert *alert) {
  return std::make_unique<lt::torrent_handle>(alert->handle);
}
int lt_alert_save_resume_data_failed_error(
    lt::save_resume_data_failed_alert *alert) {
  if (alert->error)
    return alert->error.value();
  return 0;
}

rust::Vec<InfoHashCpp> lt_session_get_torrent_hashes(lt::session &session) {
    auto handles = session.get_torrents();
    rust::Vec<InfoHashCpp> out;
    out.reserve(handles.size());
    for (auto &h : handles)
        out.push_back(info_hash_t_to_info_hash_cpp(h.info_hashes()));
    return out;
}

std::unique_ptr<lt::torrent_handle> lt_session_find_torrent(lt::session &session, rust::Str info_hash_hex) {
    std::string hex(info_hash_hex);
    auto parse_hex = [&](auto &hash, size_t len) {
        for (size_t i = 0; i < len && i * 2 + 1 < hex.size(); ++i) {
            unsigned int byte;
            std::sscanf(hex.c_str() + i * 2, "%02x", &byte);
            hash[i] = static_cast<unsigned char>(byte);
        }
    };
    if (hex.size() == 64) {
        // v2 SHA-256: session::find_torrent only accepts sha1_hash, so scan all handles
        lt::sha256_hash want;
        parse_hex(want, 32);
        for (auto &th : session.get_torrents()) {
            if (th.info_hashes().has_v2() && th.info_hashes().v2 == want)
                return std::make_unique<lt::torrent_handle>(th);
        }
        return nullptr;
    }
    lt::sha1_hash h;
    parse_hex(h, 20);
    lt::torrent_handle th = session.find_torrent(h);
    if (!th.is_valid()) return nullptr;
    return std::make_unique<lt::torrent_handle>(th);
}

rust::Vec<uint8_t> lt_session_save_state(lt::session &session) {
    auto sp = static_cast<lt::session_handle&>(session).session_state();
    auto buf = lt::write_session_params_buf(sp);
    rust::Vec<uint8_t> out;
    out.reserve(buf.size());
    for (unsigned char c : buf) out.push_back(c);
    return out;
}

void lt_session_load_state(lt::session &session, rust::Slice<const uint8_t> data) {
    auto sp = lt::read_session_params(
        lt::span<const char>(reinterpret_cast<const char*>(data.data()), data.size()));
    static_cast<lt::session_handle&>(session).apply_settings(sp.settings);
}

void lt_session_apply_settings(lt::session &session, const lt::settings_pack &settings) {
    static_cast<lt::session_handle&>(session).apply_settings(settings);
}

TorrentStatusSnapshot lt_torrent_status_snapshot(const lt::torrent_status &s) {
    TorrentStatusSnapshot snap;
    snap.download_rate      = s.download_rate;
    snap.upload_rate        = s.upload_rate;
    snap.all_time_download  = s.all_time_download;
    snap.all_time_upload    = s.all_time_upload;
    snap.total_done         = s.total_done;
    snap.total_wanted       = s.total_wanted;
    snap.total_size         = s.total;
    snap.num_peers          = s.num_peers;
    snap.num_seeds          = s.num_seeds;
    snap.num_complete       = s.num_complete;
    snap.num_incomplete     = s.num_incomplete;
    snap.progress           = s.progress;
    snap.state              = static_cast<uint8_t>(s.state);
    snap.is_seeding         = s.is_seeding;
    snap.is_finished        = s.is_finished;
    snap.is_paused          = static_cast<bool>(s.flags & lt::torrent_flags::paused);
    snap.save_path          = rust::String(s.save_path);
    snap.name               = rust::String(s.name);
    if (s.pieces.size() > 0) {
        size_t piece_count = static_cast<size_t>(s.pieces.size());
        size_t nbytes = (piece_count + 7) / 8;
        snap.pieces.reserve(nbytes);
        for (size_t i = 0; i < nbytes; ++i) {
            uint8_t byte = 0;
            for (size_t b = 0; b < 8 && i * 8 + b < piece_count; ++b)
                if (s.pieces[lt::piece_index_t(static_cast<int>(i * 8 + b))]) byte |= (1u << b);
            snap.pieces.push_back(byte);
        }
    }
    return snap;
}

// ╔===========================================================================╗
// ║                       Add Torrent from .torrent bytes                      ║
// ╚===========================================================================╝

ParseMagnetUriResult lt_add_torrent_params_from_torrent_bytes(rust::Slice<const uint8_t> data) {
    lt::error_code ec;
    std::vector<char> buf(data.begin(), data.end());
    auto ti = std::make_shared<lt::torrent_info>(lt::span<char const>{buf}, ec, lt::from_span);
    if (ec) {
        return ParseMagnetUriResult{
            .ok = nullptr,
            .error = error_code_to_error(ec),
        };
    }
    lt::add_torrent_params params;
    params.ti = ti;
    params.info_hashes = ti->info_hashes();
    return ParseMagnetUriResult{
        .ok = std::make_unique<lt::add_torrent_params>(std::move(params)),
        .error = Error{ ErrorCategory::NoError, 0 },
    };
}

// ╔===========================================================================╗
// ║                           peer_info_alert decode                           ║
// ╚===========================================================================╝

lt::peer_info_alert *lt_alert_peer_info_cast(lt::alert *alert) {
    return lt::alert_cast<lt::peer_info_alert>(alert);
}

std::unique_ptr<lt::torrent_handle> lt_alert_peer_info_handle(lt::peer_info_alert *alert) {
    return std::make_unique<lt::torrent_handle>(alert->handle);
}

rust::Vec<PeerInfoSnapshot> lt_alert_peer_info_peers(lt::peer_info_alert *alert) {
    rust::Vec<PeerInfoSnapshot> out;
    out.reserve(alert->peer_info.size());
    for (auto const &p : alert->peer_info) {
        PeerInfoSnapshot snap;
        // ip:port as "1.2.3.4:6881"
        std::ostringstream ss;
        ss << p.ip.address().to_string() << ":" << p.ip.port();
        snap.ip      = rust::String(ss.str());
        snap.port    = p.ip.port();
        snap.client  = rust::String(p.client);
        snap.down_speed    = p.down_speed;
        snap.up_speed      = p.up_speed;
        snap.total_download = p.total_download;
        snap.total_upload   = p.total_upload;
        snap.progress       = p.progress;
        snap.flags          = static_cast<uint32_t>(p.flags);
        snap.source         = static_cast<uint32_t>(static_cast<std::uint8_t>(p.source));
        snap.country        = rust::String("");
        out.push_back(snap);
    }
    return out;
}

// ╔===========================================================================╗
// ║                         file_renamed_alert decode                          ║
// ╚===========================================================================╝

lt::file_renamed_alert *lt_alert_file_renamed_cast(lt::alert *alert) {
    return lt::alert_cast<lt::file_renamed_alert>(alert);
}
std::unique_ptr<lt::torrent_handle> lt_alert_file_renamed_handle(lt::file_renamed_alert *alert) {
    return std::make_unique<lt::torrent_handle>(alert->handle);
}
int32_t lt_alert_file_renamed_index(lt::file_renamed_alert *alert) {
    return static_cast<int32_t>(alert->index);
}
rust::String lt_alert_file_renamed_new_name(lt::file_renamed_alert *alert) {
    return rust::String(alert->new_name());
}

} // namespace ltrs

namespace boost {
void throw_exception(const std::exception &e,
                     const boost::source_location & /*loc*/) {
  throw e;
}
} // namespace boost

// Removed the old closing brace above - these are additions
