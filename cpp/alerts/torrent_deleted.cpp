#include "./torrent_deleted.h"

namespace ltrs {

InfoHashCpp torrent_deleted_alert_get_info_hashes(lt::torrent_deleted_alert* alert) {
    return info_hash_t_to_info_hash_cpp(alert->info_hashes);
}

} // namespace ltrs
