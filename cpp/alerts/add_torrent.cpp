#include "./add_torrent.h"
#include "cpp/error.h"
#include "libtorrent/add_torrent_params.hpp"

namespace ltrs {
    Error add_torrent_alert_get_error(lt::add_torrent_alert* alert) {
       return error_code_to_error(alert->error);
    }

    lt::add_torrent_params *add_torrent_alert_get_add_torrent_params(lt::add_torrent_alert* alert) {
        return &alert->params;
    }
}
