#pragma once
#include <libtorrent/torrent_status.hpp>
#include <libtorrent/alert_types.hpp>
#include <libtorrent/alert.hpp>

namespace ltrs {

    struct InfoHashCpp;
    InfoHashCpp torrent_removed_alert_get_info_hashes(lt::torrent_removed_alert* a);
}
