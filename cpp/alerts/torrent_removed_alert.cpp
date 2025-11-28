#include "./torrent_removed_alert.h"

#include "cpp/lt.h"
#include "lt-rs/src/ffi/mod.rs.h"

namespace libtorrent {
    InfoHashCpp torrent_removed_alert_get_info_hashes(torrent_removed_alert* a) {
         return info_hash_t_to_info_hash_cpp(a->info_hashes);
    }
}
