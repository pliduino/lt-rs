#include "./tracker_reply.h"

namespace ltrs {

int tracker_reply_alert_get_num_peers(lt::tracker_reply_alert* alert) {
    return alert->num_peers;
}

uint8_t tracker_reply_alert_get_version(lt::tracker_reply_alert* alert) {
    return static_cast<uint8_t>(alert->version);
}

} // namespace ltrs
