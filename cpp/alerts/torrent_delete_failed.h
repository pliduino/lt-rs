#pragma once
#include <libtorrent/alert_types.hpp>
#include "lt-rs/src/ffi/mod.rs.h"
#include "cpp/error.h"

namespace ltrs {
Error torrent_delete_failed_alert_get_error(lt::torrent_delete_failed_alert* alert);
InfoHashCpp torrent_delete_failed_alert_get_info_hashes(lt::torrent_delete_failed_alert* alert);

} // namespace ltrs
