#include "./file_rename_failed.h"

namespace ltrs {
    Error file_rename_failed_alert_get_error(lt::file_rename_failed_alert* alert) {
        return error_code_to_error(alert->error);
    }
}
