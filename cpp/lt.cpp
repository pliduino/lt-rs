#include "./lt.h"

#include "lt-rs/src/ffi/mod.rs.h"
#include <libtorrent/read_resume_data.hpp>
#include <libtorrent/torrent_handle.hpp>
#include <libtorrent/write_resume_data.hpp>

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

  return InfoHashCpp{
      1,
      copied_hash
  };
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

std::unique_ptr<lt::add_torrent_params>
lt_parse_magnet_uri(rust::Str magnet_uri) {
  std::string uri(magnet_uri);
  auto params = lt::parse_magnet_uri(uri.c_str());

  return std::make_unique<lt::add_torrent_params>(std::move(params));
}

std::unique_ptr<lt::torrent_handle>
lt_session_add_torrent(lt::session &session,
                       const lt::add_torrent_params &params) {
  lt::torrent_handle handle = session.add_torrent(params);
  return std::make_unique<lt::torrent_handle>(std::move(handle));
}

void lt_session_post_torrent_updates(lt::session &session, uint32_t flags) {
  session.post_torrent_updates((lt::status_flags_t)flags);
}

void lt_session_async_add_torrent(lt::session &session,
                                  const lt::add_torrent_params &params) {
  session.async_add_torrent(params);
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

void lt_set_add_torrent_params_path(lt::add_torrent_params &params,
                                    rust::Str path) {
  std::string path_str(path);
  params.save_path = path_str;
}

InfoHashCpp
lt_add_torrent_params_info_hash(const lt::add_torrent_params &params) {
  const auto hash = params.info_hashes;
  return info_hash_t_to_info_hash_cpp(hash);
}

rust::Vec<uint8_t>
lt_write_resume_data_buf(const lt::add_torrent_params &params) {
  auto buf = lt::write_resume_data_buf(params);
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
lt_torrent_status_handle(const lt::torrent_status &status) {
  return std::make_unique<lt::torrent_handle>(status.handle);
}

uint8_t lt_torrent_status_state(const lt::torrent_status &status) {
  return status.state;
}

double lt_torrent_status_progress(const lt::torrent_status &status) {
  return status.progress;
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
} // namespace libtorrent

namespace boost {
void throw_exception(const std::exception &e,
                     const boost::source_location & /*loc*/) {
  throw e;
}
} // namespace boost
