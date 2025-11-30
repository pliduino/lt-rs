#include "./tracker_error.h"
#include "cpp/error.h"

namespace ltrs {
    rust::Str tracker_error_alert_get_failure_reason(lt::tracker_error_alert *a) {
        return a->failure_reason();
    }

    int tracker_error_alert_get_times_in_row(lt::tracker_error_alert *a) {
        return a->times_in_row;
    }

    Error tracker_error_alert_get_error(lt::tracker_error_alert *a) {
        return error_code_to_error(a->error);
    }

    uint8_t tracker_error_alert_get_op(lt::tracker_error_alert *a) {
        return static_cast<uint8_t>(a->op);
    }

    uint8_t tracker_error_alert_get_version(lt::tracker_error_alert *a) {
        return static_cast<uint8_t>(a->version);
    }
}
