#include "./tracker_warning.h"

namespace ltrs {
    rust::Str tracker_warning_alert_get_warning_message(lt::tracker_warning_alert *a) {
        return a->warning_message();
    }

    uint8_t tracker_warning_alert_get_version(lt::tracker_warning_alert *a) {
        return static_cast<uint8_t>(a->version);
    }
}
