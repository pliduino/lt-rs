#include "./lt.h"

#include "src/ffi.rs.h"

#include <libtorrent/read_resume_data.hpp>
#include <libtorrent/torrent_handle.hpp>
#include <libtorrent/write_resume_data.hpp>

#include <chrono>
#include <iostream>
#include <thread>
#include <vector>

namespace libtorrent {
InfoHashCpp info_hash_t_to_info_hash_cpp(const lt::info_hash_t &hash) {
  if (hash.has_v2()) {
    std::array<std::uint8_t, 32> v2_hash{};
    std::memcpy(v2_hash.data(), hash.v2.data(), v2_hash.size());
    return InfoHashCpp{
        2,
        v2_hash,
    };
  } else if (hash.has_v1()) {
    std::array<std::uint8_t, 32> v1_hash{};
    std::memcpy(v1_hash.data(), hash.v1.data(), 20);
    return InfoHashCpp{
        1,
        v1_hash,
    };
  }

  return InfoHashCpp{
      1,
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

std::unique_ptr<AlertListCpp> lt_session_pop_alerts(lt::session &ses) {
  std::vector<lt::alert *> alerts;
  ses.pop_alerts(&alerts);

  // for (auto alert : alerts)
  // {
  //     std::cout << alert->message() << std::endl;
  // }

  return std::make_unique<AlertListCpp>(alerts);
}

// ╔===========================================================================╗
// ║                            Add Torrent Params                             ║
// ╚===========================================================================╝

void lt_set_add_torrent_params_path(add_torrent_params &params,
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

std::unique_ptr<torrent_status>
lt_torrent_handle_status(const lt::torrent_handle &handle) {
  return std::make_unique<lt::torrent_status>(handle.status());
}

void lt_torrent_handle_save_resume_data(const lt::torrent_handle &handle,
                                        uint8_t flags) {
  handle.save_resume_data((lt::resume_data_flags_t)flags);
}

void lt_torrent_handle_read_piece(const lt::torrent_handle &handle, int piece) {
  // TODO
  //  handle.read_piece(piece);
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
