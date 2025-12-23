#include "./peer_connect.h"

namespace ltrs {

uint8_t peer_connect_alert_get_direction(lt::peer_connect_alert* alert) {
    return static_cast<uint8_t>(alert->direction);
}

uint8_t peer_connect_alert_get_socket_type(lt::peer_connect_alert* alert) {
    return static_cast<uint8_t>(alert->socket_type);
}

} // namespace ltrs
