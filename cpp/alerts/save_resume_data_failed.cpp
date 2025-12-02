#include "./save_resume_data_failed.h"
#include "cpp/error.h"

namespace ltrs {
    Error save_resume_data_failed_alert_get_error(lt::save_resume_data_failed_alert *alert) {
        return error_code_to_error(alert->error);
    }
}
