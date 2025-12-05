#include "./dht_reply.h"

namespace ltrs {

int dht_reply_alert_get_num_peers(lt::dht_reply_alert* alert) {
    return alert->num_peers;
}

} // namespace ltrs
