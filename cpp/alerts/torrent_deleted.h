#pragma once
#include <libtorrent/alert_types.hpp>
#include "lt-rs/src/ffi/mod.rs.h"

namespace ltrs {
    InfoHashCpp torrent_deleted_alert_get_info_hashes(lt::torrent_deleted_alert* alert);

} // namespace ltrs
