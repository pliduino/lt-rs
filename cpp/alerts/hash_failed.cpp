#include "./hash_failed.h"

namespace ltrs {

int32_t hash_failed_alert_get_piece_index(lt::hash_failed_alert* alert) {
    return static_cast<int32_t>(alert->piece_index);
}

} // namespace ltrs
