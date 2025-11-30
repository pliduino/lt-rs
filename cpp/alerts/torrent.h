#pragma once
#include "rust/cxx.h"
#include <libtorrent/torrent_handle.hpp>
#include <libtorrent/alert_types.hpp>

namespace ltrs {
    lt::torrent_handle *lt_torrent_alert_handle(lt::torrent_alert *a);
    rust::String lt_torrent_alert_message(const lt::torrent_alert *a);
    rust::Str lt_torrent_alert_torrent_name(const lt::torrent_alert *a);
}
