#include "./torrent_removed.h"

#include "cpp/lt.h"
#include "lt-rs/src/ffi/mod.rs.h"

namespace ltrs {
    InfoHashCpp torrent_removed_alert_get_info_hashes(lt::torrent_removed_alert* a) {
         return info_hash_t_to_info_hash_cpp(a->info_hashes);
    }
}
