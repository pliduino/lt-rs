#include "./tracker_announce.h"

namespace ltrs {

uint8_t tracker_announce_alert_get_event(lt::tracker_announce_alert* alert) {
    return static_cast<uint8_t>(alert->event);
}

uint8_t tracker_announce_alert_get_version(lt::tracker_announce_alert* alert) {
    return static_cast<uint8_t>(alert->version);
}

} // namespace ltrs
