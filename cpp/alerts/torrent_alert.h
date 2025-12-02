#pragma once
#include "rust/cxx.h"
#include <libtorrent/torrent_handle.hpp>
#include <libtorrent/alert_types.hpp>
#include <memory>

namespace ltrs {
    std::unique_ptr<lt::torrent_handle> torrent_alert_handle(lt::torrent_alert *a);
    rust::String torrent_alert_message(const lt::torrent_alert *a);
    rust::Str torrent_alert_torrent_name(const lt::torrent_alert *a);
}
