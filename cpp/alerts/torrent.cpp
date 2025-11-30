#include "./torrent.h"
#include "libtorrent/alert_types.hpp"

namespace ltrs {
    lt::torrent_handle *lt_torrent_alert_handle(lt::torrent_alert *a) {
        return &a->handle;
    }
    rust::String lt_torrent_alert_message(const lt::torrent_alert *a) {
        return a->message();
    }
    rust::Str lt_torrent_alert_torrent_name(const lt::torrent_alert *a) {
        return a->torrent_name();
    }
}
