#pragma once

#include <libtorrent/torrent_status.hpp>
#include <libtorrent/alert_types.hpp>
#include <libtorrent/alert.hpp>

namespace libtorrent {
    class InfoHashCpp;
    InfoHashCpp torrent_removed_alert_get_info_hashes(torrent_removed_alert* a);

}
