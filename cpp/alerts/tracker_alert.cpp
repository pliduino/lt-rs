#include "./tracker_alert.h"

namespace ltrs {
    rust::Str tracker_alert_get_tracker_url(lt::tracker_alert *a) {
        return a->tracker_url();
    }
}
