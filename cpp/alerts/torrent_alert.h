#pragma once
#include "rust/cxx.h"
#include <libtorrent/torrent_handle.hpp>
#include <libtorrent/alert_types.hpp>

namespace libtorrent {
    torrent_handle *lt_torrent_alert_handle(torrent_alert *a);
    rust::String lt_torrent_alert_message(const torrent_alert *a);
    rust::Str lt_torrent_alert_torrent_name(const torrent_alert *a);
}
