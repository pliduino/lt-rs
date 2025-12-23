#include "./file_error.h"

namespace ltrs {

rust::Str file_error_alert_get_filename(lt::file_error_alert* alert) {
    return alert->filename();
}

Error file_error_alert_get_error(lt::file_error_alert* alert) {
    return error_code_to_error(alert->error);
}

uint8_t file_error_alert_get_op(lt::file_error_alert* alert) {
    return static_cast<uint8_t>(alert->op);
}

} // namespace ltrs
