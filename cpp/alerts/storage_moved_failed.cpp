#include "./storage_moved_failed.h"

namespace ltrs {

rust::Str storage_moved_failed_alert_file_path(lt::storage_moved_failed_alert* alert) {
    return alert->file_path();
}

Error storage_moved_failed_alert_get_error(lt::storage_moved_failed_alert* alert) {
    return error_code_to_error(alert->error);
}

uint8_t storage_moved_failed_alert_get_op(lt::storage_moved_failed_alert* alert) {
    return static_cast<uint8_t>(alert->op);
}

} // namespace ltrs
