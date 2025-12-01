#pragma once

#include <libtorrent/add_torrent_params.hpp>
#include <libtorrent/alert_types.hpp>

#include "cpp/error.h"

namespace ltrs {
    Error add_torrent_alert_get_error(lt::add_torrent_alert* alert);
    lt::add_torrent_params *add_torrent_alert_get_add_torrent_params(lt::add_torrent_alert* alert);
}
