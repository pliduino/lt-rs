#include "./peer_disconnected.h"
#include "cpp/error.h"
#include <cstdint>

namespace ltrs {

uint8_t peer_disconnected_alert_get_socket_type(lt::peer_disconnected_alert* alert) {
    return static_cast<uint8_t>(alert->socket_type);
}

uint8_t peer_disconnected_alert_get_op(lt::peer_disconnected_alert* alert) {
    return static_cast<uint8_t>(alert->op);
}

Error peer_disconnected_alert_get_error(lt::peer_disconnected_alert* alert) {
    return error_code_to_error(alert->error);
}

uint16_t peer_disconnected_alert_get_reason(lt::peer_disconnected_alert* alert) {
    return static_cast<uint16_t>(alert->reason);
}

} // namespace ltrs
