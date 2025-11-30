#include "./performance.h"

namespace ltrs {
    unsigned char performance_alert_get_warning_code(lt::performance_alert* alert) {
        return alert->warning_code;
    }
}
