#include "./storage_moved.h"

namespace ltrs {

rust::Str storage_moved_alert_get_storage_path(lt::storage_moved_alert* alert) {
    return alert->storage_path();
}

rust::Str storage_moved_alert_get_old_path(lt::storage_moved_alert* alert) {
    return alert->old_path();
}

} // namespace ltrs
