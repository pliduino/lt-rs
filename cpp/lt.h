#pragma once
#include "rust/cxx.h"
#include <cpp/error.h>
// Wrapper type headers: complete definitions needed for UniquePtr
#include "cpp/filter.h"
#include "cpp/create_torrent.h"
#include "cpp/torrent_info.h"
#include <libtorrent/add_torrent_params.hpp>
#include <libtorrent/alert.hpp>
#include <libtorrent/alert_types.hpp>
#include <libtorrent/magnet_uri.hpp>
#include <libtorrent/session.hpp>
#include <libtorrent/settings_pack.hpp>
#include <libtorrent/storage_defs.hpp>
#include <libtorrent/torrent_handle.hpp>
#include <boost/throw_exception.hpp>
#include <memory>

namespace ltrs {
// Forward-declare cxx plain structs (defined in mod.rs.h, included before lt.h in bridge .cc)
struct InfoHashCpp;
struct CastAlertRaw;
struct ParseMagnetUriResult;
struct TorrentStatusSnapshot;
struct PeerInfoSnapshot;
struct AddTorrentParamsValues;

// ─── Params ─────────────────────────────────────────────────────────────────
ParseMagnetUriResult lt_parse_magnet_uri(rust::Str magnet_uri);
InfoHashCpp info_hash_t_to_info_hash_cpp(const lt::info_hash_t &hash);
void lt_set_add_torrent_params_path(lt::add_torrent_params *params, rust::Str path);
void lt_set_add_torrent_params_total_uploaded(lt::add_torrent_params *params, int64_t val);
void lt_set_add_torrent_params_total_downloaded(lt::add_torrent_params *params, int64_t val);
InfoHashCpp lt_add_torrent_params_info_hash(lt::add_torrent_params *params);
void set_add_torrent_params_storage_mode(lt::add_torrent_params &params, uint8_t mode);
rust::string lt_add_torrent_params_make_magnet_uri(const lt::add_torrent_params &params);
rust::string lt_torrent_handle_make_magnet_uri(const lt::torrent_handle &handle);
rust::Vec<rust::String> lt_add_torrent_params_file_paths(lt::add_torrent_params *params);
rust::Vec<uint8_t> lt_write_resume_data_buf(lt::add_torrent_params *params);
std::unique_ptr<lt::add_torrent_params> lt_read_resume_data(rust::Slice<const uint8_t> buf);

// ─── Settings ────────────────────────────────────────────────────────────────
std::unique_ptr<lt::settings_pack> lt_create_settings_pack();
void lt_set_alert_mask(lt::settings_pack &pack, uint32_t mask);
void lt_set_listen_interfaces(lt::settings_pack &pack, rust::Str value);
void lt_set_outgoing_interfaces(lt::settings_pack &pack, rust::Str value);
void lt_set_proxy_settings(lt::settings_pack &pack,
                           uint8_t proxy_kind,
                           rust::Str hostname,
                           uint16_t port,
                           rust::Str username,
                           rust::Str password,
                           bool authenticated);
void lt_settings_set_upload_rate_limit(lt::settings_pack &pack, int32_t val);
void lt_settings_set_download_rate_limit(lt::settings_pack &pack, int32_t val);
void lt_settings_set_connections_limit(lt::settings_pack &pack, int32_t val);
void lt_settings_set_active_downloads(lt::settings_pack &pack, int32_t val);
void lt_settings_set_active_seeds(lt::settings_pack &pack, int32_t val);
void lt_settings_set_active_limit(lt::settings_pack &pack, int32_t val);
void lt_settings_set_seed_time_ratio_limit(lt::settings_pack &pack, int32_t val);
void lt_settings_set_seed_time_limit(lt::settings_pack &pack, int32_t val);
void lt_settings_set_share_ratio_limit(lt::settings_pack &pack, float val);
void lt_settings_set_max_failcount(lt::settings_pack &pack, int32_t val);
void lt_settings_set_dht_enabled(lt::settings_pack &pack, bool val);
void lt_settings_set_lsd_enabled(lt::settings_pack &pack, bool val);
void lt_settings_set_upnp_enabled(lt::settings_pack &pack, bool val);
void lt_settings_set_natpmp_enabled(lt::settings_pack &pack, bool val);

// ─── Session ─────────────────────────────────────────────────────────────────
std::unique_ptr<lt::session> lt_create_session();
std::unique_ptr<lt::session> lt_create_session_with_settings(const lt::settings_pack &settings);
std::unique_ptr<lt::torrent_handle> lt_session_add_torrent(lt::session &session, lt::add_torrent_params *params);
void lt_session_delete_torrent(lt::session &session, const lt::torrent_handle &handle, uint32_t options);
void lt_session_async_add_torrent(lt::session &session, lt::add_torrent_params *params);
rust::Vec<CastAlertRaw> lt_session_pop_alerts(lt::session &ses);
void lt_session_post_torrent_updates(lt::session &session, uint32_t flags);
rust::Vec<InfoHashCpp> lt_session_get_torrent_hashes(lt::session &session);
// Look up a single handle by its v1 info-hash string (hex). Returns null UniquePtr if not found.
std::unique_ptr<lt::torrent_handle> lt_session_find_torrent(lt::session &session, rust::Str info_hash_hex);
void lt_session_set_ip_filter(lt::session &s, const IpFilterWrapper &f);
std::unique_ptr<IpFilterWrapper> lt_session_get_ip_filter(lt::session &s);
// Session state save/load (persists DHT routing table, settings etc.)
rust::Vec<uint8_t> lt_session_save_state(lt::session &session);
void lt_session_load_state(lt::session &session, rust::Slice<const uint8_t> data);
// Apply settings to a live session (does not restart listening sockets)
void lt_session_apply_settings(lt::session &session, const lt::settings_pack &settings);

// ─── Torrent Status ──────────────────────────────────────────────────────────
std::unique_ptr<lt::torrent_handle> lt_torrent_status_handle(lt::torrent_status *status);
rust::Str lt_torrent_status_name(lt::torrent_status const &status);
rust::Str lt_torrent_status_save_path(lt::torrent_status const &status);
uint8_t   lt_torrent_status_state(lt::torrent_status *status);
double    lt_torrent_status_progress(lt::torrent_status *status);
int64_t   lt_torrent_status_all_time_download(lt::torrent_status const &status);
int64_t   lt_torrent_status_all_time_upload(lt::torrent_status const &status);
int64_t   lt_torrent_status_total(lt::torrent_status const &status);
int32_t   lt_torrent_status_download_rate(lt::torrent_status const &status);
int32_t   lt_torrent_status_upload_rate(lt::torrent_status const &status);
TorrentStatusSnapshot lt_torrent_status_snapshot(const lt::torrent_status &s);

// ─── Alerts ──────────────────────────────────────────────────────────────────
int lt_alert_type(lt::alert *alert);
lt::torrent_finished_alert *lt_alert_torrent_finished_cast(lt::alert *alert);
std::unique_ptr<lt::torrent_handle> lt_alert_torrent_finished_handle(lt::torrent_finished_alert *alert);
lt::add_torrent_alert *lt_alert_add_torrent_cast(lt::alert *alert);
std::unique_ptr<lt::torrent_handle> lt_alert_add_torrent_handle(lt::add_torrent_alert *alert);
int lt_alert_add_torrent_error(lt::add_torrent_alert *alert);
std::unique_ptr<lt::add_torrent_params> lt_alert_add_torrent_params(lt::add_torrent_alert *alert);
lt::state_update_alert *lt_alert_state_update_cast(lt::alert *alert);
std::unique_ptr<std::vector<lt::torrent_status>> lt_alert_state_update_status(lt::state_update_alert *alert);

// peer_info_alert
lt::peer_info_alert *lt_alert_peer_info_cast(lt::alert *alert);
std::unique_ptr<lt::torrent_handle> lt_alert_peer_info_handle(lt::peer_info_alert *alert);
rust::Vec<PeerInfoSnapshot> lt_alert_peer_info_peers(lt::peer_info_alert *alert);

// file_renamed_alert
lt::file_renamed_alert *lt_alert_file_renamed_cast(lt::alert *alert);
std::unique_ptr<lt::torrent_handle> lt_alert_file_renamed_handle(lt::file_renamed_alert *alert);
int32_t lt_alert_file_renamed_index(lt::file_renamed_alert *alert);
rust::String lt_alert_file_renamed_new_name(lt::file_renamed_alert *alert);

// ─── IpFilter ────────────────────────────────────────────────────────────────
std::unique_ptr<IpFilterWrapper> lt_ip_filter_new();
void     lt_ip_filter_add_rule(IpFilterWrapper &f, rust::Str start, rust::Str end, uint32_t flags);
uint32_t lt_ip_filter_access(const IpFilterWrapper &f, rust::Str addr);

// ─── TorrentInfo / Add from bytes ────────────────────────────────────────────
// Parse a raw .torrent file (bencoded bytes) and return a ready add_torrent_params.
// On error, ok is null and error is set.
ParseMagnetUriResult lt_add_torrent_params_from_torrent_bytes(rust::Slice<const uint8_t> data);


// ─── FileStorage / CreateTorrent ─────────────────────────────────────────────
std::unique_ptr<FileStorageWrapper>   lt_file_storage_new();
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

} // namespace ltrs
