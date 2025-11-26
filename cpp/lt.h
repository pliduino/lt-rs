#pragma once
#define BOOST_ASIO_SEPARATE_COMPILATION 1
#define TORRENT_NO_DEPRECATE 1
#include "rust/cxx.h"

#include <libtorrent/add_torrent_params.hpp>
#include <libtorrent/alert_types.hpp>
#include <libtorrent/magnet_uri.hpp>
#include <libtorrent/session.hpp>

#include <boost/throw_exception.hpp>
#include <memory>

namespace libtorrent {
struct InfoHashCpp;
struct CastAlertRaw;

std::unique_ptr<lt::add_torrent_params>
lt_parse_magnet_uri(rust::Str magnet_uri);

void lt_set_add_torrent_params_path(add_torrent_params &params, rust::Str path);

InfoHashCpp
lt_add_torrent_params_info_hash(const lt::add_torrent_params &params);

rust::Vec<uint8_t>
lt_write_resume_data_buf(const lt::add_torrent_params &params);

std::unique_ptr<lt::add_torrent_params>
lt_read_resume_data(rust::Slice<const uint8_t> buf);

std::unique_ptr<lt::settings_pack> lt_create_settings_pack();
void lt_set_alert_mask(lt::settings_pack &pack, uint32_t mask);

// ╔===========================================================================╗
// ║                                  Session                                  ║
// ╚===========================================================================╝

std::unique_ptr<lt::session> lt_create_session();

std::unique_ptr<lt::session>
lt_create_session_with_settings(const lt::settings_pack &settings);

std::unique_ptr<lt::torrent_handle>
lt_session_add_torrent(session &session, const lt::add_torrent_params &params);

rust::Vec<CastAlertRaw> lt_session_pop_alerts(lt::session &ses);

void lt_session_async_add_torrent(session &session,
                                  const lt::add_torrent_params &params);
void lt_session_post_torrent_updates(lt::session &session, uint32_t flags);

// ╔===========================================================================╗
// ║                              Torrent Handle                               ║
// ╚===========================================================================╝

bool lt_torrent_handle_in_session(const lt::torrent_handle &handle);

void lt_torrent_handle_read_piece(const lt::torrent_handle &handle, int piece);

std::unique_ptr<torrent_status>
lt_torrent_handle_status(const lt::torrent_handle &handle);

void lt_torrent_handle_save_resume_data(const lt::torrent_handle &handle,
                                        uint8_t flags);

InfoHashCpp lt_torrent_handle_info_hash(const lt::torrent_handle &handle);

// ╔===========================================================================╗
// ║                              Torrent Status                               ║
// ╚===========================================================================╝

std::unique_ptr<lt::torrent_handle>
lt_torrent_status_handle(const lt::torrent_status &status);

uint8_t lt_torrent_status_state(const lt::torrent_status &status);

double lt_torrent_status_progress(const lt::torrent_status &status);

// ╔===========================================================================╗
// ║                                  Alerts                                   ║
// ╚===========================================================================╝

int lt_alert_type(lt::alert *alert);

// ==========================  Torrent Finished  ===========================
lt::torrent_finished_alert *lt_alert_torrent_finished_cast(lt::alert *alert);

std::unique_ptr<lt::torrent_handle>
lt_alert_torrent_finished_handle(lt::torrent_finished_alert *alert);

// =============================  Add Torrent  =============================
lt::add_torrent_alert *lt_alert_add_torrent_cast(lt::alert *alert);

std::unique_ptr<lt::torrent_handle>
lt_alert_add_torrent_handle(lt::add_torrent_alert *alert);

int lt_alert_add_torrent_error(lt::add_torrent_alert *alert);

std::unique_ptr<lt::add_torrent_params>
lt_alert_add_torrent_params(lt::add_torrent_alert *alert);

// ============================  State Changed  ============================
lt::state_changed_alert *lt_alert_state_changed_cast(lt::alert *alert);

std::unique_ptr<lt::torrent_handle>
lt_alert_state_changed_handle(lt::state_changed_alert *alert);

uint8_t lt_alert_state_changed_state(lt::state_changed_alert *alert);

uint8_t lt_alert_state_changed_prev_state(lt::state_changed_alert *alert);

// ============================  State Update  =============================
lt::state_update_alert *lt_alert_state_update_cast(lt::alert *alert);

std::unique_ptr<std::vector<lt::torrent_status>>
lt_alert_state_update_status(lt::state_update_alert *alert);

// ==========================  Save Resume Data  ===========================
lt::save_resume_data_alert *lt_alert_save_resume_data_cast(lt::alert *alert);

std::unique_ptr<lt::torrent_handle>
lt_alert_save_resume_data_handle(lt::save_resume_data_alert *alert);

std::unique_ptr<lt::add_torrent_params>
lt_alert_save_resume_data_params(lt::save_resume_data_alert *alert);

// =======================  Save Resume Data Failed  =======================
lt::save_resume_data_failed_alert *
lt_alert_save_resume_data_failed_cast(lt::alert *alert);

std::unique_ptr<lt::torrent_handle> lt_alert_save_resume_data_failed_handle(
    lt::save_resume_data_failed_alert *alert);

int lt_alert_save_resume_data_failed_error(
    lt::save_resume_data_failed_alert *alert);
} // namespace libtorrent
