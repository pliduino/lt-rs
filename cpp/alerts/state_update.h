#pragma once

#include <libtorrent/alert_types.hpp>

namespace ltrs {
    std::vector<lt::torrent_status> *state_update_alert_get_status(lt::state_update_alert *alert);
}
