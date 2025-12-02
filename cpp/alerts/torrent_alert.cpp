#include "./torrent_alert.h"
#include "libtorrent/alert_types.hpp"
#include "libtorrent/fwd.hpp"
#include <memory>

namespace ltrs {
    std::unique_ptr<lt::torrent_handle> torrent_alert_handle(lt::torrent_alert *a) {
        return std::make_unique<lt::torrent_handle>(a->handle);
    }
    rust::String torrent_alert_message(const lt::torrent_alert *a) {
        return a->message();
    }
    rust::Str torrent_alert_torrent_name(const lt::torrent_alert *a) {
        return a->torrent_name();
    }
}
