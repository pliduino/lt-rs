#include "./torrent_delete_failed.h"
#include "cpp/error.h"

namespace ltrs {

Error torrent_delete_failed_alert_get_error(lt::torrent_delete_failed_alert* alert) {
    return error_code_to_error(alert->error);
}

InfoHashCpp torrent_delete_failed_alert_get_info_hashes(lt::torrent_delete_failed_alert* alert) {
    return info_hash_t_to_info_hash_cpp(alert->info_hashes);
}

} // namespace ltrs
