#include "./metadata_failed.h"

namespace ltrs {

Error metadata_failed_alert_get_error(lt::metadata_failed_alert* alert) {
    return error_code_to_error(alert->error);
}

} // namespace ltrs
