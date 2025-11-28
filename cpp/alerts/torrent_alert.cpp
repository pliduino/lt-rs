#include "./torrent_alert.h"
#include "libtorrent/alert_types.hpp"

namespace libtorrent {
    torrent_handle *lt_torrent_alert_handle(torrent_alert *a) {
        return &a->handle;
    }
    rust::String lt_torrent_alert_message(const torrent_alert *a) {
        return a->message();
    }
    rust::Str lt_torrent_alert_torrent_name(const torrent_alert *a) {
        return a->torrent_name();
    }
}
